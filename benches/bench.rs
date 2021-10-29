#![feature(test)]

extern crate test;

use test::Bencher;
use squirrel3_rs::sq3;
use rand::prelude::*;
use wyhash::WyRng;

#[bench]
fn shuffle_wyhash(b: &mut Bencher) {
    let mut rng = WyRng::from_rng(thread_rng()).unwrap();
    let mut x = (0..100).collect::<Vec<usize>>();
    b.iter(|| {
        x.shuffle(&mut rng);
        x[0]
    })
}

#[bench]
fn shuffle_fastrand(b: &mut Bencher) {
    let rng = fastrand::Rng::new();
    let mut x = (0..100).collect::<Vec<usize>>();
    b.iter(|| {
        rng.shuffle(&mut x);
        x[0]
    })
}

#[bench]
fn shuffle_sq3(b: &mut Bencher) {
    let rng = sq3::Rng::new(sq3::u32(u32::MAX));
    let mut x = (0..100).collect::<Vec<usize>>();
    b.iter(|| {
        rng.shuffle(&mut x);
        x[0]
    })
}

#[bench]
fn u32_wyhash(b: &mut Bencher) {
    let mut rng = WyRng::from_rng(thread_rng()).unwrap();
    b.iter(|| {
        let mut sum = 0u32;
        for _ in 0..10_000 {
            sum = sum.wrapping_add(rng.gen::<u32>());
        }
        sum
    })
}

#[bench]
fn u32_fastrand(b: &mut Bencher) {
    let rng = fastrand::Rng::new();
    b.iter(|| {
        let mut sum = 0u32;
        for _ in 0..10_000 {
            sum = sum.wrapping_add(rng.u32(..));
        }
        sum
    })
}

#[bench]
fn u32_sq3(b: &mut Bencher) {
    let rng = sq3::Rng::new(sq3::u32(u32::MAX));
    b.iter(|| {
        let mut sum = 0u32;
        for _ in 0..10_000 {
            sum = sum.wrapping_add(rng.u32(u32::MAX));
        }
        sum
    })
}