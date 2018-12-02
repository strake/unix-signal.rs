#![no_std]

#[macro_use]
extern crate bitflags;
extern crate libc;
#[macro_use]
extern crate syscall;
#[macro_use]
extern crate unix;

bitflags! {
    pub struct Flags: usize {
    }
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{ucontext, mcontext, sigaction};

#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(target_os = "freebsd")]
pub use freebsd::{ucontext, mcontext};

pub type Handler = unsafe extern "C" fn(usize, &::libc::siginfo_t, &ucontext);

#[cfg(not(target_os = "linux"))]
pub unsafe fn sigaction(signal: usize, handler: Handler, flags: Flags) -> Result<Handler, ::unix::Error> {
    use core::mem;
    let mut old: ::libc::sigaction = mem::uninitialized();
    esyscall!(SIGACTION, signal, &::libc::sigaction {
        sa_sigaction: handler as _,
        sa_flags: (::libc::SA_SIGINFO as usize | flags.bits()) as _,
        .. mem::zeroed()
    } as *const _, &mut old as *mut _)?;
    Ok(mem::transmute(old.sa_sigaction))
}
