#![no_std]

#[macro_use]
extern crate bitflags;
extern crate libc;
#[macro_use]
extern crate syscall;
#[macro_use]
extern crate unix;

use core::mem;
use unix::err::OsErr;

bitflags! {
    pub struct Flags: usize {
    }
}

#[repr(C)]
struct ksigaction {
    handler: Handler,
    flags: usize,
    restorer: Option<unsafe extern "C" fn() -> !>,
    mask: usize,
}

pub type Handler = unsafe extern "C" fn(usize, &::libc::siginfo_t, &::libc::ucontext_t);

#[cfg(target_os = "linux")]
pub unsafe fn sigaction(signal: usize, handler: Handler, flags: Flags) -> Result<Handler, OsErr> {
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

#[cfg(not(target_os = "linux"))]
pub unsafe fn sigaction(signal: usize, handler: Handler, flags: Flags) -> Result<Handler, OsErr> {
    let mut old: ::libc::sigaction = mem::uninitialized();
    esyscall!(SIGACTION, signal, &::libc::sigaction {
        sa_sigaction: handler as _,
        .. mem::zeroed()
    }, &mut old)?;
    Ok(mem::transmute(old.sa_sigaction))
}

const SA_RESTORER: usize = 0x04000000;
