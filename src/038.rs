use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;


// テストケースhand_17.txt: 
// 274177 67280421310721
// 67_280_421_310_721 > 6 * 10^13
// usize=u64=2^64-1=: 18,446,744,073,709,551,615 = 1.8.. * 10^19
//                    18_446_744_073_709_551_615

fn main() {
    input! {
        a: usize,
        b: usize
    }
    let mut inf: usize = 1;
    for _ in 0..18 {
        inf *= 10;
    }

    // 3 * 7 < 22
    // 7 < 22/3
    // 7 < 3+1/3
    // 7 - 1/3 < 3

    let gcd_ab = gcd(a,b);
    // a * b / gcd <= 10^18
    // <=> b / gcd <= 10^(18) / a
    // <=> b / gcd <= int(10^(18) / a) (左辺はもともと整数なので。)
    // 18_446_744_073_709_551_615

    // 10 * 2 < 21だった場合
    // 10 < 21 / 2 となるが、
    // 整数型では21/2=10.5=10となってしまうので、
    // 10 <= int(21/2)となる。

    if  b / gcd_ab <= inf / a {
        let lcm = a * (b / gcd_ab);
        println!("{}", lcm);
        
    }
    else  {
        println!("Large");
    }

}

fn gcd(mut x: usize, mut y:usize) -> usize {
    if y <= x {
        let y_pre = y.clone();
        y = x.clone();
        x = y_pre;
    } 
    let amari = y % x;
    if amari == 0 {
        return x
    }
    else {
        y = y % x;
        return gcd(x, y);
    }

}

