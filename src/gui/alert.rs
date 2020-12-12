use super::support;

use imgui::*;
use memflow_win32::error::Error;

pub fn show_error(title: &str, text: &str, error: Error) {
    let error_str = error.to_str_pair();
    if let Some(error_details) = error_str.1 {
        show_alert(
            title,
            &format!("{}\n\n{}: {}", text, error_str.0, error_details),
        )
    } else {
        show_alert(title, &format!("{}\n\n{}", text, error_str.0))
    }
}

pub fn show_alert(title: &str, text: &str) {
    support::show_window(title, 400.0, 265.0, |run, ui| {
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
