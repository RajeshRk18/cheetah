use rand::thread_rng;

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

extern crate cheetah;

use cheetah::Scalar;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    c.bench_function("scalar add", |bench| {
        let x = Scalar::random(&mut rng);
        let y = Scalar::random(&mut rng);
        bench.iter(|| black_box(x) + black_box(y))
    });

    c.bench_function("scalar sub", |bench| {
        let x = Scalar::random(&mut rng);
        let y = Scalar::random(&mut rng);
        bench.iter(|| black_box(x) - black_box(y))
    });

    c.bench_function("scalar mul", |bench| {
        let x = Scalar::random(&mut rng);
        let y = Scalar::random(&mut rng);
        bench.iter(|| black_box(x) * black_box(y))
    });

    c.bench_function("scalar square", |bench| {
        let x = Scalar::random(&mut rng);
        bench.iter(|| black_box(x).square())
    });

    c.bench_function("scalar square_from_mul", |bench| {
        let x = Scalar::random(&mut rng);
        bench.iter(|| black_box(x) * black_box(x))
    });

    c.bench_function("scalar exp", |bench| {
        let x = Scalar::random(&mut rng);
        let y = Scalar::random(&mut rng).to_repr();
        bench.iter(|| Scalar::exp(black_box(x), black_box(&y)))
    });

    c.bench_function("scalar invert", |bench| {
        let x = Scalar::random(&mut rng);
        bench.iter(|| Scalar::invert(black_box(x)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark);
criterion_main!(benches);