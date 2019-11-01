use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct GenCtx {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    rip: u64,
}

#[derive(Default)]
struct UnwrapGen<A, B> {
    ctx: Option<NonNull<GenCtx>>,
    co_ctx: Option<NonNull<GenCtx>>,
    on: Option<NonNull<A>>,
    by: Option<NonNull<B>>,
    co: Option<NonNull<UnwrapGen<B, A>>>,

    _by: PhantomData<fn(B)>,
}

struct Gen<A, B>(Box<UnwrapGen<A, B>>);
