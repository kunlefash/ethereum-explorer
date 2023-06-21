use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct GuiApp {
    runtime: Runtime,
    window: Window,
    block_entry: Entry,
    block_button: Button,
    block_label: Label,
    shutdown_flag: Arc<AtomicBool>,
}

impl GuiApp {
    pub fn new() -> Self {
        // Shared Arc reference to the Ethereum client
        let (_eloop, transport) =
            web3::transports::Http::new("https://mainnet.infura.io/v3/my_infura_id")
                .expect("Failed to create Ethereum transport");
        let web3 = Arc::new(web3::Web3::new(transport));

        // Main window
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Ethereum Explorer");
        window.set_default_size(300, 200);

        // Entry for block number input
        let block_entry = Entry::new();
        block_entry.set_placeholder_text(Some("Enter block number"));

        // Button to fetch block information
        let block_button = Button::with_label("Get Block Info");

        //Displays a block information
        let block_label = Label::new(None);

        //Main layout
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        layout.pack_start(&block_entry, false, false, 0);
        layout.pack_start(&block_button, false, false, 0);
        layout.pack_start(&block_label, false, false, 0);

        window.add(&layout);

        //Shared flag to handle shutdown
        let shutdown_flag = Arc::new(AtomicBool::new(false));

        // Clone data for event handlers
        let web3_clone = Arc::clone(&web3);
        let block_entry_clone = block_entry.clone();
        let block_label_clone = block_label.clone();
        let shutdown_flag_clone = Arc::clone(&shutdown_flag);

        block_button.connect_clicked(move |_| {
            let block_number = block_entry_clone.get_text().unwrap().to_string();
            let web3 = web3_clone.clone();
            let block_label = block_label_clone.clone();
            let shutdown_flag = shutdown_flag_clone.clone();

            // Spawn a task on the runtime to fetch block information
            let response = get_block_info(block_number, web3);
            let runtime = Runtime::new().expect("Failed to create Tokio runtime");
            runtime.spawn(async move {
                let result = response.await;
                runtime.spawn_blocking(move || {
                    block_label.set_text(&result.unwrap_or_else(|_| "Failed to fetch block info".to_string()));
                    shutdown_flag.store(true, Ordering::Relaxed);
                });
            });
        });

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        GuiApp {
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
            window,
            block_entry,
            block_button,
            block_label,
            shutdown_flag,
        }
    }

    pub fn run(&self) {
        self.window.show_all();
        self.runtime.enter(|| gtk::main());
    }

    pub async fn shutdown(&self) {
        while !self.shutdown_flag.load(Ordering::Relaxed) {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        self.runtime.shutdown_timeout(std::time::Duration::from_secs(1));
    }
}
