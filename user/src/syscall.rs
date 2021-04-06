use crate::console::STDOUT;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

const STACK_SIZE: usize = 0x1000;

const STEXT: usize = 0x80200000;
const ETEXT: usize = 0x80205000;
const SDATA: usize = 0x8020b000;
const EDATA: usize = 0x80214000;
const SBSS: usize = 0x80224000;
const EBSS: usize = 0x80225000;

unsafe fn r_sp() -> usize {
    let mut sp: usize;
    llvm_asm!("mv $0, sp": "=r"(sp) ::: "volatile");
    sp
}

unsafe fn stack_range() -> (usize, usize) {
    let sp = r_sp();
    let top = (sp + STACK_SIZE - 1) & (!(STACK_SIZE - 1));
    (top - STACK_SIZE, top)
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    let head = buffer.as_ptr() as usize;
    let tail = head + buffer.len() as usize;
    unsafe {
        let (bottom, top) = stack_range();
        if !(SDATA <= head && tail <= EDATA) // OK
            && !(STEXT <= head && tail <= ETEXT) // OK
            && !(SBSS <= head && tail <= EBSS) // OK
            && !(bottom <= head && tail <= top) // OK
            && !(head >= 0x80400000)
        {
            return -1;
        }
    };

    match fd {
        STDOUT => syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()]),
        _ => -1,
    }
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
