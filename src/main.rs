use std::env;
use std::fs::File;
use std::io::Read;
use std::mem;

const EI_NIDENT: usize = 16;
const ELF64_OFFSET_E_TYPE: usize = EI_NIDENT;
type Elf32Half     = u16;
type Elf64Half     = u16;
type Elf32Word     = u32;
type Elf64Word     = u32;
type Elf32Sword    = i32;
type Elf64Sword    = i32;
type Elf32Addr     = u32;
type Elf64Addr     = u64;
type Elf32Off      = u32;
type Elf64Off      = u64;

struct Elf64Ehdr
{
    e_ident:    [u8; EI_NIDENT],   /* Magic number and other info */
    e_type:     Elf64Half,          /* Object file type */
    /*
    Elf64Half e_machine,      /* Architecture */
    Elf64Word e_version,      /* Object file version */
    Elf64Addr e_entry,        /* Entry point virtual address */
    Elf64Off  e_phoff,        /* Program header table file offset */
    Elf64Off  e_shoff,        /* Section header table file offset */
    Elf64Word e_flags,        /* Processor-specific flags */
    Elf64Half e_ehsize,       /* ELF header size in bytes */
    Elf64Half e_phentsize,    /* Program header table entry size */
    Elf64Half e_phnum,        /* Program header table entry count */
    Elf64Half e_shentsize,    /* Section header table entry size */
    Elf64Half e_shnum,        /* Section header table entry count */
    Elf64Half e_shstrndx,     /* Section header string table index */
    */
}

impl Elf64Ehdr {
    fn new(bytes: &[u8]) -> Elf64Ehdr{
        let mut e_ident :[u8; EI_NIDENT] = Default::default();
        e_ident.copy_from_slice(&bytes[0..EI_NIDENT]);

        let mut halfAry: [u8; 2] = Default::default();
        halfAry.copy_from_slice(&bytes[ELF64_OFFSET_E_TYPE..ELF64_OFFSET_E_TYPE + 2]);
        let e_type = Elf64Half::from_be_bytes(halfAry);
        Elf64Ehdr {
            e_ident: e_ident,
            e_type: e_type,
        }
    }
}

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

    let mut elf64ehdr :Elf64Ehdr = Elf64Ehdr::new(&buf);
    println!("{:?}", elf64ehdr.e_ident);

    /*
    unsafe {
        elf32Ehdr = mem::transmute::<[u8; 2], Elf32Ehdr>(ary);
    }
    */

    //println!("0x{:02X}", elf32Ehdr.e_type);
    /*
    let contents = fs::read_to_string(&argv[1]).expect(&msg);
    println!("{}", contents);
    */
}