use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;
use std::ptr::NonNull;
use std::panic::catch_unwind;

extern "C" {
    #[no_mangle]
    fn makecontext(
        __ucp: *mut ucontext_t,
        __func: Option<unsafe extern "C" fn() -> ()>,
        __argc: libc::c_int,
        _: ...
    );
    #[no_mangle]
    fn swapcontext(__oucp: *mut ucontext_t, __ucp: *const ucontext_t) -> libc::c_int;
    #[no_mangle]
    fn getcontext(_: *mut ucontext_t) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
struct __sigset_t {
    __val: [libc::c_ulong; 16],
}
type __uint16_t = libc::c_ushort;
type __uint32_t = libc::c_uint;
type __uint64_t = libc::c_ulong;
type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
struct sigaltstack {
    ss_sp: *mut libc::c_void,
    ss_flags: libc::c_int,
    ss_size: size_t,
}
type stack_t = sigaltstack;
type greg_t = libc::c_longlong;
type gregset_t = [greg_t; 23];
#[derive(Copy, Clone)]
#[repr(C)]
struct _libc_fpxreg {
    significand: [libc::c_ushort; 4],
    exponent: libc::c_ushort,
    padding: [libc::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct _libc_xmmreg {
    element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct _libc_fpstate {
    cwd: __uint16_t,
    swd: __uint16_t,
    ftw: __uint16_t,
    fop: __uint16_t,
    rip: __uint64_t,
    rdp: __uint64_t,
    mxcsr: __uint32_t,
    mxcr_mask: __uint32_t,
    _st: [_libc_fpxreg; 8],
    _xmm: [_libc_xmmreg; 16],
    padding: [__uint32_t; 24],
}
type fpregset_t = *mut _libc_fpstate;
#[derive(Copy, Clone)]
#[repr(C)]
struct mcontext_t {
    gregs: gregset_t,
    fpregs: fpregset_t,
    __reserved1: [libc::c_ulonglong; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ucontext {
    uc_flags: libc::c_ulong,
    uc_link: *mut ucontext,
    uc_stack: stack_t,
    uc_mcontext: mcontext_t,
    uc_sigmask: __sigset_t,
    __fpregs_mem: _libc_fpstate,
}
type ucontext_t = ucontext;

/***********************************************
*  gen
************************************************/

const DEFAULT_STACK_SIZE: usize = 1024 * 1024;

#[derive(Copy, Clone, Debug)]
enum GenState {
    Complete,
    Yield,
}

pub struct Gen<A, B> {
    gen: Box<UnwrapGen<A, B>>,
}

struct UnwrapGen<A, B> {
    state: GenState,
    ctx: ucontext,
    stack: Option<Vec<u8>>,
    send: A,
    co: NonNull<UnwrapGen<B, A>>,
    _pin: PhantomPinned,
}

impl<A, B> Gen<A, B> {
    pub fn new(f: for<'g> fn(&'g mut Gen<B, A>, A)) -> Self {
        let mut gen = Box::new(UnwrapGen {
            state: GenState::Yield,
            ctx: unsafe { mem::MaybeUninit::uninit().assume_init() },
            stack: Some(vec![0; DEFAULT_STACK_SIZE]),
            send: unsafe { mem::MaybeUninit::uninit().assume_init() },
            co: NonNull::dangling(),
            _pin: PhantomPinned,
        });

        let mut co_gen = Box::new(UnwrapGen {
            state: GenState::Yield,
            ctx: unsafe { mem::MaybeUninit::uninit().assume_init() },
            stack: None,
            send: unsafe { mem::MaybeUninit::uninit().assume_init() },
            co: NonNull::from(&*gen),
            _pin: PhantomPinned,
        });

        co_gen.ctx.uc_stack.ss_sp = 0 as *mut _;
        gen.co = NonNull::from(&*co_gen);
        gen.ctx.uc_stack.ss_sp = gen.stack.as_mut().unwrap().as_mut_ptr() as *mut _;
        gen.ctx.uc_stack.ss_size = DEFAULT_STACK_SIZE as size_t;
        gen.ctx.uc_stack.ss_flags = 0;
        gen.ctx.uc_link = &mut co_gen.ctx as *mut _;

        unsafe {
            getcontext(&mut gen.ctx as *mut _);
            makecontext(
                &mut gen.ctx as *mut _,
                mem::transmute(Some(
                    launch as unsafe fn(*mut UnwrapGen<B, A>, for<'g> fn(&'g mut Gen<B, A>, A)),
                )),
                2,
                Box::into_raw(co_gen),
                f,
            );
        }

        Gen { gen }
    }

    pub fn resume(&mut self, x: A) -> Option<B> {
        match self.gen.state {
            GenState::Complete => None,
            GenState::Yield => unsafe {
                mem::replace(&mut self.gen.send, x);
                swapcontext(&mut self.gen.co.as_mut().ctx, &mut self.gen.ctx);
                Some(mem::replace(&mut self.gen.co.as_mut().send, mem::MaybeUninit::uninit().assume_init()))
            },
        }
    }
}

unsafe fn launch<A, B>(g: *mut UnwrapGen<B, A>, f: for<'g> fn(&'g mut Gen<B, A>, A)) {
    let mut gen: Gen<B, A> = Gen {
        gen: Box::from_raw(g) ,
    };
    let start = mem::replace(&mut gen.gen.co.as_mut().send, mem::MaybeUninit::uninit().assume_init());
    f(&mut gen, start);
    gen.gen.co.as_mut().state = GenState::Complete;

    mem::forget(gen);
}

impl<A, B> Drop for Gen<A, B> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.gen.co.as_ptr())
        };
    }
}