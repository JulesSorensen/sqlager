extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Gestionnaire de bases de données");
    window.set_default_size(400, 300);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);

    let menu_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vbox.pack_start(&menu_box, false, false, 0);

    let new_db_button = Button::new();
    new_db_button.set_label("+");
    menu_box.pack_start(&new_db_button, false, false, 0);

    let db_list_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&db_list_box, true, true, 0);

    let window_clone = window.clone();
    new_db_button.connect_clicked(move |_| {
        let dialog = gtk::Dialog::with_buttons(
            Some("Ajouter une base de données"),
            Some(&window_clone),
            gtk::DialogFlags::MODAL,
            &[
                ("Ajouter", gtk::ResponseType::Ok.into()),
                ("Annuler", gtk::ResponseType::Cancel.into()),
            ],
        );

        let content_area = dialog.content_area();
        if let Some(content_area) = content_area.downcast_ref::<gtk::Box>() {
            // your code here
        } else {
            println!("Failed to downcast content area to Box");
        }

        let form_grid = gtk::Grid::new();
        form_grid.set_column_spacing(10);
        form_grid.set_row_spacing(10);
        content_area.add(&form_grid);

        let name_label = Label::new(Some("Nom :"));
        let name_entry = Entry::new();
        form_grid.attach(&name_label, 0, 0, 1, 1);
        form_grid.attach_next_to(
            &name_entry,
            Some(&name_label),
            gtk::PositionType::Right,
            1,
            1,
        );

        let host_label = Label::new(Some("Hôte :"));
        let host_entry = Entry::new();
        form_grid.attach(&host_label, 0, 1, 1, 1);
        form_grid.attach_next_to(
            &host_entry,
            Some(&host_label),
            gtk::PositionType::Right,
            1,
            1,
        );

        let username_label = Label::new(Some("Nom d'utilisateur :"));
        let username_entry = Entry::new();
        form_grid.attach(&username_label, 0, 2, 1, 1);
        form_grid.attach_next_to(
            &username_entry,
            Some(&username_label),
            gtk::PositionType::Right,
            1,
            1,
        );

        let password_label = Label::new(Some("Mot de passe :"));
        let password_entry = Entry::new();
        password_entry.set_visibility(false);
        form_grid.attach(&password_label, 0, 3, 1, 1);
        form_grid.attach_next_to(
            &password_entry,
            Some(&password_label),
            gtk::PositionType::Right,
            1,
            1,
        );

        dialog.show_all();

        let response_type = dialog.run();
        if response_type == gtk::ResponseType::Ok.into() {
            let name = name_entry.text();
            let host = host_entry.text();
            let username = username_entry.text();
            let password = password_entry.text();

            println!("Nom : {}", name);
            println!("Hôte : {}", host);
            println!("Nom d'utilisateur : {}", username);
            println!("Mot de passe : {}", password);
        }

        unsafe {
            dialog.destroy();
        }
        window_clone.show_all();
    });

    window.show_all();

    gtk::main();
}