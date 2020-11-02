use std::process::Stdio;
use std::io::BufReader;
use std::process::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::thread;

pub enum Event {
    OnServerReady
}

pub struct McServer {

    child: Child,
    stdin: ChildStdin,
    //stdout: Arc<Mutex<BufReader<ChildStdout>>>,
    stdout_thread: Option<thread::JoinHandle<()>>,

    callbacks: Arc<Mutex<HashMap<u32, Vec<Arc<dyn Fn() + Send + Sync>>>>>
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
        let stdout = Arc::from(Mutex::from(BufReader::new(child.stdout.take().unwrap())));

        let callbacks: Arc<Mutex<HashMap<u32, Vec<Arc<dyn Fn() + Send + Sync>>>>> = Arc::from(Mutex::from(HashMap::new()));
        
        let callbacks2 = callbacks.clone();

        let callbacks = callbacks.clone();
        let stdout_thread = thread::Builder::new()
            .name(String::from("thread_server_listen"))
            .spawn( move || {
                loop { Self::listen(stdout.clone(), callbacks2.clone()); }
            }).unwrap();
        let stdout_thread = Some(stdout_thread);

        return McServer {
            child,
            stdin,
            //stdout,
            callbacks,
            stdout_thread
        }
    }

    pub fn add_event_callback(&mut self, event: Event, callback: Arc<dyn Fn() + Send + Sync>) {      
        let mut callbacks = self.callbacks.lock().unwrap();
        
        let vec = callbacks.entry(event as u32).or_insert(Vec::new());

        vec.push(callback);
    }

    pub fn listen(stdout: Arc<Mutex<BufReader<ChildStdout>>>, callbacks: Arc<Mutex<HashMap<u32, Vec<Arc<dyn Fn() + Send + Sync>>>>>) {

        loop {
            let mut lines = stdout.lock().unwrap();
            let lines = lines.by_ref().lines();
            for line in lines {
                let line = match line {
                    Ok(line) => {
                        line
                    }
                    Err(_) => {String::new()}
                };
    
                if line.contains("Done") {
                    Self::execute_callbacks(Event::OnServerReady, callbacks.clone());
                }
            }
        }
    }

    fn execute_callbacks(event: Event, callbacks: Arc<Mutex<HashMap<u32, Vec<Arc<dyn Fn() + Send + Sync>>>>>) {
        for callback in callbacks.lock().unwrap().get(&(event as u32)).unwrap() {
            callback();
        }
    }
}
