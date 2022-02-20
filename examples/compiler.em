struct elf {
  magic: [u8; 4],
  class: u8,
  endian: u8,
  version: u8,
  abi_sysv: u8,
  abi_version: u8,
  _pad0: [u8; 7],
  kind: u16,
  machine: u16,
  version2: u64,
  entry_address: u64,
  program_address: u64,
  section_address: u64,
  flags: u32,
  header: u16,
  program_size: u16,
  program_len: u16,
  section_size: u16,
}

fn entry() {
  let elf = elf {
    magic: '\x{7F}ELF',
    class: 2,
    endian: 1,
    version: 1,
    abi_sysv: 0,
    abi_version: 0,
    kind: 2,
    machine: 0x3E,
    version2: 1,
    entry_address: 0x201120,
    program_address: 64,
    section_address: 352,
    flags: 0,
    header: 64,
    section_size: 64,
    section_len: 3,
    section_index: 2,
  }

  sys::syscall(1, 'Hello ')
  sys::syscall(1, 'World!')
  sys::syscall(1, '\n')
  sys::syscall(60, 0)
}
