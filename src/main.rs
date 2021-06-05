use std::env;
use std::fs::File;
use std::io::Read;
use std::mem;
mod elf;

fn main() {

    let argv:Vec<String> = env::args().collect();
    if argv.len() < 2 {
        eprintln!("no input file");
        std::process::exit(-1);
    }

    let msg = format!("File open failed filepath:{}", &argv[1]);
    let mut f = File::open(&argv[1]).expect(&msg);
    let mut buf = Vec::new();
    let _ret = f.read_to_end(&mut buf);

    let elf64ehdr: elf::Elf64Ehdr = elf::Elf64Ehdr::new(&buf);
    elf64ehdr.show_elf_header_info();
}

