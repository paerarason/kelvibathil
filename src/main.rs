use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button,Entry,Label};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("HTTP Client")
            .default_width(350)
            .default_height(70)
            .build();
        
        let url=Entry ::builder();
        let result=Label::builder();
        let button = Button::with_label("Send");

        let grid = gtk::Grid::new();
        grid.set_row_spacing(5); // Add spacing between rows (optional)
        grid.set_column_spacing(5); // Add spacing between columns (optional)
        window.add(&grid);
        button.connect_clicked(|_| {
            let link=url.get_text().unwrap();
            let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;
        });

        grid.attach(&url, 0, 0, 1, 1); // (x, y, width, height)
        grid.attach(&send, 1, 0, 1, 1); // Adjust positions as needed
        grid.attach(&result, 0, 1, 2, 1);
        window.show_all();
    });

    application.run();
}