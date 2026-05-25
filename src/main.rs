use gtk::prelude::*;
// Force Rust to use GTK objects instead of Rust embedded ones
use gtk::{Application, ApplicationWindow, Button, MenuBar, MenuItem, AboutDialog, Box as GtkBox};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.tutorial")
        .build();

    app.connect_activate(|app| {
        // --- Fenêtre principale ---
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Tutorial Rust")
            .default_width(400)
            .default_height(300)
            .build();

        // --- Bar Menu to host File and About
        let menu_bar = MenuBar::new();

        // --- File Menu _ good to have one at first
        let file_menu_item = MenuItem::with_label("File");
        let file_menu = gtk::Menu::new();
        file_menu_item.set_submenu(Some(&file_menu));
        menu_bar.append(&file_menu_item);

        // --- About Menu with Version Item
        let help_menu_item = MenuItem::with_label("Help");
        let help_menu = gtk::Menu::new();
        help_menu_item.set_submenu(Some(&help_menu));
        menu_bar.append(&help_menu_item);

        // --- Item "Version" ---
        let version_menu_item = MenuItem::with_label("Version");

        // --- About is based on a dialog box, have to declare it before activation
        let about_dialog = AboutDialog::builder()
            .program_name(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .authors(vec!["Aen Blyd"])
            .comments("Rust with GTK simple app")
            .transient_for(&window)
            .build();

        version_menu_item.connect_activate(move |_| {
            about_dialog.show(); // ✅ Show it, not in locked mode
            about_dialog.run();  // ✅ Lock it until closure
            about_dialog.hide(); // ✅ To avoid destroy (obsolete), just hide it
        });
        help_menu.append(&version_menu_item);

        // --- 1 Bouton to test print in console
        let button = Button::with_label("Click here!");
        button.connect_clicked(|_| {
            println!("The Button has been clicked !");
        });

        // --- To host the button and the menu : a vertical gtk bar
        let vbox = GtkBox::new(gtk::Orientation::Vertical, 0); // ✅ `gtk::Box` with spacing
        vbox.pack_start(&menu_bar, false, false, 0);
        vbox.pack_start(&button, true, true, 0);

        // --- add the vertical box to the window
        window.set_child(Some(&vbox));
        window.show_all();
    });

    app.run()
}