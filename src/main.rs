use gtk::prelude::*;
use gtk::{Button, Grid, Label, Window, WindowType};

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
    let button1 = Button::with_label("Button 1"); // Use with_label instead of new_with_label
    let button2 = Button::with_label("Button 2"); // Use with_label instead of new_with_label

    // Attach widgets to grid cells
    grid.attach(&label1, 0, 0, 1, 1); // Attach widget, column, row, width, height
    grid.attach_next_to(&label2, Some(&label1), gtk::PositionType::Bottom, 1, 1);
    grid.attach_next_to(&button1, Some(&label1), gtk::PositionType::Right, 1, 1);
    grid.attach_next_to(&button2, Some(&button1), gtk::PositionType::Bottom, 1, 1);

    // Add the grid to the window
    window.add(&grid);

    // Show all widgets
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
