use std::ffi::{CStr, CString};
use std::ptr::null_mut;
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use winapi::um::winnt::{MEM_COMMIT, PAGE_READWRITE, PROCESS_ALL_ACCESS};

pub fn find_process_id_by_name(exe_name: &str) -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return None;
        }
        println!("Snapshot: {:?}", &snapshot);

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 1 {
            loop {
                let exe_name_cstr = CString::new(exe_name).unwrap();
                let process_name = CStr::from_ptr(entry.szExeFile.as_ptr());
                println!("Exe name cstr:{:?} process name cstr: {:?}", &exe_name_cstr, &process_name);
                if process_name == exe_name_cstr.as_c_str() {
                    CloseHandle(snapshot);
                    return Some(entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
    }
    None
}

pub fn inject_dll(process_id: u32, dll_path: &str) -> bool {
    unsafe {
        let dll_path_cstr = CString::new(dll_path).unwrap();
        let h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
        println!("process handle: {:?}", &h_process);

        if h_process.is_null() {
            return false;
        }

        let alloc_memory = VirtualAllocEx(h_process, null_mut(), dll_path_cstr.to_bytes_with_nul().len(), MEM_COMMIT, PAGE_READWRITE);
        if alloc_memory.is_null() {
            CloseHandle(h_process);
            return false;
        }

        if WriteProcessMemory(h_process, alloc_memory, dll_path_cstr.as_ptr() as *const _, dll_path_cstr.to_bytes_with_nul().len(), null_mut()) == 0 {
            CloseHandle(h_process);
            return false;
        }

        let h_kernal32 = GetModuleHandleA(CString::new("kernel32.dll").unwrap().as_ptr());
        let load_lib_addr = GetProcAddress(h_kernal32, CString::new("LoadLibraryA").unwrap().as_ptr());

        if CreateRemoteThread(h_process, null_mut(), 0, Some(std::mem::transmute(load_lib_addr)), alloc_memory, 0, null_mut()).is_null() {
            CloseHandle(h_process);
            return false;
        }
        CloseHandle(h_process);
        true
    }
}