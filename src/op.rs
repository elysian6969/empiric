#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    /// leaq <int>(%rip), %rsi
    lea64_rip_rsi(u8),
    /// movq <int>, %rax
    mov64_int_rax(u8),
    /// movq <int>, %rdi
    mov64_int_rdi(u8),
    /// movq %rax, %rdi
    mov64_rax_rdi,
    /// popq %rax
    pop64_rax,
    /// popq %rdx
    pop64_rdx,
    /// pushq <int>
    push64(u8),
    /// syscall
    syscall,
    /// xorq %rdi, %rdi
    xor64_rdi_rdi,
}

impl Op {
    pub fn to_bytes(&self) -> std::vec::Vec<u8> {
        use Op::*;

        match self {
            lea64_rip_rsi(n) => vec![0x48, 0x8D, 0x35, *n, 0x00, 0x00, 0x00],
            mov64_int_rax(n) => vec![0x48, 0xC7, 0xC0, *n, 0x00, 0x00, 0x00],
            mov64_int_rdi(n) => vec![0x48, 0xC7, 0xC7, *n, 0x00, 0x00, 0x00],
            mov64_rax_rdi => vec![0x48, 0x89, 0xC7],
            pop64_rax => vec![0x58],
            pop64_rdx => vec![0x5A],
            push64(n) => vec![0x6A, *n],
            syscall => vec![0x0F, 0x05],
            xorq_rdi_rdi => vec![0x48, 0x31, 0xFF],
        }
    }

    pub fn display(&self) -> String {
        use Op::*;

        match self {
            lea64_rip_rsi(n) => format!("leaq \x1b[38;5;11m{n}\x1b[m(%rip), %rsi"),
            mov64_int_rax(n) => format!("movq \x1b[38;5;11m${n}\x1b[m, %rax"),
            mov64_int_rdi(n) => format!("movq \x1b[38;5;11m${n}\x1b[m, %rdi"),
            mov64_rax_rdi => format!("movq %rax, %rdi"),
            pop64_rax => format!("popq %rax"),
            pop64_rdx => format!("popq %rdx"),
            push64(n) => format!("pushq \x1b[38;5;11m${n}\x1b[m"),
            syscall => format!("syscall"),
            xorq_rdi_rdi => format!("xorq $rdi, $rdi"),
        }
    }
}

use pancake::Vec;

#[derive(Debug)]
pub struct Opcode {
    /// Opcodes have a max encoded length of 15 bytes.
    data: Vec<u8, 15>,
}

impl Opcode {
    pub(crate) const fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub const fn len(&self) -> usize {
        self.data.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub const fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub(crate) const unsafe fn push_bytes_unchecked(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice_unchecked(bytes)
    }

    pub const fn load_effective_address(address: u32) -> Self {
        let mut this = Self::new();
        let address = address.to_le_bytes();

        unsafe {
            this.push_bytes_unchecked(&[0x48, 0x8D, 0x35]);
            this.push_bytes_unchecked(&address);
        }

        this
    }

    pub const fn move_to_register(register: u8, integer: u64) -> Self {
        let mut this = Self::new();
        let register = register.to_le_bytes();
        let integer = integer.to_le_bytes();

        unsafe {
            this.push_bytes_unchecked(&[0x48, 0xC7]);
            this.push_bytes_unchecked(&register);
            this.push_bytes_unchecked(&integer);
        }

        this
    }
}
