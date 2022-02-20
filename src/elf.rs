#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Class {
    Class32 = 1,
    Class64 = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Endian {
    Big = 2,
    Little = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
enum Header {
    Header32 = 52,
    Header64 = 64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Elf {
    magic: [u8; 4],
    class: Class,
    endian: Endian,
    version: u8,
    abi_sysv: u8,
    abi_version: u8,
    _pad0: [u8; 7],
    kind: [u8; 2],
    machine: [u8; 2],
    version2: [u8; 4],
    entry_address: u64,
    program_headers_address: u64,
    section_headers_address: u64,
    flags: u32,
    header: Header,
    program_size: u16,
    program_len: u16,
    section_size: u16,
    section_len: u16,
    section_index: u16,
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            magic: *b"\x7fELF",
            class: Class::Class64,
            endian: Endian::Little,
            version: 0,
            abi_sysv: 0,
            abi_version: 0,
            _pad0: [0; 7],
            kind: [0; 2],
            machine: [0; 2],
            version2: [0; 4],
            entry_address: 0,
            program_headers_address: 0,
            section_headers_address: 0,
            flags: 0,
            header: Header::Header64,
            program_size: 0,
            program_len: 0,
            section_size: 0,
            section_len: 0,
            section_index: 0,
        }
    }

    pub const fn class32(&mut self) -> &mut Elf {
        self.class = Class::Class32;
        self
    }

    pub const fn class64(&mut self) -> &mut Elf {
        self.class = Class::Class64;
        self
    }

    pub const fn endian_big(&mut self) -> &mut Elf {
        self.endian = Endian::Big;
        self
    }

    pub const fn endian_little(&mut self) -> &mut Elf {
        self.endian = Endian::Little;
        self
    }

    pub const fn version(&mut self, version: u8) -> &mut Elf {
        self.version = version;
        self
    }

    pub const fn abi_sysv(&mut self) -> &mut Elf {
        self.abi_sysv = 0;
        self
    }

    pub const fn abi_version(&mut self) -> &mut Elf {
        self.abi_version = 0;
        self
    }

    pub const fn kind_exec(&mut self) -> &mut Elf {
        self.kind[0] = 2;
        self.kind[1] = 0;
        self
    }

    pub const fn machine_x86_64(&mut self) -> &mut Elf {
        self.machine[0] = 0x3E;
        self.machine[1] = 0x00;
        self
    }

    pub const fn version2(&mut self) -> &mut Elf {
        self.version2[0] = 1;
        self.version2[1] = 0;
        self.version2[2] = 0;
        self.version2[3] = 0;
        self
    }

    pub const fn entry_address(&mut self, address: u64) -> &mut Elf {
        self.entry_address = address;
        self
    }

    pub const fn program_headers_address(&mut self, address: u64) -> &mut Elf {
        self.program_headers_address = address;
        self
    }

    pub const fn section_headers_address(&mut self, address: u64) -> &mut Elf {
        self.section_headers_address = address;
        self
    }

    pub const fn flags(&mut self, flags: u32) -> &mut Elf {
        self.flags = flags;
        self
    }

    pub const fn header32(&mut self) -> &mut Elf {
        self.header = Header::Header32;
        self
    }

    pub const fn header64(&mut self) -> &mut Elf {
        self.header = Header::Header64;
        self
    }

    pub const fn program_size(&mut self, len: u16) -> &mut Elf {
        self.program_size = len;
        self
    }

    pub const fn program_len(&mut self, len: u16) -> &mut Elf {
        self.program_len = len;
        self
    }

    pub const fn section_size(&mut self, size: u16) -> &mut Elf {
        self.section_size = size;
        self
    }

    pub const fn section_len(&mut self, len: u16) -> &mut Elf {
        self.section_len = len;
        self
    }

    pub const fn section_index(&mut self, index: u16) -> &mut Elf {
        self.section_index = index;
        self
    }

    pub const fn to_array(self) -> [u8; 64] {
        unsafe { core::mem::transmute(self) }
    }
}
