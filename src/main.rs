mod api;
mod gui;

use actix_web::HttpServer;
use gtk::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix web server
    let server = HttpServer::new(|| actix_web::App::new().service(api::get_block_info))
        .bind("127.0.0.1:8080")?
        .run();

    // Initialize the GTK runtime
    gtk::init().expect("Failed to initialize GTK.");

    // Create and run the GUI application
    let gui = gui::GuiApp::new();
    gui.run();

    // Shutdown the server and GTK event loop gracefully
    futures::try_join!(server, gui.shutdown()).map(|_| ())
}
