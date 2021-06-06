use std::env;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::process::Stdio;
use std::io::Write;

mod elf;
mod gdbmi;

fn main() {

    let argv:Vec<String> = env::args().collect();
    if argv.len() < 2 {
        eprintln!("input debug target");
        std::process::exit(-1);
    }

    let msg = format!("File open failed filepath:{}", &argv[1]);
    let mut f = File::open(&argv[1]).expect(&msg);
    let mut buf = Vec::new();
    let _ret = f.read_to_end(&mut buf);

    let elf64ehdr: elf::Elf64Ehdr = elf::Elf64Ehdr::new(&buf);
    elf64ehdr.show_elf_header_info();

    // start gdb
    let mut gdbmi = gdbmi::GdbMi::new(&argv[1], 1000);
    gdbmi.start()
}

