//use tokio::process::*;
use std::process::Stdio;
use std::io::BufReader;
use std::process::*;
use std::io::BufRead;

//use tokio::io::{self, BufRead,AsyncBufReadExt, BufReader, AsyncReadExt, Lines};
//use tokio::prelude::*;

pub struct McServer {

    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>

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

        return McServer {
            child,
            stdin,
            stdout
        }
    }

    pub fn test(self) {
       
        let lines = self.stdout.lines();

        lines.for_each(|line| {
            println!("{:?}", line);
        })
    }
}
