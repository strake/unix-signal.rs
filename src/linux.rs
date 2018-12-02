use core::mem;
use unix::Error as OsErr;
pub use ::libc::{ucontext_t as ucontext,
                 mcontext_t as mcontext};
use {Flags, Handler};

pub unsafe fn sigaction(signal: usize, handler: Handler, flags: Flags) -> Result<Handler, OsErr> {
    #[repr(C)]
    struct ksigaction {
        handler: Handler,
        flags: usize,
        restorer: Option<unsafe extern "C" fn() -> !>,
        mask: usize,
    }

    const SA_RESTORER: usize = 0x04000000;

    let _ = flags;
    unsafe extern "C" fn restore() -> ! {
        syscall!(RT_SIGRETURN);
        ::core::hint::unreachable_unchecked()
    }
    let mut old: ksigaction = mem::uninitialized();
    esyscall!(RT_SIGACTION, signal, &ksigaction {
        handler, flags: ::libc::SA_SIGINFO as usize | SA_RESTORER, restorer: Some(restore), mask: 0
    } as *const _, &mut old as *mut _, mem::size_of::<usize>())?;
    Ok(old.handler)
}
