
use mcw_rs::*;

fn main() {
    let mut server = McServer::init();

    //server.test();

    let x = || {};

    server.add_event_callback(Event::OnServerReady, Box::from(x));
    server.add_event_callback(Event::OnServerReady, Box::from(x));
}