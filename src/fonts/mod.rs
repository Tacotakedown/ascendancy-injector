
use std::path::PathBuf;
use std::path::Path;
use runas::Command as RunAsCommand;
use font_kit::source::SystemSource;



pub struct Fonts {
    font_path:PathBuf
}

impl Fonts {
    pub fn new() -> Self {
        Self{
            font_path :PathBuf::new()
        }
    }


}


pub fn font_exists(font_name: &str) -> bool {
    let mut font_exists:bool = false;

    let source  = SystemSource::new();
    let font_families = source.all_families().unwrap();

    for family in font_families {
      if family == font_name {
          println!("family:{} is equal to:{}", family, font_name);
          return true}
    };

    false

}

pub fn install_font() {
    println!("installing emoji font");

    let link = "https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji_WindowsCompatible.ttf";
    let temp_file_path = "C:\\tmp\\NotoColorEmoji_WindowsCompatible.ttf";
    let destination_path = "C:\\Windows\\Fonts\\NotoColorEmoji_WindowsCompatible.ttf";

    let response = reqwest::blocking::get(link);
    if let Ok(mut font_file) = response {
        let mut file = std::fs::File::create(temp_file_path).unwrap();
        std::io::copy(&mut font_file, &mut file).unwrap();

        println!("Font download finished, copying");
        move_font_with_admin_permissions(temp_file_path);
        println!("Complete");
    }else {
        println!("failed to download font file");
    }
}


fn move_font_with_admin_permissions(font_path: &str) {
    let source_path = Path::new(font_path);
    let destination_path = Path::new("C:\\Windows\\Fonts");

    let command = format!(
        "xcopy {} {} /Y",
        source_path.to_str().unwrap(),
        destination_path.to_str().unwrap()
    );

    println!("executing command: {}", &command);

    match RunAsCommand::new("cmd").arg("/C").arg(&command).status() {
        Ok(status) if status.success() => {
            println!("Font installed successfully!");
        }
        Ok(status) => {
            println!("Failed to install font.");
            println!("Error code: {}", status.code().unwrap_or_default());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}