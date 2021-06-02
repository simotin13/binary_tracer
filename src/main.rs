use std::env;
use std::fs::File;
use std::io::Read;
use std::mem;

const SIZE_ELF64_HALF: usize    = mem::size_of::<Elf64Half>();
const SIZE_ELF64_WORD: usize    = mem::size_of::<Elf64Word>();
const SIZE_ELF64_SWORD: usize   = mem::size_of::<Elf64Sword>();
const SIZE_ELF64_ADDR: usize    = mem::size_of::<Elf64Addr>();
const SIZE_ELF64_OFF: usize     = mem::size_of::<Elf64Off>();

const EI_NIDENT: usize = 16;
const ELF64_OFFSET_E_TYPE: usize    = EI_NIDENT;
const ELF64_OFFSET_E_MACHINE: usize = ELF64_OFFSET_E_TYPE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_VERSION: usize = ELF64_OFFSET_E_MACHINE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_ENTRY: usize   = ELF64_OFFSET_E_VERSION + SIZE_ELF64_WORD;
const ELF64_OFFSET_E_PHOFF: usize   = ELF64_OFFSET_E_ENTRY + SIZE_ELF64_ADDR;
const ELF64_OFFSET_E_SHOFF: usize   = ELF64_OFFSET_E_PHOFF + SIZE_ELF64_OFF;
const ELF64_OFFSET_E_FLAGS: usize   = ELF64_OFFSET_E_SHOFF + SIZE_ELF64_WORD;
const ELF64_OFFSET_E_EHSIZE: usize   = ELF64_OFFSET_E_FLAGS + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_PHENTSIZE: usize   = ELF64_OFFSET_E_EHSIZE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_PHNUM: usize   = ELF64_OFFSET_E_PHENTSIZE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_SHENTSIZE: usize   = ELF64_OFFSET_E_PHNUM + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_SHNUM: usize   = ELF64_OFFSET_E_SHENTSIZE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_SHSTRNDX: usize   = ELF64_OFFSET_E_SHNUM + SIZE_ELF64_HALF;

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
    e_ident:        [u8; EI_NIDENT],    // Magic number and other info
    e_type:         Elf64Half,          // Object file type
    e_machine:      Elf64Half,          // Architecture
    e_version:      Elf64Word,          // Object file version
    e_entry:        Elf64Addr,          // Entry point virtual address
    e_phoff:        Elf64Off,           // Program header table file offset
    e_shoff:        Elf64Off,           // Section header table file offset
    e_flags:        Elf64Word,          // Processor-specific flags
    e_ehsize:       Elf64Half,          // ELF header size in bytes
    e_phentsize:    Elf64Half,          // Program header table entry size
    e_phnum:        Elf64Half,          // Program header table entry count
    e_shentsize:    Elf64Half,          // Section header table entry size
    e_shnum:        Elf64Half,          // Section header table entry count
    e_shstrndx:     Elf64Half,          // Section header string table index
}

impl Elf64Ehdr {
    fn new(bytes: &[u8]) -> Elf64Ehdr{
        let mut buf_half: [u8; 2] = Default::default();
        let mut buf_word: [u8; 4] = Default::default();
        let mut buf_dword: [u8; 8] = Default::default();

        let mut e_ident :[u8; EI_NIDENT] = Default::default();
        e_ident.copy_from_slice(&bytes[0..EI_NIDENT]);

        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_TYPE..ELF64_OFFSET_E_TYPE + SIZE_ELF64_HALF]);
        let e_type = Elf64Half::from_be_bytes(buf_half);

        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_MACHINE..ELF64_OFFSET_E_MACHINE + SIZE_ELF64_HALF]);
        let e_machine = Elf64Half::from_be_bytes(buf_half);

        buf_word.copy_from_slice(&bytes[ELF64_OFFSET_E_VERSION..ELF64_OFFSET_E_VERSION + SIZE_ELF64_WORD]);
        let e_version = Elf64Word::from_be_bytes(buf_word);

        buf_dword.copy_from_slice(&bytes[ELF64_OFFSET_E_VERSION..ELF64_OFFSET_E_ENTRY + SIZE_ELF64_ADDR]);
        let e_entry = Elf64Addr::from_be_bytes(buf_dword);

        buf_dword.copy_from_slice(&bytes[ELF64_OFFSET_E_PHOFF..ELF64_OFFSET_E_PHOFF + SIZE_ELF64_OFF]);
        let e_phoff =  Elf64Off::from_be_bytes(buf_dword);

        buf_dword.copy_from_slice(&bytes[ELF64_OFFSET_E_SHOFF..ELF64_OFFSET_E_SHOFF + SIZE_ELF64_OFF]);
        let e_shoff =  Elf64Off::from_be_bytes(buf_dword);

        buf_word.copy_from_slice(&bytes[ELF64_OFFSET_E_FLAGS..ELF64_OFFSET_E_FLAGS + SIZE_ELF64_WORD]);
        let e_flags = Elf64Word::from_be_bytes(buf_word);
        
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_EHSIZE..ELF64_OFFSET_E_EHSIZE + SIZE_ELF64_HALF]);
        let e_ehsize = Elf64Half::from_be_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_PHENTSIZE..ELF64_OFFSET_E_PHENTSIZE + SIZE_ELF64_HALF]);
        let e_phentsize = Elf64Half::from_be_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_PHNUM..ELF64_OFFSET_E_PHNUM + SIZE_ELF64_HALF]);
        let e_phnum = Elf64Half::from_be_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_SHENTSIZE..ELF64_OFFSET_E_SHENTSIZE + SIZE_ELF64_HALF]);
        let e_shentsize = Elf64Half::from_be_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_SHNUM..ELF64_OFFSET_E_SHNUM + SIZE_ELF64_HALF]);
        let e_shnum = Elf64Half::from_be_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_SHSTRNDX..ELF64_OFFSET_E_SHSTRNDX + SIZE_ELF64_HALF]);
        let e_shstrndx = Elf64Half::from_be_bytes(buf_half);
        Elf64Ehdr {
            e_ident: e_ident,
            e_type: e_type,
            e_machine: e_machine,
            e_version: e_version,
            e_entry: e_entry,
            e_phoff: e_phoff,
            e_shoff: e_shoff,
            e_flags: e_flags,
            e_ehsize: e_ehsize,
            e_phentsize: e_phentsize,
            e_phnum: e_phnum,
            e_shentsize: e_shentsize,
            e_shnum: e_shnum,
            e_shstrndx: e_shstrndx,
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