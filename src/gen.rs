use std::any::Any;
use std::marker::{PhantomData, PhantomPinned};
use std::panic::{catch_unwind, resume_unwind, RefUnwindSafe};
use std::ptr::NonNull;
use std::ptr;
use std::mem::ManuallyDrop;

#[link(name = "asm", kind = "static")]
extern "C" {
    fn switch_ctx(old: *mut Ctx, new: *const Ctx);
    fn set_ctx(new: *const Ctx) -> !;
}

pub struct Gen<'a, A, B> {
    gen: NonNull<UnwrapGen<'a, A, B>>,
    _marker: PhantomData<Box<UnwrapGen<'a, A, B>>>,
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

#[derive(Copy, Clone, Debug)]
struct Dropping;
unsafe impl Send for Dropping {}
unsafe impl Sync for Dropping {}

struct UnwrapGen<'a, A, B> {
    state: GenState,
    ctx: Ctx,
    _stack: Option<Vec<u8>>,
    send: Option<A>,
    co: NonNull<UnwrapGen<'a, B, A>>,
    f: Option<Box<dyn for<'g> FnMut(&'g mut Gen<A, B>, B) + 'a>>,
    panic: Option<Box<dyn Any + Send + 'static>>,
    _pin: PhantomPinned,
}

impl<'a, A, B> Gen<'a, A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: for<'g> FnMut(&'g mut Gen<B, A>, A) + 'a,
    {
        let mut stack = vec![0; DEFAULT_STACK_SIZE];
        let stack_ptr = stack.as_mut_ptr();
        let stack_size = stack.len();
        let (mut gen, co_gen) = dual_gen(Box::new(f), stack);
        unsafe {
            init_ctx(&mut gen.as_mut().ctx, co_gen, stack_ptr, stack_size);
        }

        Gen {
            gen,
            _marker: PhantomData,
        }
    }

    pub fn resume(&mut self, x: A) -> Option<B> {
        unsafe {
            let gen_raw = self.gen.as_mut();
            match gen_raw.state {
                GenState::Complete => None,
                GenState::Yield | GenState::Ready => {
                    gen_raw.send.replace(x);
                    switch_ctx(&mut gen_raw.co.as_mut().ctx, &gen_raw.ctx);
                    dispatch_panic(gen_raw.panic.take());
                    gen_raw.co.as_mut().send.take()
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
unsafe fn init_ctx<A, B>(
    ctx: &mut Ctx,
    co_gen: NonNull<UnwrapGen<B, A>>,
    stack_ptr: *mut u8,
    stack_size: usize,
) {
    ctx.gen_ptr = co_gen.as_ptr() as u64;
    ptr::write(
        stack_ptr.add(stack_size - 32) as *mut u64,
        bootstrap::<A, B> as usize as u64,
    );
    ctx.rsp = stack_ptr.add(stack_size - 32) as u64;
    ctx.stack_start = stack_ptr.offset(stack_size as isize) as u64
}

#[cfg(not(target_os = "windows"))]
unsafe fn init_ctx<A, B>(
    ctx: &mut Ctx,
    co_gen: NonNull<UnwrapGen<B, A>>,
    stack_ptr: *mut u8,
    stack_size: usize,
) {
    ctx.gen_ptr = co_gen.as_ptr() as u64;
    ptr::write(
        stack_ptr.add(stack_size - 32) as *mut u64,
        bootstrap::<A, B> as usize as u64,
    );
    ctx.rsp = stack_ptr.add(stack_size - 32) as u64;
}

fn dual_gen<'a, A, B>(
    f: Box<dyn for<'g> FnMut(&'g mut Gen<B, A>, A) + 'a>,
    stack: Vec<u8>,
) -> (NonNull<UnwrapGen<A, B>>, NonNull<UnwrapGen<B, A>>) {
    let mut gen = Box::new(UnwrapGen {
        state: GenState::Yield,
        ctx: Ctx::default(),
        _stack: Some(stack),
        send: None,
        co: NonNull::dangling(),
        f: None,
        panic: None,
        _pin: PhantomPinned,
    });

    let co_gen = Box::new(UnwrapGen {
        state: GenState::Ready,
        ctx: Ctx::default(),
        _stack: None,
        send: None,
        co: NonNull::from(&*gen),
        f: Some(f),
        panic: None,
        _pin: PhantomPinned,
    });
    gen.co = NonNull::from(&*co_gen);

    (
        NonNull::new(Box::into_raw(gen)).unwrap(),
        NonNull::new(Box::into_raw(co_gen)).unwrap(),
    )
}

unsafe fn dispatch_panic(panic: Option<Box<dyn Any + Send + 'static>>) {
    if let Some(err) = panic {
        resume_unwind(err);
    }
}

impl<A, B> RefUnwindSafe for UnwrapGen<'_, A, B> {}

unsafe fn bootstrap<A, B>(gen_raw: *mut UnwrapGen<B, A>) {
    (*gen_raw).co.as_mut().panic = catch_unwind( || {
        let start = (*gen_raw).co.as_mut().send.take().unwrap();
        let mut f = (*gen_raw).f.take().unwrap();
        let mut gen = ManuallyDrop::new(Gen {
            gen: NonNull::new(gen_raw).unwrap(),
            _marker: PhantomData,
        });
        f(&mut gen, start);
    }).err().filter(|x| !x.is::<Dropping>());

    (*gen_raw).co.as_mut().state = GenState::Complete;
    set_ctx(&(*gen_raw).ctx);
}

impl<'a, A, B> Drop for Gen<'a, A, B> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.gen.as_ptr());
            let mut co_gen = Box::from_raw(self.gen.as_mut().co.as_ptr());
            match self.gen.as_ref().state {
                GenState::Yield => {
                    co_gen.panic = Some(Box::new(Dropping));
                    switch_ctx(&mut co_gen.ctx, &mut self.gen.as_mut().ctx);
                },
                _ => {}
            }
        }
    }
}

