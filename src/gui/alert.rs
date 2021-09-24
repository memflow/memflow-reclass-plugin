use super::support;

use imgui::*;
use memflow::prelude::v1::Error;

pub fn show_error(title: &str, text: &str, error: Error) {
    show_alert(title, &format!("{}:\n{}", text, error.as_str()))
}

pub fn show_alert(title: &str, text: &str) {
    support::show_window(title, 400.0, 160.0, |run, ui| {
        Window::new(im_str!("Warning"))
            .position([10.0, 10.0], Condition::Always)
            .size([375.0, 1000.0], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .scroll_bar(false)
            .save_settings(false)
            .focus_on_appearing(false)
            .movable(false)
            .build(ui, || {
                ui.text(title);
                ui.separator();

                ui.dummy([0.0, 16.0]);

                ui.text(text);

                ui.dummy([0.0, 16.0]);

                if ui.button(im_str!("Ok"), [64.0, 26.0]) {
                    // close window
                    *run = false;
                }
            });
    })
}
