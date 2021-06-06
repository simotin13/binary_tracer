use std::process::Command;
use std::process::Stdio;
use std::io::Write;

pub struct GdbMi {
    target: String,
    timeout: u32,
}

impl GdbMi {
    pub fn new(target: &str, timeout:u32) -> GdbMi {
        GdbMi{target: target.to_string(), timeout: timeout}
    }

    pub fn start(&self) {
        let mut proc = Command::new("gdb").
            args(&["-q", "-mi", &self.target]).
            stdin(Stdio::piped()).
            //stdout(Stdio::piped()).
            spawn().expect("gdb command failed");

            /*
        let mut stdin = proc.stdin.take().expect("Failed to open stdin");
        stdin.write_all("b main".as_bytes()).expect("Failed to write to stdin");
        */
    }

    pub fn run(&self) {
        self.exec_cmd("-exec-run");
    }

    fn write_cmd(&self, cmd: &str) {
        //self.stdin.write_all(cmd.as_bytes()).expect("Failed to write to stdin");
        println!("TODO");
    }

    fn wait_until_writable(&self) {
        println!("TODO");
    }

    fn check_status(&self) {
        println!("TODO");
    }

    fn exec_cmd(&self, cmd: &str) {
        self.wait_until_writable();
        self.write_cmd(cmd);
        self.check_status();
    }
}
