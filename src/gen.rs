use std::marker::PhantomPinned;
use std::ptr;
use std::ptr::NonNull;

#[cfg(not(target_os="windows"))]
global_asm!{
    include_str!("context_linux.s")
}

#[cfg(target_os="windows")]
global_asm!{
    include_str!("context_windows.s")
}

extern "C" {
    fn switch_ctx(old: *mut Ctx, new: *const Ctx);
    fn set_ctx(new: *const Ctx) -> !;
}

pub struct Gen<'a, A, B> {
    gen: Box<UnwrapGen<'a, A, B>>,
}

const DEFAULT_STACK_SIZE: usize = 1024 * 1024;

#[cfg(not(target_os="windows"))]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct Ctx {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    gen_ptr: u64,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct Ctx {
    xmm6: [u64; 2],
    xmm7: [u64; 2],
    xmm8: [u64; 2],
    xmm9: [u64; 2],
    xmm10: [u64; 2],
    xmm11: [u64; 2],
    xmm12: [u64; 2],
    xmm13: [u64; 2],
    xmm14: [u64; 2],
    xmm15: [u64; 2],
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    rdi: u64,
    rsi: u64,
    stack_start: u64,
    stack_end: u64,
    gen_ptr: u64,
}

#[derive(Copy, Clone, Debug)]
enum GenState {
    Complete,
    Yield,
}

struct UnwrapGen<'a, A, B> {
    state: GenState,
    ctx: Ctx,
    stack: Option<Vec<u8>>,
    send: Option<A>,
    co: NonNull<UnwrapGen<'a, B, A>>,
    f: Option<Box<dyn for<'g> FnMut(&'g mut Gen<A, B>, B) + 'a>>,
    _pin: PhantomPinned,
}


impl<'a, A, B> Gen<'a, A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: for<'g> FnMut(&'g mut Gen<B, A>, A) + 'a,
    {
        let f = Box::new(f) as Box<dyn for<'g> FnMut(&'g mut Gen<B, A>, A) + 'a>;
        let mut gen = Box::new(UnwrapGen {
            state: GenState::Yield,
            ctx: Ctx::default(),
            stack: Some(vec![0; DEFAULT_STACK_SIZE]),
            send: None,
            co: NonNull::dangling(),
            f: None,
            _pin: PhantomPinned,
        });

        let co_gen = Box::new(UnwrapGen {
            state: GenState::Yield,
            ctx: Ctx::default(),
            stack: None,
            send: None,
            co: NonNull::from(&*gen),
            f: Some(f),
            _pin: PhantomPinned,
        });
        gen.co = NonNull::from(&*co_gen);

        let size = gen.stack.as_ref().unwrap().len();
        let s_ptr = gen.stack.as_mut().unwrap().as_mut_ptr();
        gen.ctx.gen_ptr = Box::into_raw(co_gen) as *const _ as u64;
        unsafe {
            ptr::write(
                s_ptr.add(size - 16) as *mut u64,
                launch::<A, B> as usize as u64,
            );
            gen.ctx.rsp = s_ptr.add(size - 16) as u64;
        }

        Gen { gen }
    }

    pub fn resume(&mut self, x: A) -> Option<B> {
        match self.gen.state {
            GenState::Complete => None,
            GenState::Yield => unsafe {
                self.gen.send.replace(x);
                switch_ctx(&mut self.gen.co.as_mut().ctx, &self.gen.ctx);
                self.gen.co.as_mut().send.take()
            },
        }
    }
}

unsafe fn launch<A, B>(gen: u64) -> ! {
    let gen = gen as *mut UnwrapGen<B, A>;
    let mut gen: Gen<B, A> = Gen {
        gen: Box::from_raw(gen),
    };

    let start = gen.gen.co.as_mut().send.take().unwrap();
    let mut f = gen.gen.as_mut().f.take().unwrap();

    f(&mut gen, start);

    gen.gen.co.as_mut().state = GenState::Complete;

    set_ctx(&gen.gen.ctx as *const _)
}

impl<'a, A, B> Drop for Gen<'a, A, B> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.gen.co.as_ptr());
        }
    }
}
