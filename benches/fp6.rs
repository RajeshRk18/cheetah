// Copyright (c) 2021-2022 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rand_core::OsRng;

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

extern crate cheetah;

use cheetah::Fp6;
use group::ff::Field;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = OsRng;

    c.bench_function("fp6 add", |bench| {
        let x = Fp6::random(&mut rng);
        let y = Fp6::random(&mut rng);
        bench.iter(|| black_box(x) + black_box(y))
    });

    c.bench_function("fp6 sub", |bench| {
        let x = Fp6::random(&mut rng);
        let y = Fp6::random(&mut rng);
        bench.iter(|| black_box(x) - black_box(y))
    });

    c.bench_function("fp6 double", |bench| {
        let x = Fp6::random(&mut rng);
        bench.iter(|| black_box(x).double())
    });

    c.bench_function("fp6 mul", |bench| {
        let x = Fp6::random(&mut rng);
        let y = Fp6::random(&mut rng);
        bench.iter(|| black_box(x) * black_box(y))
    });

    c.bench_function("fp6 square", |bench| {
        let x = Fp6::random(&mut rng);
        bench.iter(|| black_box(x).square())
    });

    c.bench_function("fp6 square_from_mul", |bench| {
        let x = Fp6::random(&mut rng);
        bench.iter(|| black_box(x) * black_box(x))
    });

    c.bench_function("fp6 sqrt", |bench| {
        let x = Fp6::random(&mut rng).square();
        bench.iter(|| Fp6::sqrt(black_box(&x)))
    });

    c.bench_function("fp6 exp", |bench| {
        let x = Fp6::random(&mut rng);
        let y = Fp6::random(&mut rng).output_reduced_limbs();
        bench.iter(|| Fp6::exp(black_box(x), black_box(&y)))
    });

    c.bench_function("fp6 invert", |bench| {
        let x = Fp6::random(&mut rng);
        bench.iter(|| Fp6::invert(black_box(&x)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark);
criterion_main!(benches);
