mod app_results;
mod path_results;
mod search_results;
mod search_window;

use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "com.github.prajwalch.launchthing";

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let window = search_window::SearchWindow::new(app);
        window.present();
    });
    app.run()
}
