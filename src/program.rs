#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Header {
    /// kind
    kind: u32,
    /// flags
    flags: u32,
    /// offset
    offset: u64,
    /// virtual address
    virtual_address: u64,
    /// physical address
    physical_address: u64,
    /// physical length
    file_size: u64,
    /// virtual length
    memory_size: u64,
    /// align
    align: u64,
}

impl Header {
    pub const fn new() -> Header {
        Header {
            kind: 0,
            flags: 0,
            offset: 0,
            virtual_address: 0,
            physical_address: 0,
            file_size: 0,
            memory_size: 0,
            align: 0,
        }
    }

    pub const fn kind(&mut self, kind: u32) -> &mut Header {
        self.kind = kind;
        self
    }

    pub const fn flags(&mut self, flags: u32) -> &mut Header {
        self.flags = flags;
        self
    }

    pub const fn offset(&mut self, offset: u64) -> &mut Header {
        self.offset = offset;
        self
    }

    pub const fn virtual_address(&mut self, address: u64) -> &mut Header {
        self.virtual_address = address;
        self
    }

    pub const fn physical_address(&mut self, address: u64) -> &mut Header {
        self.physical_address = address;
        self
    }

    pub const fn file_size(&mut self, len: u64) -> &mut Header {
        self.file_size = len;
        self
    }

    pub const fn memory_size(&mut self, len: u64) -> &mut Header {
        self.memory_size = len;
        self
    }

    pub const fn align(&mut self, align: u64) -> &mut Header {
        self.align = align;
        self
    }

    pub const fn to_array(self) -> [u8; 56] {
        unsafe { core::mem::transmute(self) }
    }
}
