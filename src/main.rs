#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod inject;
mod button;
mod fetch_dll;
mod fonts;

use std::default::Default;
use eframe::egui::{self, CentralPanel, Context, ViewportCommand, Label, TextEdit, Button, Id, RichText};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc,Mutex};


use eframe::Frame;
use crate::button::RunButton;
use crate::fonts::{font_exists, install_font};


#[derive(Eq, PartialEq)]
enum LoadingState {
    UNINJECTED,
    INJECTING,
    INJECTED,
}

struct App {
    exe_name: String,
    dll_path: String,
    sender: Sender<(String, String)>,
    state: LoadingState,

}

impl App {
    fn new(sender: Sender<(String, String)>) -> Self {
        Self {
            exe_name: "eldenring.exe".to_string(),
            dll_path: String::new(),
            sender,
            state: LoadingState::UNINJECTED,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let mut run_button = RunButton::default();
        custom_window_frame(ctx, "Ascendancy", |ui| {
            ui.centered_and_justified(|ui| {
                let id = Id::new("buttonId");

                
                match self.state {
                    LoadingState::UNINJECTED => {
                        if run_button.draw_button(ui, "Run", id) {
                            let exe_name = self.exe_name.clone();
                            let dll_path = self.dll_path.clone();
                            // self.sender.send((exe_name, dll_path)).unwrap();
                            // self.state = LoadingState::INJECTING;
                            let font_name = "EmojiFontThatWeDontHave";
                            if font_exists(font_name) {
                                println!("The font '{}' is installed.", font_name);
                            }else {
                                println!("The font '{}' is not installed.", font_name);
                               std::thread::spawn(move|| {
                                   install_font();
                               });
                            }
                        }
                    }
                    LoadingState::INJECTING => {
                        ui.add(Label::new("Injecting"));
                        // loading icon
                    }
                    LoadingState::INJECTED => {
                        ui.add(Label::new("Injected"));
                        // Complete
                    }
                }
            });


            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(Label::new(RichText::new("Current DLL Version: 0.0.1")));
                ui.add(Label::new(RichText::new("Fonts Installed: ✅")));
                ui.add(Label::new(RichText::new("Injector up to date: ✅")));
            });
        })
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

fn main() {
    let (sender, reciever): (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_decorations(false).with_resizable(false).with_inner_size([600.0, 300.0]).with_min_inner_size([600.0, 300.0]).with_transparent(true),

        ..Default::default()
    };

    std::thread::spawn(move || {
        while let Ok((exe_name, dll_path)) = reciever.recv() {
            println!("handle inject: path: {} exe: {}", &dll_path, &exe_name);
            if let Some(process_id) = inject::find_process_id_by_name(&exe_name) {
                println!("process id: {}", &process_id);
                if inject::inject_dll(process_id, &dll_path) {
                    println!("DLL injected successfully into process ID {}", process_id);
                } else {
                    println!("DLL injection failed.");
                }
            } else {
                println!("Process with executable name '{}' not found.", exe_name);
            }
        }
    });

    eframe::run_native("Dll Injector", options, Box::new(|_cc| Box::new(App::new(sender)))).unwrap();
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: Color32::BLACK,
        rounding: 10.0.into(),
        stroke: Stroke::NONE,
        outer_margin: 0.5.into(),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let painter = ui.painter();
        let gradient_rect = app_rect;

        let gradient_colors = vec![
            (0.5, Color32::from_rgb(0, 0, 0)),
            (1.0, Color32::from_rgb(136, 109, 74)),
        ];

        paint_abstract_wierd_thing_that_i_accidently_made_but_looks_pretty_cool_ngl(painter, gradient_rect, gradient_colors);


        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
            .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}


fn paint_gradient(painter: &egui::Painter, rect: egui::Rect, colors: Vec<(f32, egui::Color32)>) {
    let mesh = {
        use egui::epaint::*;
        let mut mesh = Mesh::with_texture(egui::TextureId::default());

        let n = 64;
        for i in 0..=n {
            let t = i as f32 / n as f32;
            let color = lerp_color(&colors, t);
            let x = lerp(rect.min.x..=rect.max.x, t);

            mesh.colored_vertex(egui::pos2(x, rect.min.y), color);
            mesh.colored_vertex(egui::pos2(x, rect.max.y), color);

            if i > 0 {
                let idx = mesh.vertices.len() as u32;
                mesh.add_triangle(idx - 4, idx - 3, idx - 2);
                mesh.add_triangle(idx - 2, idx - 3, idx - 1);
            }
        }

        mesh
    };
    painter.add(egui::Shape::mesh(mesh));
}

fn paint_abstract_wierd_thing_that_i_accidently_made_but_looks_pretty_cool_ngl(painter: &egui::Painter, rect: egui::Rect, colors: Vec<(f32, egui::Color32)>) {
    let corner_radius = 90.0;
    let mesh = {
        use egui::epaint::*;
        let mut mesh = Mesh::with_texture(egui::TextureId::default());
        let n = 300;
        for i in 0..=n {
            let t = i as f32 / n as f32;
            let color = lerp_color(&colors, t);
            let x = lerp(rect.min.x..=rect.max.x, t);
            let y = lerp(rect.min.y..=rect.max.y, t);

            let mut x_adjusted = x;
            let mut y_adjusted = y;

            if x < rect.min.x + corner_radius {
                x_adjusted = rect.min.x + corner_radius;
            } else if x > rect.max.x - corner_radius {
                x_adjusted = rect.max.x - corner_radius;
            }

            if y < rect.min.y + corner_radius {
                y_adjusted = rect.min.y + corner_radius;
            } else if y > rect.max.y - corner_radius {
                y_adjusted = rect.max.y - corner_radius;
            }

            mesh.colored_vertex(egui::pos2(x_adjusted, rect.min.y), color);
            mesh.colored_vertex(egui::pos2(rect.max.x, y_adjusted), color);

            if i > 0 {
                let idx = mesh.vertices.len() as u32;
                mesh.add_triangle(idx - 4, idx - 3, idx - 2);
                mesh.add_triangle(idx - 2, idx - 3, idx - 1);
            }
        }
        mesh
    };
    painter.add(egui::Shape::mesh(mesh));
}

fn lerp_color(colors: &Vec<(f32, egui::Color32)>, t: f32) -> egui::Color32 {
    if colors.is_empty() {
        return egui::Color32::TRANSPARENT;
    }
    if t <= colors[0].0 {
        return colors[0].1;
    }
    if t >= colors.last().unwrap().0 {
        return colors.last().unwrap().1;
    }
    for i in 0..colors.len() - 1 {
        let (t0, c0) = colors[i];
        let (t1, c1) = colors[i + 1];
        if t0 <= t && t <= t1 {
            let s = (t - t0) / (t1 - t0);
            return egui::Color32::from_rgba_unmultiplied(
                lerp(c0.r() as f32..=c1.r() as f32, s) as u8,
                lerp(c0.g() as f32..=c1.g() as f32, s) as u8,
                lerp(c0.b() as f32..=c1.b() as f32, s) as u8,
                lerp(c0.a() as f32..=c1.a() as f32, s) as u8,
            );
        }
    }
    unreachable!()
}

fn lerp(range: std::ops::RangeInclusive<f32>, t: f32) -> f32 {
    range.start() + t * (range.end() - range.start())
}


fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("title_bar"),
        Sense::click_and_drag(),
    );

    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );


    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.dragged_by(PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_minimize(ui);
        });
    });
}

fn close_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 20.0;

    let close_response = ui.add(Button::new(RichText::new("❌").size(button_height)));
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }


    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)));
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}