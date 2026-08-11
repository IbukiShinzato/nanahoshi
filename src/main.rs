use std::env;
use std::fs;

// ELF Identification
const EI_MAG0: usize = 0;
const EI_MAG1: usize = 1;
const EI_MAG2: usize = 2;
const EI_MAG3: usize = 3;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
const EI_ABIVERSION: usize = 8;

// ELF Header Member
const E_TYPE: usize = 0x10;
const E_MACHINE: usize = 0x12;
const E_VERSION: usize = 0x14;
const E_ENTRY: usize = 0x18;
const E_PHOFF: usize = 0x20;
const E_SHOFF: usize = 0x28;
const E_FLAGS: usize = 0x30;
const E_EHSIZE: usize = 0x34;
const E_PHENTSIZE: usize = 0x36;
const E_PHNUM: usize = 0x38;
const E_SHENTSIZE: usize = 0x3a;
const E_SHNUM: usize = 0x3c;
const E_SHSTRNDX: usize = 0x3e;

#[derive(Debug)]
pub struct ElfIdent {
    class: u8,
    data: u8,
    version: u8,
    osabi: u8,
    abi_version: u8,
}

#[derive(Debug)]
enum ElfType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Unknown(u16),
}

#[derive(Debug)]
enum ElfVersion {
    None,
    Current,
    Unknown(u32),
}

#[derive(Debug)]
enum ElfMachine {
    X86_64,
    Unknown(u16),
}

#[derive(Debug)]
pub struct Elf64Ehdr {
    e_ident: ElfIdent,
    e_type: ElfType,
    e_machine: ElfMachine,
    e_version: ElfVersion,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn read_u64_le(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ])
}

pub fn parse_e_ident(data: &[u8]) -> ElfIdent {
    if data[EI_MAG0] != 0x7f
        || data[EI_MAG1] != b'E'
        || data[EI_MAG2] != b'L'
        || data[EI_MAG3] != b'F'
    {
        panic!("not an ELF file");
    }

    ElfIdent {
        class: data[EI_CLASS],
        data: data[EI_DATA],
        version: data[EI_VERSION],
        osabi: data[EI_OSABI],
        abi_version: data[EI_ABIVERSION],
    }
}

pub fn parse_elf64_header(data: &[u8]) -> Elf64Ehdr {
    if data.len() < 64 {
        panic!("file is too small to contain ELF64 header");
    }

    let e_ident = parse_e_ident(data);

    let raw_machine = read_u16_le(data, E_MACHINE);
    let e_machine = match raw_machine {
        62 => ElfMachine::X86_64,
        value => ElfMachine::Unknown(value),
    };

    let raw_type = read_u16_le(data, E_TYPE);
    let e_type = match raw_type {
        0 => ElfType::None,
        1 => ElfType::Rel,
        2 => ElfType::Exec,
        3 => ElfType::Dyn,
        4 => ElfType::Core,
        value => ElfType::Unknown(value),
    };

    let raw_version = read_u32_le(data, E_VERSION);
    let e_version = match raw_version {
        0 => ElfVersion::None,
        1 => ElfVersion::Current,
        value => ElfVersion::Unknown(value),
    };

    let e_entry = read_u64_le(data, E_ENTRY);
    let e_phoff = read_u64_le(data, E_PHOFF);
    let e_shoff = read_u64_le(data, E_SHOFF);
    let e_flags = read_u32_le(data, E_FLAGS);
    let e_ehsize = read_u16_le(data, E_EHSIZE);
    let e_phentsize = read_u16_le(data, E_PHENTSIZE);
    let e_phnum = read_u16_le(data, E_PHNUM);
    let e_shentsize = read_u16_le(data, E_SHENTSIZE);
    let e_shnum = read_u16_le(data, E_SHNUM);
    let e_shstrndx = read_u16_le(data, E_SHSTRNDX);

    Elf64Ehdr {
        e_ident,
        e_type,
        e_machine,
        e_version,
        e_entry,
        e_phoff,
        e_shoff,
        e_flags,
        e_ehsize,
        e_phentsize,
        e_phnum,
        e_shentsize,
        e_shnum,
        e_shstrndx,
    }
}

fn main() {
    let path = env::args().nth(1).expect("ELF file required");

    let data = fs::read(path).expect("failed to read file");

    let header = parse_elf64_header(&data);

    println!("{header:#?}");
}
