use gtk::prelude::*;
use gtk::{Button, Grid, Label, Window, WindowType, ScrolledWindow, TextView, PolicyType};

fn main() {
    // Initialize GTK application
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Grid Example");
    window.set_default_size(400, 300);

    // Create a new grid
    let grid = Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);
    grid.set_border_width(10);

    // Add widgets to the grid
    let label1 = Label::new(Some("Label 1"));
    let label2 = Label::new(Some("Label 2"));
    let button1 = Button::with_label("Button 1");
    let button2 = Button::with_label("Button 2");

    // Create a ScrolledWindow
    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_border_width(10);
    scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Automatic);

    // Create a TextView and add it to the ScrolledWindow
    let text_view = TextView::new();
    // Add the text view to the scrolled window
    scrolled_window.add(&text_view);

    // Attach widgets to grid cells
    grid.attach(&label1, 0, 0, 1, 1);
    grid.attach_next_to(&label2, Some(&label1), gtk::PositionType::Bottom, 1, 1);
    grid.attach_next_to(&button1, Some(&label1), gtk::PositionType::Right, 1, 1);
    grid.attach_next_to(&button2, Some(&button1), gtk::PositionType::Bottom, 1, 1);
    grid.attach_next_to(&scrolled_window, Some(&button2), gtk::PositionType::Bottom, 1, 1);

    // Add the grid to the window
    window.add(&grid);

    // Show all widgets
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
