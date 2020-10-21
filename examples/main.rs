extern crate gtk;
extern crate gio;
extern crate gepub;

use gepub::Doc;
use gio::{
    ApplicationExt,
    prelude::ApplicationExtManual,
};
use gtk::{
    Application,
    ApplicationWindow,
    Button,
    ButtonExt,
    ContainerExt,
    GtkWindowExt,
    WidgetExt,
};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let doc = Doc::new("/home/bouanto/Nadine Burke Harris, M.D - The deepest well_ healing the long-term effects of childhood adversity-Houghton Mifflin Harcourt (2018).epub").expect("document");
        let viewer = gepub::Widget::new();
        viewer.set_doc(Some(&doc));
        window.add(&viewer);

        window.show_all();
    });

    application.run(&[]);
}
