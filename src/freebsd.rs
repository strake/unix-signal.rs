#[repr(C)]
#[derive(Clone, Copy)]
pub struct ucontext {
    pub uc_sigmask: ::libc::sigset_t,
    pub uc_mcontext: mcontext,
    pub uc_link: *mut ucontext,
    pub uc_stack: ::libc::stack_t,
    pub uc_flags: ::libc::c_int,
    pad: [::libc::c_int; 4]
}

#[cfg(target_arch = "x86_64")]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct mcontext {
    mc_onstack: usize,
    mc_regs: [usize; 15],
    mc_trapno: u32,
    mc_fs: u16,
    mc_gs: u16,
    mc_addr: usize,
    mc_flags: u32,
    mc_es: u16,
    mc_ds: u16,
    mc_err: usize,
    mc_rip: usize,
    mc_cs: usize,
    mc_rflags: usize,
    mc_rsp: usize,
    mc_ss: usize,

    mc_len: ::libc::c_long,

    mc_fpformat: ::libc::c_long,
    mc_ownedfp: ::libc::c_long,
    mc_fpstate: FPState,

    mc_fsbase: usize,
    mc_gsbase: usize,

    mc_xfpustate: usize,
    mc_xfpustate_len: usize,

    mc_spare: [::libc::c_long; 4]
}

#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct FPState(pub [::libc::c_long; 64]);
