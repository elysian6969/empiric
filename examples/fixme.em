fn entry() {
    sys::syscall(1, 'Hello ')
    sys::syscall(1, 'World!')
    sys::syscall(1, '\n')
    sys::syscall(60, 0)
}
