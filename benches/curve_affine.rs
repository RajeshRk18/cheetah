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

use cheetah::AffinePoint;
use cheetah::BasePointTable;
use cheetah::Scalar;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = OsRng;
    let p = AffinePoint::random(&mut rng);
    let p2 = AffinePoint::random(&mut rng);
    let pow = Scalar::random(&mut rng).to_bytes();
    let pow2 = Scalar::random(&mut rng).to_bytes();

    let uncompressed_encoding = p.to_uncompressed();
    let compressed_encoding = p.to_compressed();

    c.bench_function("Affine scalar multiplication (variable base)", |bench| {
        bench.iter(|| AffinePoint::multiply(black_box(&p), black_box(&pow)))
    });

    c.bench_function(
        "Affine scalar multiplication (variable base) - variable time",
        |bench| bench.iter(|| AffinePoint::multiply_vartime(black_box(&p), black_box(&pow))),
    );

    c.bench_function(
        "Affine double scalar multiplication (variable base)",
        |bench| {
            bench.iter(|| {
                AffinePoint::multiply_double(
                    black_box(&p),
                    black_box(&p2),
                    black_box(&pow),
                    black_box(&pow2),
                )
            })
        },
    );

    c.bench_function(
        "Affine double scalar multiplication (variable base) - variable time",
        |bench| {
            bench.iter(|| {
                AffinePoint::multiply_double_vartime(
                    black_box(&p),
                    black_box(&p2),
                    black_box(&pow),
                    black_box(&pow2),
                )
            })
        },
    );

    c.bench_function("Affine basepoint table creation", |bench| {
        bench.iter(|| BasePointTable::from(black_box(&p)))
    });

    c.bench_function("Affine uncompressed encoding", |bench| {
        bench.iter(|| AffinePoint::to_uncompressed(black_box(&p)))
    });

    c.bench_function("Affine uncompressed decoding", |bench| {
        bench.iter(|| AffinePoint::from_uncompressed(black_box(&uncompressed_encoding)))
    });

    c.bench_function("Affine compressed encoding", |bench| {
        bench.iter(|| AffinePoint::to_compressed(black_box(&p)))
    });

    c.bench_function("Affine compressed decoding", |bench| {
        bench.iter(|| AffinePoint::from_compressed(black_box(&compressed_encoding)))
    });

    c.bench_function("Affine curve check", |bench| {
        bench.iter(|| AffinePoint::is_on_curve(black_box(&p)))
    });

    c.bench_function("Affine subgroup check", |bench| {
        bench.iter(|| AffinePoint::is_torsion_free(black_box(&p)))
    });

    c.bench_function("Affine points equality", |bench| {
        bench.iter(|| black_box(p) == black_box(p))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark);
criterion_main!(benches);
