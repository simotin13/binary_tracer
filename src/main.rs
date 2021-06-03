use std::env;
use std::fs::File;
use std::io::Read;
use std::mem;

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

const SIZE_ELF64_HALF: usize    = mem::size_of::<Elf64Half>();
const SIZE_ELF64_WORD: usize    = mem::size_of::<Elf64Word>();
const SIZE_ELF64_SWORD: usize   = mem::size_of::<Elf64Sword>();
const SIZE_ELF64_ADDR: usize    = mem::size_of::<Elf64Addr>();
const SIZE_ELF64_OFF: usize     = mem::size_of::<Elf64Off>();

const EI_NIDENT: usize  = 16;
const EI_CLASS: usize   = 4;
const EI_DATA: usize    = 5;    /* Data encoding byte index */
const EI_VERSION: usize = 6;    /* File version byte index */
const EI_OSABI: usize   = 7;    /* OS ABI identification */
const EI_ABIVERSION:usize   = 8;    /* ABI version */

const ELFOSABI_NONE: u8 = 0;    /* UNIX System V ABI */
const ELFOSABI_SYSV: u8	= 0;    /* Alias.  */
const ELFOSABI_HPUX: u8	= 1;    /* HP-UX */
const ELFOSABI_NETBSD: u8   = 2;  /* NetBSD.  */
const ELFOSABI_GNU: u8      = 3;  /* Object uses GNU ELF extensions.  */
const ELFOSABI_LINUX: u8    = ELFOSABI_GNU; /* Compatibility alias.  */
const ELFOSABI_SOLARIS: u8  = 6; /* Sun Solaris.  */
const ELFOSABI_AIX: u8      = 7;    /* IBM AIX.  */
const ELFOSABI_IRIX: u8		= 8;    /* SGI Irix.  */
const ELFOSABI_FREEBSD: u8  = 9;    /* FreeBSD.  */
const ELFOSABI_TRU64: u8    = 10;   /* Compaq TRU64 UNIX.  */
const ELFOSABI_MODESTO: u8  = 11;   /* Novell Modesto.  */
const ELFOSABI_OPENBSD: u8  = 12;   /* OpenBSD.  */
const ELFOSABI_ARM_AEABI: u8 = 64;      /* ARM EABI */
const ELFOSABI_ARM: u8      = 97;       /* ARM */
const ELFOSABI_STANDALONE: u8 = 255;    /* Standalone (embedded) application */

const ELFCLASSNONE: u8  = 0;
const ELFCLASS32: u8    = 1;
const ELFCLASS64: u8    = 2;

const ELFDATANONE: u8   = 0;		/* Invalid data encoding */
const ELFDATA2LSB: u8   = 1;		/* 2's complement, little endian */
const ELFDATA2MSB: u8   = 2;		/* 2's complement, big endian */
const ELFDATANUM: u8    = 3;

const ET_NONE:u16   = 0;     /* No file type */
const ET_REL:u16	= 1;    /* Relocatable file */
const ET_EXEC:u16	= 2;    /* Executable file */
const ET_DYN:u16    = 3;    /* Shared object file */
const ET_CORE:u16   = 4;    /* Core file */
const ET_NUM:u16    = 5;    /* Number of defined types */

const ELF64_OFFSET_E_TYPE: usize        = EI_NIDENT;
const ELF64_OFFSET_E_MACHINE: usize     = ELF64_OFFSET_E_TYPE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_VERSION: usize     = ELF64_OFFSET_E_MACHINE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_ENTRY: usize       = ELF64_OFFSET_E_VERSION + SIZE_ELF64_WORD;
const ELF64_OFFSET_E_PHOFF: usize       = ELF64_OFFSET_E_ENTRY + SIZE_ELF64_ADDR;
const ELF64_OFFSET_E_SHOFF: usize       = ELF64_OFFSET_E_PHOFF + SIZE_ELF64_OFF;
const ELF64_OFFSET_E_FLAGS: usize       = ELF64_OFFSET_E_SHOFF + SIZE_ELF64_WORD;
const ELF64_OFFSET_E_EHSIZE: usize      = ELF64_OFFSET_E_FLAGS + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_PHENTSIZE: usize   = ELF64_OFFSET_E_EHSIZE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_PHNUM: usize       = ELF64_OFFSET_E_PHENTSIZE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_SHENTSIZE: usize   = ELF64_OFFSET_E_PHNUM + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_SHNUM: usize       = ELF64_OFFSET_E_SHENTSIZE + SIZE_ELF64_HALF;
const ELF64_OFFSET_E_SHSTRNDX: usize    = ELF64_OFFSET_E_SHNUM + SIZE_ELF64_HALF;

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
        let e_type = Elf64Half::from_le_bytes(buf_half);

        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_MACHINE..ELF64_OFFSET_E_MACHINE + SIZE_ELF64_HALF]);
        let e_machine = Elf64Half::from_le_bytes(buf_half);

        buf_word.copy_from_slice(&bytes[ELF64_OFFSET_E_VERSION..ELF64_OFFSET_E_VERSION + SIZE_ELF64_WORD]);
        let e_version = Elf64Word::from_le_bytes(buf_word);

        buf_dword.copy_from_slice(&bytes[ELF64_OFFSET_E_ENTRY..ELF64_OFFSET_E_ENTRY + SIZE_ELF64_ADDR]);
        let e_entry = Elf64Addr::from_le_bytes(buf_dword);

        buf_dword.copy_from_slice(&bytes[ELF64_OFFSET_E_PHOFF..ELF64_OFFSET_E_PHOFF + SIZE_ELF64_OFF]);
        let e_phoff =  Elf64Off::from_le_bytes(buf_dword);

        buf_dword.copy_from_slice(&bytes[ELF64_OFFSET_E_SHOFF..ELF64_OFFSET_E_SHOFF + SIZE_ELF64_OFF]);
        let e_shoff =  Elf64Off::from_le_bytes(buf_dword);

        buf_word.copy_from_slice(&bytes[ELF64_OFFSET_E_FLAGS..ELF64_OFFSET_E_FLAGS + SIZE_ELF64_WORD]);
        let e_flags = Elf64Word::from_le_bytes(buf_word);
        
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_EHSIZE..ELF64_OFFSET_E_EHSIZE + SIZE_ELF64_HALF]);
        let e_ehsize = Elf64Half::from_le_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_PHENTSIZE..ELF64_OFFSET_E_PHENTSIZE + SIZE_ELF64_HALF]);
        let e_phentsize = Elf64Half::from_le_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_PHNUM..ELF64_OFFSET_E_PHNUM + SIZE_ELF64_HALF]);
        let e_phnum = Elf64Half::from_le_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_SHENTSIZE..ELF64_OFFSET_E_SHENTSIZE + SIZE_ELF64_HALF]);
        let e_shentsize = Elf64Half::from_le_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_SHNUM..ELF64_OFFSET_E_SHNUM + SIZE_ELF64_HALF]);
        let e_shnum = Elf64Half::from_le_bytes(buf_half);
        buf_half.copy_from_slice(&bytes[ELF64_OFFSET_E_SHSTRNDX..ELF64_OFFSET_E_SHSTRNDX + SIZE_ELF64_HALF]);
        let e_shstrndx = Elf64Half::from_le_bytes(buf_half);
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

    let elf64ehdr :Elf64Ehdr = Elf64Ehdr::new(&buf);
    show_elf_header_info(elf64ehdr);

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

