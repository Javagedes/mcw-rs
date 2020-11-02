use std::process::Stdio;
use std::io::BufReader;
use std::process::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Read;
use std::sync::Mutex;
use std::io::Write;

pub enum Event {
    OnServerReady
}

pub struct McServer {

    child: Child,
    stdin: ChildStdin,
    stdout: Mutex<BufReader<ChildStdout>>,

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
        let stdout = Mutex::from(BufReader::new(child.stdout.take().unwrap()));

        let callbacks = HashMap::new();

        return McServer {
            child,
            stdin,
            stdout,
            callbacks
        }
    }

    pub fn add_event_callback(&mut self, event: Event, callback: Box<dyn Fn()>) {      
        let vec = self.callbacks.entry(event as u32).or_insert(Vec::new());

        vec.push(callback);
    }

    pub fn listen(&self) {

        loop {
            let mut lines = self.stdout.lock().unwrap();
            let lines = lines.by_ref().lines();
            for line in lines {
                let line = match line {
                    Ok(line) => {
                        line
                    }
                    Err(_) => {String::new()}
                };
    
                if line.contains("Done") {
                    self.execute_callbacks(Event::OnServerReady);
                }
            }
        }
    }

    fn execute_callbacks(&self, event: Event) {
        for callback in self.callbacks.get(&(event as u32)).unwrap() {
            callback();
        }
    }
}
