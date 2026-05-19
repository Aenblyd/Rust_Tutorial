use gtk::prelude::*;
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

        // --- Barre de menu ---
        let menu_bar = MenuBar::new();

        // --- Menu "Fichier" ---
        let file_menu_item = MenuItem::with_label("File");
        let file_menu = gtk::Menu::new();
        file_menu_item.set_submenu(Some(&file_menu));
        menu_bar.append(&file_menu_item);

        // --- Menu "Aide" avec item "Version" ---
        let help_menu_item = MenuItem::with_label("Help");
        let help_menu = gtk::Menu::new();
        help_menu_item.set_submenu(Some(&help_menu));
        menu_bar.append(&help_menu_item);

        // --- Item "Version" ---
        let version_menu_item = MenuItem::with_label("Version");

        // --- Déclare la boîte de dialogue en dehors du connect_activate ---
        let about_dialog = AboutDialog::builder()
            .program_name(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .authors(vec!["Aen Blyd"])
            .comments("Rust with GTK simple app")
            .transient_for(&window)
            .build();

        version_menu_item.connect_activate(move |_| {
            about_dialog.show(); // ✅ Affiche la boîte (non bloquant)
            about_dialog.run();  // ✅ Bloque jusqu'à la fermeture
            about_dialog.hide(); // ✅ Masque la boîte (optionnel)
        });
        help_menu.append(&version_menu_item);

        // --- Bouton principal ---
        let button = Button::with_label("Click here!");
        button.connect_clicked(|_| {
            println!("The Button has been clicked !");
        });

        // --- Conteneur vertical ---
        let vbox = GtkBox::new(gtk::Orientation::Vertical, 0); // ✅ `gtk::Box` avec espacement
        vbox.pack_start(&menu_bar, false, false, 0);
        vbox.pack_start(&button, true, true, 0);

        // --- Ajoute le conteneur à la fenêtre ---
        window.set_child(Some(&vbox));
        window.show_all();
    });

    app.run()
}