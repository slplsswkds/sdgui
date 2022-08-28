mod time;

use std::cell::RefCell;


use fltk::{
    app::App,
    window::Window,
    text::TextDisplay,
    text::TextBuffer,
    prelude::*,
    button::*,
    enums::Color,
    *,
};

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

    let mut disp_time_scheduled = TextDisplay::default().with_size(114, 50).with_pos(143, 200);
    let buf_time_scheduled = RefCell::from(TextBuffer::default());
    disp_time_scheduled.set_buffer(buf_time_scheduled.clone().into_inner());
    disp_time_scheduled.set_text_size(36);
    disp_time_scheduled.set_text_color(Color::from_hex(0x226611));

    window.end();
    window.show();

    let mut time_scheduled = time::Time24::new();
    time_scheduled.now();
    buf_time_scheduled.clone().into_inner().set_text( &time_scheduled.to_str() );

    but_schedule.set_callback({
        let time_sch = time_scheduled.clone();
        move |_| {
            time_sch.shutdown_schedule();
            app.quit();
        }
    });

    but_cancel.set_callback(
        move |_| {
            app.quit();
        }
    );

    but_hours_up.set_callback({
        let mut disp_buf = buf_time_scheduled.clone().into_inner();
        let mut time_sch = time_scheduled.clone();
        move |_| {
            time_sch.add_hours(1);
            disp_buf.set_text(&time_sch.to_str());
        }
    });

    but_hours_down.set_callback({
        let mut disp_buf = buf_time_scheduled.clone().into_inner();
        let mut time_sch = time_scheduled.clone();
        move |_| {
            time_sch.subtract_hours(1);
            disp_buf.set_text(&time_sch.to_str());
        }
    });

    but_minutes_up.set_callback({
        let mut disp_buf = buf_time_scheduled.clone().into_inner();
        let mut time_sch = time_scheduled.clone();
        move |_| {
            time_sch.add_minutes(1);
            disp_buf.set_text(&time_sch.to_str());
        }
    });

    but_minutes_down.set_callback({
        let mut disp_buf = buf_time_scheduled.clone().into_inner();
        let mut time_sch = time_scheduled.clone();
        move |_| {
            time_sch.subtract_minutes(1);
            disp_buf.set_text(&time_sch.to_str());
        }
    });

    app.run().unwrap();
}