fn show_elf_header_info(elf64ehdr: Elf64Ehdr) {
    println!("ELF Header:");

    // magic number
    print!("  Magic:{:3}", "");
    for by in elf64ehdr.e_ident.iter() {
        print!("{:02x} ", by);
    }
    println!("");

    // class
    match elf64ehdr.e_ident[EI_CLASS] {
        ELFCLASSNONE => println!("  Class:{:29}ELF None", ""),
        ELFCLASS32 => println!("  Class:{:29}ELF32", ""),
        ELFCLASS64 => println!("  Class:{:29}ELF64", ""),
        _ => println!("  Class: {:29}{}", "", elf64ehdr.e_ident[EI_CLASS])
    }

    // endian
    match elf64ehdr.e_ident[EI_DATA] {
        ELFDATANONE => println!("  Data:{:30}Invalid data encoding", ""),
        ELFDATA2LSB => println!("  Data:{:30}2's complement, little endian", ""),
        ELFDATA2MSB => println!("  Data:{:30}2's complement, big endian", ""),
        _ => println!("  Data: {:30}{}", "", elf64ehdr.e_ident[EI_CLASS])
    }

    // version
    println!("  Version:{:27}{}", "", elf64ehdr.e_ident[EI_VERSION]);

    // OS/ABI
    match elf64ehdr.e_ident[EI_OSABI] {
        ELFOSABI_NONE => println!("  OS/ABI:{:28}UNIX - System V", ""),
        ELFOSABI_SYSV => println!("  OS/ABI:{:28}Alias.", ""),
        ELFOSABI_HPUX => println!("  OS/ABI:{:28}HP-UX", ""),
        ELFOSABI_NETBSD => println!("  OS/ABI:{:28}NetBSD", ""),
        ELFOSABI_GNU => println!("  OS/ABI:{:28}Object uses GNU ELF extensions.", ""),
        ELFOSABI_LINUX => println!("  OS/ABI:{:28}Compatibility alias.", ""),
        ELFOSABI_SOLARIS => println!("  OS/ABI:{:28}Sun Solaris.", ""),
        ELFOSABI_AIX => println!("  OS/ABI:{:28}IBM AIX.", ""),
        ELFOSABI_IRIX => println!("  OS/ABI:{:28}SGI Irix.", ""),
        ELFOSABI_FREEBSD => println!("  OS/ABI:{:28}FreeBSD.", ""),
        ELFOSABI_TRU64 => println!("  OS/ABI:{:28}Compaq TRU64 UNIX.", ""),
        ELFOSABI_MODESTO => println!("  OS/ABI:{:28}Novell Modesto.", ""),
        ELFOSABI_OPENBSD =>	println!("  OS/ABI:{:28}OpenBSD.", ""),
        ELFOSABI_ARM_AEABI => println!("  OS/ABI:{:28}ARM EABI", ""),
        ELFOSABI_ARM => println!("  OS/ABI:{:28}ARM", ""),
        ELFOSABI_STANDALONE => println!("  OS/ABI:{:28}Standalone (embedded) application", ""),
        _ => println!("  OS/ABI:{:28}{}", "", elf64ehdr.e_ident[EI_OSABI])
    }

    // ABI Version
    println!("  ABI Version:{:23}{}", "", elf64ehdr.e_ident[EI_ABIVERSION]);

    // type
    print!("  Type:{:30}", "");
    match elf64ehdr.e_type {
        ET_NONE => println!("None"),     /* No file type */
        ET_REL	=> println!("REL"),    /* Relocatable file */
        ET_EXEC	=> println!("EXE"),    /* Executable file */
        ET_DYN  => println!("DYN"),    /* Shared object file */
        ET_CORE => println!("Core"),    /* Core file */
        _ => println!("{}", elf64ehdr.e_type)
    }
}
