#![feature(asm)]
#![feature(naked_functions)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod c2rust;
mod gen;
use crate::gen::Gen;

fn main() {
    let mut simple_gen = Gen::new(|gen, _| {
        println!("in gen 1");
        gen.resume(());
        println!("in gen 2");
    });
    simple_gen.resume(());
    simple_gen.resume(());

    let mut fib_gen = Gen::new(|gen, _| {
        let mut an: u8 = 0;
        let mut an_1: u8 = 1;
        while let Some(an_2) = an.checked_add(an_1) {
            gen.resume(an);
            an = an_1;
            an_1 = an_2;
        }
        gen.resume(an_1);
    });

    while let Some(fib) = fib_gen.resume(()) {
        println!("{}", fib);
    }


}