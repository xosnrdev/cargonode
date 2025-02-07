use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cargonode::util::platform::{self};

fn platform_detection_benchmark(c: &mut Criterion) {
    c.bench_function("platform_detection", |b| {
        b.iter(|| {
            black_box(platform::is_windows());
            black_box(platform::is_unix_like());
            black_box(platform::get_platform_name());
        })
    });
}

fn path_handling_benchmark(c: &mut Criterion) {
    let mixed_path = "path\\to/file/with\\mixed/separators";
    let mixed_content = "line1\r\nline2\nline3\r\n";

    c.bench_function("normalize_path", |b| {
        b.iter(|| platform::normalize_path(black_box(mixed_path)))
    });

    c.bench_function("normalize_line_endings", |b| {
        b.iter(|| platform::normalize_line_endings(black_box(mixed_content)))
    });
}

fn filesystem_benchmark(c: &mut Criterion) {
    c.bench_function("get_home_dir", |b| {
        b.iter(|| black_box(platform::get_home_dir()))
    });

    c.bench_function("get_temp_dir", |b| {
        b.iter(|| black_box(platform::get_temp_dir()))
    });
}

criterion_group!(
    benches,
    platform_detection_benchmark,
    path_handling_benchmark,
    filesystem_benchmark
);
criterion_main!(benches);
