use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Grid, Entry};
fn main() {
    let but_array = ['c', '√', '%', '^', '7', '8', '9', '÷', '4', '5', '6', 'x', '1', '2', '3', '-', '0', '.', '=', '+'];
    let app_name = "gtkcalc".to_string();
    let (grid_offset, text_box_height) = (3, 1);
    let app_interface = Application::builder()
        .application_id("com.example".to_string() + &app_name)
        .build();
    app_interface.connect_activate(move |app_interface| {
        let mut count = 0;
        let window = ApplicationWindow::builder()
            .application(app_interface)
            .default_height(300)
            .default_width(280)
            .title(&app_name)
            .build();
        let grid = Grid::builder() 
            .margin_top(grid_offset)
            .margin_bottom(grid_offset)
            .margin_start(grid_offset)
            .margin_end(grid_offset)
            .row_spacing(grid_offset)
            .column_spacing(grid_offset)
            .build();
        let top_panel = Entry::new();
        grid.attach(&top_panel, 0, 0, 4, text_box_height);
        top_panel.set_size_request(4, 3);
        for i in text_box_height..text_box_height + 5 {
            for j in 0..4{
                let button = Button::with_label(&but_array[count].to_string());
                //button.set_aspect_ratio(1.0);
                button.set_hexpand(true);
                button.set_vexpand(true);
                button.connect_clicked(move |_| {
                    println!("activated this: {:?}", but_array[count].to_string());
                });
                grid.attach(&button, j, i, 1, 1);
                count +=1;
            }
        }
        window.set_child(Some(&grid));
        window.show();
    });
    app_interface.run();
}
