use gen_rs::helper::{generator, Resume};


fn main() {
    let fib_gen = generator(|gen, _| {
        let mut re = Resume::new(gen);
        let mut an: u64 = 0;
        let mut an_1: u64 = 1;
        re.resume(an);
        while let Some(an_2) = an.checked_add(an_1) {
            an = an_1;
            an_1 = an_2;
            re.resume(an_1);
        }
    });

    for (i, ai) in fib_gen.enumerate() {
        println!("fib({}) = {}", i, ai);
    }


    /*
        let mut gen = Gen::new(move |co_gen, _| {
            *co_gen = fib_gen;
            // you can't get &mut Gen
        });

        Gen::resume(&mut gen.as_mut(), 3);
    */
    /*
    let mut gen1: Gen<_, ()> = Gen::new(move |co_gen1, _| {
        std::mem::swap(co_gen1, &mut fib_gen);
        // you can't get &mut Gen
    });

    Gen::resume(&mut gen.as_mut(), 3);
    */
}
