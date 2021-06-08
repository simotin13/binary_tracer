use std::process::Command;
use std::process::Stdio;
use std::process::Child;
use std::io::Write;
use mio::{Events, Poll, Token};

pub struct GdbMi {
    target: String,
    timeout: u32,
    proc: Option<Child>,
}

impl GdbMi {
    pub fn new(target: &str, timeout:u32) -> GdbMi {

        GdbMi{target: target.to_string(), timeout: timeout, proc: None}
    }

    pub fn start(&mut self) {
        let mut proc = Command::new("gdb").
            args(&["-q", "-mi", &self.target]).
            stdin(Stdio::piped()).
            //stdout(Stdio::piped()).
            spawn().expect("gdb command failed");
        self.proc = Some(proc);
        /*
        let mut stdin = proc.stdin.take().expect("Failed to open stdin");
        stdin.write_all("b main".as_bytes()).expect("Failed to write to stdin");
        */
    }

    pub fn run(&mut self) {
        self.exec_cmd("-exec-run");
    }

    pub fn set_break_point(&mut self, func_name: &str) {
        self.exec_cmd(& format!("-break-insert {}", &func_name));
    }

    pub fn stepi(&mut self) {
        self.exec_cmd("-exec-step-instruction");
    }

    fn write_cmd(&mut self, cmd: &str) {
        let mut stdin = self.proc.take().unwrap().stdin.take().unwrap();
        stdin.write_all(cmd.as_bytes()).expect("Failed to write to stdin");
    }

    fn wait_until_writable(&self) {
        //poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();
        loop {
            // TODO
            //ret = IO::select(nil, [@stdin], nil, @timeout)
            //break if 0 < ret[1].length
            println!("stdin waiting...");
        }
    }

    fn check_status(&self) {
        println!("TODO");
    }

    fn exec_cmd(&mut self, cmd: &str) {
        self.wait_until_writable();
        self.write_cmd(cmd);
        self.check_status();
    }
}
