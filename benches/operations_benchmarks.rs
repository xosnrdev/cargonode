use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::tempdir;

use cargonode::core::package::{PackageOptions, WorkspaceConfig};
use cargonode::ops::{init::init, new::create_package};

fn package_creation_benchmark(c: &mut Criterion) {
    c.bench_function("create_binary_package", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.set_bin(true);
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });

    c.bench_function("create_library_package", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.set_lib(true);
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });

    c.bench_function("create_typescript_package", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.set_typescript(true);
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });
}

fn typescript_package_benchmark(c: &mut Criterion) {
    c.bench_function("create_typescript_binary", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.set_bin(true).set_typescript(true);
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });

    c.bench_function("create_typescript_library", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.set_lib(true).set_typescript(true);
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });
}

fn workspace_benchmark(c: &mut Criterion) {
    c.bench_function("create_workspace", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.workspace = true;
                opts.workspace_config = Some(WorkspaceConfig {
                    patterns: vec!["packages/*".to_string()],
                    inherit_scripts: true,
                    hoist_dependencies: true,
                });
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });
}

fn package_initialization_benchmark(c: &mut Criterion) {
    c.bench_function("init_package", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let opts = PackageOptions::new(temp.path());
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(init(&opts)).unwrap();
                black_box(temp)
            },
        )
    });
}

fn performance_regression_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("regression");
    group
        .significance_level(0.01)
        .sample_size(100)
        .noise_threshold(0.05); // 5% noise threshold

    // Package creation regression test
    group.bench_function("package_creation_regression", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.set_bin(true);
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });

    // Workspace creation regression test
    group.bench_function("workspace_creation_regression", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.workspace = true;
                opts.workspace_config = Some(WorkspaceConfig {
                    patterns: vec!["packages/*".to_string()],
                    inherit_scripts: true,
                    hoist_dependencies: true,
                });
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });

    // TypeScript package regression test
    group.bench_function("typescript_creation_regression", |b| {
        b.iter_with_setup(
            || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path());
                opts.set_typescript(true);
                (temp, opts)
            },
            |(temp, opts)| {
                black_box(create_package(&opts)).unwrap();
                black_box(temp)
            },
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    package_creation_benchmark,
    typescript_package_benchmark,
    workspace_benchmark,
    package_initialization_benchmark,
    performance_regression_benchmark
);
criterion_main!(benches);
