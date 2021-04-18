use gio::prelude::*;
use gtk::prelude::*;
use gtk::*;
use std::env::args;

fn main() {
    let application = gtk::Application::new(Some("trickypr.toc"), Default::default())
        .expect("GTK application initialization failed...");

    application.connect_activate(|app| {
        // Load the compiled resource bundle
        let resources_bytes = include_bytes!("../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::new_from_data(&resource_data).unwrap();
        gio::resources_register(&res);

        // Load the window UI
        let builder = Builder::new_from_resource("/trickypr/toc/toc.glade");

        // Get a reference to the window
        let window: ApplicationWindow = builder
            .get_object("main")
            .expect("Couldn't get the main application window");
        window.set_application(Some(app));

        // Show the UI
        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());

    println!("Hello, world!");
}
