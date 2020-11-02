use std::process::Stdio;
use std::io::BufReader;
use std::process::*;
use std::collections::HashMap;
use std::io::BufRead;

pub enum Event {
    OnServerReady
}

pub struct McServer {

    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,

    callbacks: HashMap<u32, Vec<Box<dyn Fn()>>>
}

impl McServer {

    pub fn init()->McServer {
        let mut child = Command::new("java")
                .current_dir("./server")
                .arg("-Xmx1024M")
                .arg("-Xms1024M")
                .arg("-jar")
                .arg("server.jar")
                .arg("nogui")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process");
  
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());

        let callbacks = HashMap::new();

        return McServer {
            child,
            stdin,
            stdout,
            callbacks
        }
    }

    pub fn add_event_callback(&mut self, event: Event, callback: Box<dyn Fn()>) {
        let event = event as u32;
        
        let vec = self.callbacks.entry(event).or_insert(Vec::new());

        vec.push(callback);

        println!("{:?}", self.callbacks.get(&event).unwrap().len());
    }

    pub fn test(self) {
       
        let lines = self.stdout.lines();

        lines.for_each(|line| {
            let line = match line {
                Ok(line) => {
                    line
                }
                Err(_) => {String::new()}
            };

            if line.contains("Done") {
                println!("Server is ready to go...");
            }
        })
    }
}
