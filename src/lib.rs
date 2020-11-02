use std::process::Stdio;
use std::process::*;
use std::collections::HashMap;
use std::io::{Read, Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

pub enum Event 
{   
    /// An event that occurs when the server begins the startup process
    OnServerStarting,
    /// An event that occurs when the server has loaded and is ready to accept clients
    OnServerReady,
    /// An event that occurs when the server has completely shutdown
    OnServerStop,
    /// An event that occurs when the eula has not been signed, causing the server to shutdown
    NeedEulaSigned,
}

pub struct Builder 
{
    callbacks: HashMap<u32, Vec<Box<dyn Fn() + Send>>>
}

impl Builder {
    pub fn init() -> Builder 
    {
        let mut callbacks = HashMap::default();

        callbacks.insert(Event::OnServerStop     as u32, Vec::default());
        callbacks.insert(Event::OnServerReady    as u32, Vec::default());
        callbacks.insert(Event::OnServerStarting as u32, Vec::default());
        callbacks.insert(Event::NeedEulaSigned   as u32, Vec::default());

        return Builder 
        {
            callbacks
        }
    }

    pub fn add_event_callback(mut self, event: Event, callback: Box<dyn Fn() + Send + Sync> ) -> Self 
    {
        let vec = self.callbacks.entry(event as u32).or_default();
        vec.push(callback);

        return self;
    }

    pub fn build(self)-> McServer 
    {
        return McServer::build(self)
    }
}

pub struct McServer 
{
    //child: Child,
    stdin: ChildStdin,
    //stdout: Arc<Mutex<BufReader<ChildStdout>>>,
    stdout_thread: Option<thread::JoinHandle<()>>,
    //callbacks: Arc<HashMap<u32, Vec<Box<dyn Fn() + Send + Sync>>>>
}

impl Drop for McServer 
{
    fn drop(&mut self) 
    {
        match self.stdin.write("/stop\n".as_bytes()) {
            Ok(_) => {},
            Err(_) => {/*When Server shuts down on its own and the pipe is disconnected*/}
        }
        self.stdout_thread.take().map(thread::JoinHandle::join);
    } 
}

impl McServer 
{
    fn build(builder: Builder)->McServer 
    {
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

        let callbacks = builder.callbacks;

        let stdout_thread = thread::Builder::new()
            .name(String::from("thread_server_listen"))
            .spawn( move || {
                Self::listen(stdout.clone(), callbacks);
            }).unwrap();
        
            let stdout_thread = Some(stdout_thread);

        return McServer {
            //child,
            stdin,
            //callbacks,
            stdout_thread
        }
    }

    fn listen(stdout: Arc<Mutex<BufReader<ChildStdout>>>, callbacks: HashMap<u32, Vec<Box<dyn Fn() + Send>>>) 
    {
        let mut stop = false;
        Self::execute_callbacks(Event::OnServerStarting, &callbacks);
        while !stop {
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
                    Self::execute_callbacks(Event::OnServerReady, &callbacks);
                }

                else if line.contains("Stopping server") {
                    stop = true;
                }

                else if line.contains("eula.txt") {
                    Self::execute_callbacks(Event::NeedEulaSigned, &callbacks);
                    stop = true;
                }
            }
        }
        Self::execute_callbacks(Event::OnServerStop, &callbacks);
    }

    fn execute_callbacks(event: Event, callbacks: &HashMap<u32, Vec<Box<dyn Fn() + Send>>>) {
        for callback in callbacks.get(&(event as u32)).unwrap() {
            callback();
        }
    }
}
