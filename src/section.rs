#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Header {
    /// name (string table index)
    name: u32,
    /// kind
    kind: u32,
    /// flags
    flags: u64,
    /// virtual address
    address: u64,
    /// physical offset
    offset: u64,
    /// size
    len: u64,
    /// link to another section
    link: u32,
    /// additional information
    info: u32,
    /// align
    align: u64,
    /// entry size (if this section holds a table)
    entry_len: u64,
}

impl Header {
    pub const fn new() -> Header {
        Header {
            name: 0,
            kind: 0,
            flags: 0,
            address: 0,
            offset: 0,
            len: 0,
            link: 0,
            info: 0,
            align: 0,
            entry_len: 0,
        }
    }

    pub const fn name(&mut self, name: u32) -> &mut Header {
        self.name = name;
        self
    }

    pub const fn kind(&mut self, kind: u32) -> &mut Header {
        self.kind = kind;
        self
    }

    pub const fn flags(&mut self, flags: u64) -> &mut Header {
        self.flags = flags;
        self
    }

    pub const fn address(&mut self, address: u64) -> &mut Header {
        self.address = address;
        self
    }

    pub const fn offset(&mut self, offset: u64) -> &mut Header {
        self.offset = offset;
        self
    }

    pub const fn len(&mut self, len: u64) -> &mut Header {
        self.len = len;
        self
    }

    pub const fn link(&mut self, link: u32) -> &mut Header {
        self.link = link;
        self
    }

    pub const fn info(&mut self, info: u32) -> &mut Header {
        self.info = info;
        self
    }

    pub const fn align(&mut self, align: u64) -> &mut Header {
        self.align = align;
        self
    }

    pub const fn entry_len(&mut self, len: u64) -> &mut Header {
        self.entry_len = len;
        self
    }

    pub const fn to_array(self) -> [u8; 64] {
        unsafe { core::mem::transmute(self) }
    }
}
