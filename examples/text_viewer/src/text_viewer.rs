//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;

use std::io::{BufferedReader, File};
use std::num::FromPrimitive;

use rgtk::*;
use rgtk::gtk::signals::{Clicked, DeleteEvent, MotionNotifyEvent};

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    window.set_title("Text File Viewer");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 300);

    let mut toolbar = gtk::Toolbar::new().unwrap();

    let open_icon = gtk::Image::new_from_icon_name("document-open", gtk::IconSize::SmallToolbar).unwrap();
    let text_view = gtk::TextView::new().unwrap();

    let mut label = gtk::Label::new("hi").unwrap();

    let mut open_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Open")).unwrap();
    open_button.set_is_important(true);
    Connect::connect(&open_button, Clicked::new(|| {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new("Open File", None, gtk::FileChooserAction::Open).unwrap();
        let response: Option<gtk::ResponseType> = FromPrimitive::from_i32(file_chooser.run());

        match response {
            Some(gtk::ResponseType::Accept) => {
                let filename = file_chooser.get_filename().unwrap();
                let file = File::open(&Path::new(filename));

                let mut reader = BufferedReader::new(file);
                let contents = reader.read_to_string().unwrap();

                text_view.get_buffer().unwrap().set_text(contents);

            },
            _ => {}
        };

        file_chooser.destroy();
    }));

    text_view.connect( MotionNotifyEvent::new(| signal | -> bool { // :
        let x :f64 = unsafe { (*signal).x };
        let y :f64 = unsafe { (*signal).y };
        &label.set_text(format!("x: {}, y: {}", x, y).as_slice());
        true
    }));

    toolbar.add(&open_button);

    let mut scroll = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let mut vbox = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    vbox.pack_start(&toolbar, false, true, 0);
    vbox.pack_start(&scroll, true, true, 0);
    vbox.pack_start(&label, false, false, 0);

    window.add(&vbox);

    Connect::connect(&window, DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    let mut buffer = text_view.get_buffer().unwrap();
    text_tag_example(&mut buffer);

    window.show_all();
    gtk::main();
}

pub fn text_tag_example(buffer : &mut gtk::TextBuffer) {
    let tag_green = buffer.create_tag("green", "foreground", "green");
    let tag_shrink = buffer.create_tag_double("shrink", "scale", 0.7);
    let tag_bold = buffer.create_tag_int("embolden", "weight", 900);
    let tag_strike = buffer.create_tag_int("strike", "strikethrough", 1);
    let mut iter = gtk::TextIter::new().unwrap();
    buffer.get_iter_at_offset(&iter, 0);
    insert_with_tag(buffer, &mut iter, "Try ", &tag_bold);
    insert_with_tag(buffer, &mut iter, "opening ", &tag_green);
    insert_with_tag(buffer, &mut iter, "a ", &tag_shrink);
    insert_with_tag(buffer, &mut iter, "file", &tag_strike);

}

pub fn insert_with_tag(buffer :&mut gtk::TextBuffer, iter :&mut gtk::TextIter, text :&str, tag :&gtk::TextTag) {
    let start = gtk::TextIter::new().unwrap(); //iter.copy().unwrap();
    let start_offset = iter.get_offset();
    buffer.insert(iter, text, -1);
    buffer.get_iter_at_offset(&start, start_offset);
    buffer.apply_tag(tag, &start, iter);

}