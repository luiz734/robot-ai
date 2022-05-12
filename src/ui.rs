use crate::robot::Nannou;
use nannou::{color::BLACK, prelude::Rect};

pub struct UI {
    background: Rect,
    display_blueprint: String,
    display_text: String,
}

impl UI {
    pub fn new(window_rect: &Rect) -> UI {
        UI {
            background: Rect::from_w_h(200.0, window_rect.h()).top_left_of(window_rect.to_owned()),
            display_blueprint: r#"
    sensor -45º   {}
    sensor -35º   {}
    sensor - 0º   {}
    sensor +35º   {}
    sensor +45º   {}
    rotation      {}"#
                .to_owned(),
            display_text: "PLACEHOLDER".to_owned(),
        }
    }
    pub fn update_display_text(&mut self, values: &Vec<f32>) {
        self.display_text = self.display_blueprint.to_owned();
        for v in values {
            self.display_text = self.display_text.replacen("{}", v.to_string().as_str(), 1);
        }
    }
}
impl Nannou for UI {
    fn draw(&self, draw: &nannou::Draw) {
        // background
        draw.rect()
            .xy(self.background.xy())
            .wh(self.background.wh())
            .color(BLACK);
        // text
        draw.text(&self.display_text)
            .xy(self.background.xy())
            .wh(self.background.wh())
            .font_size(16)
            .no_line_wrap()
            .left_justify()
            .align_text_top()
            .line_spacing(3.0);
    }

    fn update(&mut self) {
        //
    }
}
