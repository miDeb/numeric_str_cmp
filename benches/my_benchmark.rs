use std::cmp::Ordering;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_str_cmp::numeric_str_cmp;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cmp two digits", |b| {
        b.iter(|| {
            assert_eq!(
                numeric_str_cmp(black_box("20"), black_box("10")),
                Ordering::Greater
            )
        })
    });
    c.bench_function("cmp two digits parse", |b| {
        b.iter(|| {
            let a = black_box("20").parse::<f64>().unwrap();
            let b = black_box("10").parse().unwrap();
            let ord = if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            };
            assert_eq!(ord, Ordering::Greater);
        })
    });

    c.bench_function("cmp 100 digits", |b| {
        let s1 = black_box("20".repeat(50));
        let s2 = black_box("50".repeat(50));
        b.iter(|| assert_eq!(numeric_str_cmp(&s1, &s2), Ordering::Less));
    });
    c.bench_function("cmp 100 digits parse", |b| {
        let s1 = black_box("20".repeat(50));
        let s2 = black_box("50".repeat(50));
        b.iter(|| {
            let a = s1.parse::<f64>().unwrap();
            let b = s2.parse().unwrap();
            let ord = if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            };
            assert_eq!(ord, Ordering::Less);
        })
    });

    c.bench_function("cmp 100,000 digits", |b| {
        let s1 = black_box("20".repeat(50000));
        let s2 = black_box("50".repeat(50000));
        b.iter(|| assert_eq!(numeric_str_cmp(&s1, &s2), Ordering::Less));
    });
    // parsing a 100,000 digit number is much faster because they are greater than f64::MAX and parse to f64::INFINITY.
    // comparisons are not accurate.
    c.bench_function("cmp 100,000 digits parse", |b| {
        let s1 = black_box("20".repeat(50000));
        let s2 = black_box("50".repeat(50000));
        b.iter(|| {
            let a = s1.parse::<f64>().unwrap();
            let b = s2.parse().unwrap();
            assert!(a == f64::INFINITY);
            assert!(b == f64::INFINITY);
            let ord = if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            };
            assert_eq!(ord, Ordering::Equal)
        })
    });

    c.bench_function("cmp 100 digits after decimal pt", |b| {
        let s1 = black_box({
            let mut string = "0.".to_string();
            string.push_str(&"20".repeat(50));
            string
        });
        let s2 = black_box({
            let mut string = "0.".to_string();
            string.push_str(&"50".repeat(50));
            string
        });
        b.iter(|| assert_eq!(numeric_str_cmp(&s1, &s2), Ordering::Less));
    });
    c.bench_function("cmp 100 digits after decimal pt parse", |b| {
        let s1 = black_box({
            let mut string = "0.".to_string();
            string.push_str(&"20".repeat(50));
            string
        });
        let s2 = black_box({
            let mut string = "0.".to_string();
            string.push_str(&"50".repeat(50));
            string
        });
        b.iter(|| {
            let a = s1.parse::<f64>().unwrap();
            let b = s2.parse().unwrap();
            let ord = if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            };
            assert_eq!(ord, Ordering::Less);
        })
    });

    c.bench_function("cmp 100 digits before and after decimal pt", |b| {
        let s1 = black_box({
            let mut string = "0.".to_string();
            string.push_str(&"20".repeat(50));
            string
        });
        let s2 = black_box({
            let mut string = "0.".to_string();
            string.push_str(&"50".repeat(50));
            string
        });
        b.iter(|| assert_eq!(numeric_str_cmp(&s1, &s2), Ordering::Less));
    });
    c.bench_function("cmp 100 digits before and after decimal pt parse", |b| {
        let s1 = black_box({
            let mut string = "20".repeat(50);
            string.push_str(&"20".repeat(50));
            string
        });
        let s2 = black_box({
            let mut string = "50".repeat(50);
            string.push_str(&"50".repeat(50));
            string
        });
        b.iter(|| {
            let a = s1.parse::<f64>().unwrap();
            let b = s2.parse().unwrap();
            let ord = if a > b {
                Ordering::Greater
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Equal
            };
            assert_eq!(ord, Ordering::Less);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
