use gen_rs::Gen;

fn main() {
    let mut fib_gen = Gen::new(|gen, _| {
        let mut an: u64 = 0;
        let mut an_1: u64 = 1;
        while let Some(an_2) = an.checked_add(an_1) {
            gen.resume(an);
            an = an_1;
            an_1 = an_2;
        }
        gen.resume(an_1);
    });

    let mut i: usize = 0;
    while let Some(fib) = fib_gen.resume(()) {
        println!("fib({}) = {}", i, fib);
        i += 1;
    }
}