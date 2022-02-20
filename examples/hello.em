fn entry() {
    sys::syscall(1, 'hello world\n')
    sys::syscall(60, 0)
}
