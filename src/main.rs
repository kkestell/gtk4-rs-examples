use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::{Box, ListBox, Orientation, SelectionMode};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("First App")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.present();
    });

    application.run();
}

// use gtk::{glib, prelude::*};
// 
// fn main() -> glib::ExitCode {
//     let application = gtk::Application::builder()
//         .application_id("com.github.gtk-rs.examples.basic")
//         .build();
//     application.connect_activate(build_ui);
//     application.run()
// }
// 
// fn build_ui(application: &gtk::Application) {
//     let window = gtk::ApplicationWindow::new(application);
// 
//     window.set_title(Some("Hello gtk-rs!"));
//     window.set_default_size(350, 70);
// 
//     let button = gtk::Button::with_label("Click me!");
// 
//     window.set_child(Some(&button));
// 
//     window.present();
// }