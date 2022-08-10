use criterion::{black_box, criterion_group, criterion_main, Criterion};
use warp_subdomain::with_subdomain;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark", |b| {
        b.iter(|| async {
            let filter = with_subdomain();
            warp::test::request()
                .path("/")
                .header("host", black_box("super-alloy.api.cilen.com:3000"))
                .filter(&filter)
                .await
                .unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
