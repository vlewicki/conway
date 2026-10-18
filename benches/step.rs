// benches/step.rs
use conway::{Cell, GameOfLife, Grid};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_step(c: &mut Criterion) {
    let mut game = GameOfLife::from_grid(Grid::<Cell>::new(1000, 1000));
    c.bench_function("step 100x100", |b| {
        b.iter(|| {
            game.step();
            black_box(&game);
        })
    });
}

criterion_group!(benches, bench_step);
criterion_main!(benches);
