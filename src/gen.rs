use std::any::Any;
use std::marker::PhantomPinned;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::pin::Pin;
use std::ptr;
use std::ptr::NonNull;

#[link(name = "asm", kind = "static")]
extern "C" {
    fn switch_ctx(old: *mut Ctx, new: *const Ctx);
    fn set_ctx(new: *const Ctx) -> !;
}

const DEFAULT_STACK_SIZE: usize = 1024 * 1024;

#[cfg(not(target_os = "windows"))]
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
    Ready,
}

impl Default for GenState {
    fn default() -> Self {
        GenState::Yield
    }
}

#[derive(Copy, Clone, Debug)]
struct Dropping;
unsafe impl Send for Dropping {}
unsafe impl Sync for Dropping {}

type GenCallback<'a, Send, Recv> = Box<dyn for<'g> FnOnce(Pin<&'g mut Gen<'a, Recv, Send>>, Send) + 'a>;

pub struct Gen<'a, Send, Recv> {
    state:  GenState,
    ctx:    Ctx,
    stack:  Option<Vec<u8>>,
    send:   Option<Send>,
    dual:   Option<NonNull<Gen<'a, Recv, Send>>>,
    cb:     Option<GenCallback<'a, Send, Recv>>,
    panic:  Option<Box<dyn Any + std::marker::Send + 'static>>,
    _pin:   PhantomPinned,
}

impl<Send, Recv> Default for Gen<'_, Send, Recv> {
    fn default() -> Self {
        Self {
            state: Default::default(),
            ctx: Default::default(),
            stack: None,
            send: None,
            dual: None,
            cb: None,
            panic: None,
            _pin: PhantomPinned,
        }
    }
}

impl<'a, Send, Recv> Gen<'a, Send, Recv> {
    pub fn new<F>(f: F) -> Pin<Box<Self>>
    where
        F: for<'g> FnOnce(Pin<&'g mut Gen<'a, Recv, Send>>, Send) + 'a,
    {
        let mut stack = vec![0; DEFAULT_STACK_SIZE];
        let stack_ptr = stack.as_mut_ptr();
        let stack_size = stack.len();
        let (mut gen, dual_gen) = dual_gen(Box::new(f), stack);
        unsafe {
            init_ctx(
                &mut gen.as_mut().get_unchecked_mut().ctx,
                dual_gen,
                stack_ptr,
                stack_size,
            );
        }

        gen
    }

    pub fn resume(this: &mut Pin<&mut Self>, x: Send) -> Option<Recv> {
        unsafe {
            let gen = this.as_mut().get_unchecked_mut();
            let dual_gen = gen.dual.as_mut().unwrap().as_mut();
            match gen.state {
                GenState::Complete => None,
                GenState::Yield | GenState::Ready => {
                    gen.send.replace(x);
                    switch_ctx(&mut dual_gen.ctx, &gen.ctx);
                    dispatch_panic(gen.panic.take());
                    dual_gen.send.take()
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
unsafe fn init_ctx<Send, Recv>(
    ctx:        &mut Ctx,
    dual_gen:   Pin<Box<Gen<Recv, Send>>>,
    stack_ptr:  *mut u8,
    stack_size: usize,
) {
    ctx.gen_ptr = Box::into_raw(Pin::into_inner_unchecked(dual_gen)) as u64;
    ptr::write(
        stack_ptr.add(stack_size - 32) as *mut u64,
        bootstrap::<Send, Recv> as usize as u64,
    );
    ctx.rsp         = stack_ptr.add(stack_size - 32) as u64;
    ctx.stack_start = stack_ptr.add(stack_size) as u64
}

#[cfg(not(target_os = "windows"))]
unsafe fn init_ctx<Send, Recv>(
    ctx: &mut Ctx,
    dual_gen: Pin<Box<Gen<Recv, Send>>>,
    stack_ptr: *mut u8,
    stack_size: usize,
) {
    ctx.gen_ptr = Box::into_raw(Pin::into_inner_unchecked(dual_gen)) as u64;
    ptr::write(
        stack_ptr.add(stack_size - 32) as *mut u64,
        bootstrap::<Send, Recv> as usize as u64,
    );
    ctx.rsp = stack_ptr.add(stack_size - 32) as u64;
}

fn dual_gen<Send, Recv>(
    cb:     GenCallback<Send, Recv>,
    stack:  Vec<u8>,
) -> (Pin<Box<Gen<Send, Recv>>>, Pin<Box<Gen<Recv, Send>>>) {
    let mut gen = Box::pin(Gen::default());
    let mut dual_gen = Box::pin(Gen::default());

    unsafe {
        let gen_ref = gen.as_mut().get_unchecked_mut();
        let dual_gen_ref = dual_gen.as_mut().get_unchecked_mut();
        gen_ref.dual        = Some(NonNull::from(&*dual_gen_ref));
        gen_ref.stack       = Some(stack);
        gen_ref.cb          = Some(cb);
        gen_ref.state       = GenState::Ready;
        dual_gen_ref.dual   = Some(NonNull::from(&*gen_ref));
    }

    (gen, dual_gen)
}

fn dispatch_panic(panic: Option<Box<dyn Any + Send + 'static>>) {
    if let Some(err) = panic {
        resume_unwind(err);
    }
}

unsafe fn bootstrap<Send, Recv>(dual_gen_raw: *mut Gen<Recv, Send>) -> ! {
    let gen = (*dual_gen_raw).dual.as_mut().take().unwrap().as_mut();
    gen.state = GenState::Yield;

    gen.panic = catch_unwind(AssertUnwindSafe(|| {
        let start = gen.send.take().unwrap();
        let cb = gen.cb.take().unwrap();
        let dual_gen = Pin::new_unchecked(dual_gen_raw.as_mut().unwrap());
        cb(dual_gen, start);
    }))
    .err()
    .filter(|x| !x.is::<Dropping>());

    gen.state = GenState::Complete;
    set_ctx(&(*dual_gen_raw).ctx)
}

impl<Send, Recv> Drop for Gen<'_, Send, Recv> {
    fn drop(&mut self) {
        unsafe {
            if let Some(dual_gen) = self.dual.take() {
                let mut dual_gen = Box::from_raw(dual_gen.as_ptr());
                dual_gen.dual = None;
                if let GenState::Yield = self.state {
                    dual_gen.panic = Some(Box::new(Dropping));
                    switch_ctx(&mut dual_gen.ctx, &self.ctx);
                }
            }
        }
    }
}
