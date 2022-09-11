mod scheduler;
mod time;

use scheduler::shutdown_schedule;

use std::cell::RefCell;

use fltk::{
    app::App, button::*, enums::Color, prelude::*, text::TextBuffer, text::TextDisplay,
    window::Window, *,
};

//use  fltk::enums::FrameType;

fn main() {
    let app = App::default();
    app::set_font_size(18);

    let mut window = Window::new(400, 400, 400, 300, "sdgui");

    let mut but_schedule = Button::new(290, 240, 100, 50, "Schedule");
    let mut but_cancel = Button::new(10, 240, 100, 50, "Cancel");
    let mut but_hours_down = Button::new(10, 10, 40, 40, "H-");
    let mut but_hours_up = Button::new(350, 10, 40, 40, "H+");
    let mut but_minutes_down = Button::new(10, 60, 40, 40, "M-");
    let mut but_minutes_up = Button::new(350, 60, 40, 40, "M+");

    let buf_time_scheduled = RefCell::from(TextBuffer::default());
    let mut disp_time_scheduled = TextDisplay::default()
        .with_size(114, 50)
        .with_pos(276, 180)
        .with_label("Scheduled");
    disp_time_scheduled.set_buffer(buf_time_scheduled.clone().into_inner());
    disp_time_scheduled.set_text_size(36);
    disp_time_scheduled.set_text_color(Color::from_hex(0x226611));

    let buf_time_remained = RefCell::from(TextBuffer::default());
    let mut disp_time_remained = TextDisplay::default()
        .with_size(114, 50)
        .with_pos(143, 180)
        .with_label("Remained");
    disp_time_remained.set_buffer(buf_time_remained.clone().into_inner());
    disp_time_remained.set_text_size(36);
    disp_time_remained.set_text_color(Color::from_hex(0x226611));

    let buf_time_current = RefCell::from(TextBuffer::default());
    let mut disp_time_current = TextDisplay::default()
        .with_size(114, 50)
        .with_pos(10, 180)
        .with_label("Current");
    disp_time_current.set_buffer(buf_time_current.clone().into_inner());
    disp_time_current.set_text_size(36);
    disp_time_current.set_text_color(Color::from_hex(0x226611));

    window.end();
    window.show();

    let mut time_scheduled = time::Time24::new();
    time_scheduled.now();
    buf_time_scheduled
        .clone()
        .into_inner()
        .set_text(&time_scheduled.to_str());

    let time_remained = time::Time24::new();
    buf_time_remained
        .clone()
        .into_inner()
        .set_text(&time_remained.to_str());

    let mut time_current = time::Time24::new();
    time_current.now();
    buf_time_current
        .clone()
        .into_inner()
        .set_text(&time_current.to_str());

    but_schedule.set_callback({
        let time_remained = time_remained.clone();
        move |_| {
            shutdown_schedule(&time_remained);
            app.quit();
        }
    });

    but_cancel.set_callback(move |_| {
        app.quit();
    });

    but_hours_up.set_callback({
        let mut buf_time_scheduled = buf_time_scheduled.clone().into_inner();
        let mut time_scheduled = time_scheduled.clone();
        let mut buf_time_remained = buf_time_remained.clone().into_inner();
        let mut time_remained = time_remained.clone();
        move |_| {
            time_scheduled.add_hours(1);
            time_remained.add_hours(1);
            buf_time_scheduled.set_text(&time_scheduled.to_str());
            buf_time_remained.set_text(&time_remained.to_str());
        }
    });

    but_hours_down.set_callback({
        let mut buf_time_scheduled = buf_time_scheduled.clone().into_inner();
        let mut time_scheduled = time_scheduled.clone();
        let mut buf_time_remained = buf_time_remained.clone().into_inner();
        let mut time_remained = time_remained.clone();
        move |_| {
            time_scheduled.subtract_hours(1);
            time_remained.subtract_hours(1);
            buf_time_scheduled.set_text(&time_scheduled.to_str());
            buf_time_remained.set_text(&time_remained.to_str());
        }
    });

    but_minutes_up.set_callback({
        let mut buf_time_scheduled = buf_time_scheduled.clone().into_inner();
        let mut time_scheduled = time_scheduled.clone();
        let mut buf_time_remained = buf_time_remained.clone().into_inner();
        let mut time_remained = time_remained.clone();
        move |_| {
            time_scheduled.add_minutes(1);
            time_remained.add_minutes(1);
            buf_time_scheduled.set_text(&time_scheduled.to_str());
            buf_time_remained.set_text(&time_remained.to_str());
        }
    });

    but_minutes_down.set_callback({
        let mut buf_time_scheduled = buf_time_scheduled.clone().into_inner();
        let mut time_scheduled = time_scheduled.clone();
        let mut buf_time_remained = buf_time_remained.clone().into_inner();
        let mut time_remained = time_remained.clone();
        move |_| {
            time_scheduled.subtract_minutes(1);
            time_remained.subtract_minutes(1);
            buf_time_scheduled.set_text(&time_scheduled.to_str());
            buf_time_remained.set_text(&time_remained.to_str());
        }
    });

    app.run().unwrap();
}
