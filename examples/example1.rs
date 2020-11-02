
use mcw_rs::*;
use std::io;
fn main() {
    let x = || {println!("Callback: Server is started!")};
    let y = || {println!("Callback: Server is stopped!")};
    let z = || {println!("Callback: Sign the Eula!")};
    
    let _server = Builder::init()
        .add_event_callback(Event::OnServerReady, Box::from(x))
        .add_event_callback(Event::OnServerStop, Box::from(y))
        .add_event_callback(Event::NeedEulaSigned, Box::from(z))
        .build();

        println!("Press any button to stop...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        println!("{}",input);
}