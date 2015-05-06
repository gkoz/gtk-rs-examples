//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate gtk;

use gtk::prelude::*;
use gtk::window;
use gtk::button::Button;

fn main() {
    gtk::init();

    let window = window::Window::new(gtk::WindowType::TopLevel);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let button = Button::new_with_label("Click me!");

    button.connect_clicked(|_| println!("Clicked!"));

    window.add(&button);

    window.show_all();

    let children = window.get_children();
    let btn: Result<Button, _> = children[0].clone().downcast();
    if let Ok(btn) = btn {
        btn.clicked();
    }

    gtk::main();
}
