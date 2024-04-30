extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Entry};

fn main() {
    // Initialize GTK application
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("KElvi bathil");
    window.set_default_size(900, 450);

    // Create a new entry and button
    let url = Entry::new();
    let send = Button::with_label("send");
    url.set_text("URL://");

    // Connect a callback to the entry to handle text changes
    url.connect_changed(|entry| {
        if let Some(text) = entry.get_text() {
            println!("Entry text changed: {}", text);
        }
    });

    let paned = gtk::Paned::new(gtk::Orientation::Horizontal);

    // Add widgets to the paned container and allow resizing
    paned.pack1(&url, true, true);
    paned.pack2(&send, true, true);

    // Add the paned container to the window
    window.add(&paned);
    
    // Show all widgets
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
