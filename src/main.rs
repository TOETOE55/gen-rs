use gen_rs::Gen;

fn main() {
    let fib_gen = Gen::new(|gen, _| {
        let mut an: u64 = 0;
        let mut an_1: u64 = 1;
        while let Some(an_2) = an.checked_add(an_1) {
            gen.resume(an);
            an = an_1;
            an_1 = an_2;
        }
        gen.resume(an_1);
    });


    for (i, ai) in fib_gen.enumerate() {
        println!("fib({}) = {}", i, ai);
    }
}
