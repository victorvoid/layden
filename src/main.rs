extern crate gtk;

use gtk::prelude::*;
use gtk::{
    AboutDialog, CheckMenuItem, Label, Menu, MenuBar, MenuItem, Window,
    WindowPosition, WindowType
};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("Layden");
    window.set_position(WindowPosition::Center);
    window.set_size_request(900, 600);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu = Menu::new();
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let new_file = MenuItem::new_with_label("New");
    let quit = MenuItem::new_with_label("Quit");
    let check_item = CheckMenuItem::new_with_label("Click me!");

    menu.append(&new_file);
    menu.append(&check_item);
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    quit.connect_activate(|_| {
        gtk::main_quit();
    });

    let label = Label::new(Some("Layden"));

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&label, true, true, 0);
    window.add(&v_box);
    window.show_all();

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["Victor Igor"]);
        p.set_website_label(Some("Layden"));
        p.set_website(Some("http://github.com/victorvoid/layden"));
        p.set_authors(&["Victor Igor"]);
        p.set_title("About!");
        p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    });
    check_item.connect_toggled(|w| {
        w.set_label(if w.get_active() {
            "Checked"
        } else {
            "Unchecked"
        });
    });
    gtk::main();
}
