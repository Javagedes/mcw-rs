
use mcw_rs::*;
use std::io;
use std::io::*;
fn main() {
    let x = || {println!("Server is ready to go!!")};
    
    let server = Builder::init()
        .add_event_callback(Event::OnServerReady, Box::from(x))
        .add_event_callback(Event::OnServerReady, Box::from(x))
        .build();

        println!("Press any button to stop...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        println!("{}",input);
}