use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q]
    }

    let r = l / 2.0;

    for now in e {
        let y_now = - r * (now / t * 2.0 * PI).sin();
        let z_now = r - r * (now / t * 2.0 * PI).cos();
        // println!("y_now, z_now: {} {}", y_now, z_now);
        // (y/x).atan()を使うとき
        // let answer: f64 = (z_now / (x * x + (y_now - y)*(y_now - y)).sqrt()).atan() / PI * 180.0;
        // y.atan2(x)を使う時: https://doc.rust-lang.org/std/primitive.f64.html#method.atan2
        let answer: f64 = (z_now).atan2((x * x + (y_now - y)*(y_now - y)).sqrt()) / PI * 180.0;
        // 小数点12桁まで表示 (ここまで表示する必要はない。問題の例に合わせているだけ)
        println!("{:.12}", answer);
    }
}