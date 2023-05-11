#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{prelude::*, text::TextBuffer, *};
mod ui;

fn main() {
    let app = app::App::default();
    let mut ui = ui::UserInterface::make_window();

    let mut text_buf = TextBuffer::default();
    ui.text.set_buffer(text_buf.clone());

    let mut counter = 0;
    text_buf.set_text(&counter.to_string());

    ui.button.set_callback(move |_| {
        println!("Works!");
        counter += 1;
        text_buf.set_text(&counter.to_string());
    });
    app.run().unwrap();
}
