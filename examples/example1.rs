
use mcw_rs::*;
use std::sync::Arc;

fn main() {
    let mut server = McServer::init();

    let x = || {println!("Server is ready to go!!")};

    server.add_event_callback(Event::OnServerReady, Arc::from(x));
    server.add_event_callback(Event::OnServerReady, Arc::from(x));

    loop {}
}