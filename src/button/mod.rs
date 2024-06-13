use eframe::egui::{self, Align2, Sense, PointerButton, Color32, TextStyle, Ui, Pos2, Id, FontId};


pub struct RunButton {
    normal_bg_color: Color32,
    hovered_bg_color: Color32,
    pressed_bg_color: Color32,
    text_color: Color32,
}

impl Default for RunButton {
    fn default() -> Self {
        Self {
            normal_bg_color: Color32::from_rgb(136, 109, 74),
            hovered_bg_color: Color32::from_rgb(120, 120, 120),
            pressed_bg_color: Color32::from_rgb(80, 80, 80),
            text_color: Color32::WHITE,
        }
    }
}

impl RunButton {
    pub fn draw_button(&self, ui: &mut Ui, label: &str, id: Id) -> bool {
        let button_width = 150.0;
        let button_height = 60.0;

        let mut pos: Pos2 = ui.next_widget_position();
        pos[0] = pos[0] - button_width / 2.0;
        pos[1] = pos[1] - button_height / 2.0;
        let rect = egui::Rect::from_min_max(pos, pos + egui::Vec2::new(button_width, button_height));

        let painter = ui.painter();

        let mut bg_color = Color32::from_rgb(0, 0, 0);

        match ui.input(|i| i.pointer.latest_pos()) {
            Some(pos) => {
                if rect.contains(pos) && ui.input(|i| i.pointer.button_down(PointerButton::Primary)) {
                    bg_color = self.pressed_bg_color
                } else if rect.contains(pos) {
                    bg_color = self.hovered_bg_color
                } else {
                    bg_color = self.normal_bg_color
                }
            }
            None => {
                bg_color = self.normal_bg_color
            }
        }


        painter.rect_filled(rect, 5.0, bg_color);

        let text_style = TextStyle::Button;
        let id = FontId::default();

        // let label_size = ui.fonts().layout_no_wrap(text_style, label);

        let label_pos = rect.center();
        painter.text(label_pos, Align2::CENTER_CENTER, label, id, self.text_color);

        let response = ui.allocate_response(rect.size(), Sense::click());
        response.clicked()
    }
}