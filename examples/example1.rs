
use mcw_rs::*;

fn main() {
    let mut server = McServer::init();

    let x = || {println!("Server is ready to go!!")};

    server.add_event_callback(Event::OnServerReady, Box::from(x));
    server.add_event_callback(Event::OnServerReady, Box::from(x));

    server.listen();
    println!("Finished listening");
}