extern crate gtk;
extern crate gio;
extern crate gepub;

use gepub::{Doc, DocExtManual};
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
    Orientation,
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

        let doc = Doc::new("example.epub").expect("document");
        let viewer = gepub::Widget::new();
        viewer.set_hexpand(true);
        viewer.set_vexpand(true);
        viewer.set_doc(Some(&doc));
        let toc = doc.get_toc();
        println!("{:#?}", toc);

        let vbox = gtk::Box::new(Orientation::Vertical, 0);
        vbox.add(&viewer);
        window.add(&vbox);

        let hbox = gtk::Box::new(Orientation::Horizontal, 0);
        let previous = Button::with_label("Previous");
        {
            let viewer = viewer.clone();
            previous.connect_clicked(move |_| {
                viewer.page_prev();
            });
        }
        hbox.add(&previous);

        let next = Button::with_label("Next");
        {
            let viewer = viewer.clone();
            next.connect_clicked(move |_| {
                viewer.page_next();
            });
        }
        hbox.add(&next);
        vbox.add(&hbox);

        window.show_all();
    });

    application.run(&[]);
}
