use rand::thread_rng;

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

extern crate cheetah;

use cheetah::Scalar;
use cheetah::{AffinePoint, ProjectivePoint};
use group::ff::Field;
use group::Group;

static BATCH_SIZES: [u32; 5] = [1, 2, 4, 8, 16];

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("curve add projective", |bench| {
        let mut rng = thread_rng();
        let x = ProjectivePoint::random(&mut rng);
        let y = ProjectivePoint::random(&mut rng);
        bench.iter(|| black_box(x) + black_box(y))
    });

    c.bench_function("curve add mixed", |bench| {
        let mut rng = thread_rng();
        let x = AffinePoint::random(&mut rng);
        let y = ProjectivePoint::random(&mut rng);
        bench.iter(|| black_box(x) + black_box(y))
    });

    c.bench_function("curve sub projective", |bench| {
        let mut rng = thread_rng();
        let x = ProjectivePoint::random(&mut rng);
        let y = ProjectivePoint::random(&mut rng);
        bench.iter(|| black_box(x) - black_box(y))
    });

    c.bench_function("curve double projective", |bench| {
        let mut rng = thread_rng();
        let p = ProjectivePoint::random(&mut rng);
        bench.iter(|| black_box(p).double())
    });

    c.bench_function("curve scalar mul affine", |bench| {
        let mut rng = thread_rng();
        let p = AffinePoint::random(&mut rng);
        let pow = Scalar::random(&mut rng).to_bytes();
        bench.iter(|| AffinePoint::multiply(black_box(&p), black_box(&pow)))
    });

    c.bench_function("curve double scalar mul affine", |bench| {
        let mut rng = thread_rng();
        let p1 = AffinePoint::random(&mut rng);
        let p2 = AffinePoint::random(&mut rng);
        let pow1 = Scalar::random(&mut rng).to_bytes();
        let pow2 = Scalar::random(&mut rng).to_bytes();
        bench.iter(|| {
            AffinePoint::multiply_double(
                black_box(&p1),
                black_box(&p2),
                black_box(&pow1),
                black_box(&pow2),
            )
        })
    });

    c.bench_function("curve scalar mul projective", |bench| {
        let mut rng = thread_rng();
        let p = ProjectivePoint::random(&mut rng);
        let pow = Scalar::random(&mut rng).to_bytes();
        bench.iter(|| ProjectivePoint::multiply(black_box(&p), black_box(&pow)))
    });

    c.bench_function("curve double scalar mul projective", |bench| {
        let mut rng = thread_rng();
        let p1 = ProjectivePoint::random(&mut rng);
        let p2 = ProjectivePoint::random(&mut rng);
        let pow1 = Scalar::random(&mut rng).to_bytes();
        let pow2 = Scalar::random(&mut rng).to_bytes();
        bench.iter(|| {
            ProjectivePoint::multiply_double(
                black_box(&p1),
                black_box(&p2),
                black_box(&pow1),
                black_box(&pow2),
            )
        })
    });

    let batch_str = "curve batch normalize ".to_string();
    for &batch_size in BATCH_SIZES.iter() {
        let name = batch_str.clone() + &batch_size.to_string();
        c.bench_function(&name, |bench| {
            let mut rng = thread_rng();
            let projective_points = vec![ProjectivePoint::random(&mut rng); batch_size as usize];
            let mut affine_points = vec![AffinePoint::identity(); batch_size as usize];
            bench.iter(|| {
                ProjectivePoint::batch_normalize(
                    black_box(&projective_points),
                    black_box(&mut affine_points),
                )
            })
        });
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark);
criterion_main!(benches);
