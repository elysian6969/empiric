#![feature(array_methods)]
#![feature(const_mut_refs)]

pub mod elf;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod program;
pub mod section;

use elf::Elf;
use lexer::Lexer;
use op::{Op, Opcode};
use parser::{Argument, Parser};
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let code = fs::read_to_string(path).unwrap();

    println!();

    let mut lexer = Lexer::new(&code);

    for lexme in lexer {
        print!("{}", lexme.display());
    }

    println!();

    let mut parser = Parser::new(&code);
    let source = parser.parse();
    let mut bytes = vec![];
    let mut elf = Elf::new();

    let lea = Opcode::load_effective_address(69);

    //println!("{:02x?}", lea);

    elf.class64()
        .endian_little()
        .version(1)
        .abi_sysv()
        .abi_version()
        .kind_exec()
        .machine_x86_64()
        .version2()
        .entry_address(0x201120u64)
        .program_headers_address(64)
        .section_headers_address(352)
        .flags(0)
        .header64()
        .program_size(56)
        .program_len(4)
        .section_size(64)
        .section_len(3)
        .section_index(2);

    //println!("{:?} -> {:?}", &elf, elf.to_array());

    bytes.extend(elf.to_array().as_slice());

    let mut header = program::Header::new();

    header
        .kind(6)
        .flags(1 << 2)
        .offset(0x0000000000000040)
        .file_size(0x00000000000000e0)
        .virtual_address(0x0000000000200040)
        .memory_size(0x00000000000000e0)
        .physical_address(0x0000000000200040)
        .align(0x8);

    //println!("{:?} -> {:?}", &header, header.to_array());

    bytes.extend(header.to_array().as_slice());

    let mut header = program::Header::new();

    header
        .kind(1)
        .flags(1 << 2)
        .offset(0x0000000000000000)
        .file_size(0x0000000000000120)
        .virtual_address(0x0000000000200000)
        .memory_size(0x0000000000000120)
        .physical_address(0x0000000000200000)
        .align(0x1000);

    //println!("{:?} -> {:?}", &header, header.to_array());

    bytes.extend(header.to_array().as_slice());

    let mut header = program::Header::new();

    header
        .kind(1)
        .flags(1 | (1 << 2))
        .offset(0x0000000000000120)
        .file_size(0x000000000000002c)
        .virtual_address(0x0000000000201120)
        .memory_size(0x000000000000002c)
        .physical_address(0x0000000000201120)
        .align(0x1000);

    //println!("{:?} -> {:?}", &header, header.to_array());

    bytes.extend(header.to_array().as_slice());

    let mut header = program::Header::new();

    header.kind(0x6474_e551).flags((1 << 1) | (1 << 2));

    //println!("{:?} -> {:?}", &header, header.to_array());

    bytes.extend(header.to_array().as_slice());

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum Intermediate {
        lea64_str_rdi_rsi(usize),
        mov64_int_rax(u8),
        mov64_int_rdi(u8),
        mov64_rax_rdi,
        pop64_rax,
        pop64_rdx,
        push64(u8),
        syscall,
        xor64_rdi_rdi,
    }

    impl Intermediate {
        pub fn len(&self) -> usize {
            use Intermediate::*;

            match self {
                lea64_str_rdi_rsi(_) => 7,
                mov64_int_rax(_) => 7,
                mov64_int_rdi(_) => 7,
                mov64_rax_rdi => 3,
                pop64_rax => 1,
                pop64_rdx => 1,
                push64(_) => 2,
                syscall => 2,
                xor64_rdi_rdi => 3,
            }
        }
    }

    #[derive(Debug)]
    pub struct Code {
        pub ops: Vec<Intermediate>,
        pub strings: Vec<String>,
    }

    let mut code = Code {
        ops: vec![],
        strings: vec![],
    };

    for function in &source.functions {
        for syscall in &function.body {
            let mut reqstack = false;

            match &syscall.arg0 {
                Argument::String(_) => reqstack = true,
                _ => {}
            }

            if reqstack {
                code.ops.push(Intermediate::push64(syscall.id as u8));
                code.ops.push(Intermediate::pop64_rax);
            } else {
                code.ops.push(Intermediate::mov64_int_rax(syscall.id as u8));
            }

            match &syscall.arg0 {
                Argument::U64(int) => {
                    if reqstack {
                        code.ops.push(Intermediate::push64(*int as u8));
                        code.ops.push(Intermediate::pop64_rax);
                    } else {
                        if *int == 0 {
                            code.ops.push(Intermediate::xor64_rdi_rdi);
                        } else {
                            code.ops.push(Intermediate::mov64_int_rdi(*int as u8));
                        }
                    }
                }
                Argument::String(string) => {
                    code.ops
                        .push(Intermediate::lea64_str_rdi_rsi(code.strings.len()));
                    code.strings
                        .push(string[1..string.len() - 1].replace("\\n", "\n").to_string());
                }
                _ => {}
            }

            code.ops.push(Intermediate::syscall);
        }
    }

    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    let mut off = 0;

    for op in &code.ops {
        map.insert(off, op);

        off += op.len();
    }

    let mut coden = vec![];

    for (o, op) in map {
        match op {
            Intermediate::lea64_str_rdi_rsi(i) => {
                coden.push(Op::push64((code.strings[*i].len() + 1) as u8));
                coden.push(Op::pop64_rdx);
                coden.push(Op::lea64_rip_rsi((off - o - 5) as u8));
                coden.push(Op::mov64_rax_rdi);
            }
            Intermediate::mov64_int_rax(n) => {
                coden.push(Op::mov64_int_rax(*n));
            }
            Intermediate::mov64_int_rdi(n) => {
                coden.push(Op::mov64_int_rdi(*n));
            }
            Intermediate::mov64_rax_rdi => {
                coden.push(Op::mov64_rax_rdi);
            }
            Intermediate::pop64_rax => {
                coden.push(Op::pop64_rax);
            }
            Intermediate::pop64_rdx => {
                coden.push(Op::pop64_rdx);
            }
            Intermediate::push64(n) => {
                coden.push(Op::push64(*n));
            }
            Intermediate::syscall => {
                coden.push(Op::syscall);
            }
            Intermediate::xor64_rdi_rdi => {
                coden.push(Op::xor64_rdi_rdi);
            }
        }
    }

    for op in coden {
        let bytes2 = op.to_bytes();

        for byte in &bytes2 {
            print!("{byte:02x?} ");
        }

        for _ in bytes2.len()..7 {
            print!("   ");
        }

        println!("{}", op.display());

        bytes.extend(bytes2);
    }

    for string in &code.strings {
        bytes.extend(string.as_bytes());
    }

    bytes.extend(b"\0\0");
    bytes.extend(b".text\0");
    bytes.extend(b".shstrtab\0");
    bytes.extend(b"\0\0\0");

    let mut section_header = section::Header::new();

    bytes.extend(section_header.to_array().as_slice());

    // progbits
    let mut section_header = section::Header::new();

    section_header
        .name(1)
        .len(0x2c)
        .kind(1)
        .align(16)
        .offset(0x00000120u64)
        .address(0x0000000000201120u64)
        .flags(0x2 | 0x4);

    bytes.extend(section_header.to_array().as_slice());

    // strtab
    let mut section_header = section::Header::new();

    section_header
        .name(7)
        .len(0x11)
        .kind(3)
        .align(1)
        .offset(0x0000014Cu64);

    bytes.extend(section_header.to_array().as_slice());

    //println!("{bytes:02x?}");

    std::fs::write("bin.elf", &bytes[..]).unwrap();
}
