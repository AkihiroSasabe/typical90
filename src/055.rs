use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;


// 3 * 3 + 1 = 10
// 3 * 3 + 2 = 11

// (10 % 3) * (11 % 3) = 2
// 110 % 3 = 2


// 積の余りは
// p0 = d * x0 + r0
// p1 = d * x1 + r1

// p0*p1 = (d * x0 + r0) * (d * x1 + r1)
//       = d(x0 * x1 * d + x0 * r1 + x1 + r0) + r0*r1
// ここでr0 = p0 % d, r1 = p1 % dより、
// (p0 * p1) % d = (p0 % d) * (p1 % d)
// が成り立つ。
// さらに
// (p0 * p1 * p2) % d = ((p0 * p1 % d) * p2) % d)
// (p0 * p1 * p2) % d = ((p0 * p1) % d) * (p2 % d)
// (p3 % d) * (p2 % d) = ((p3%d) * p2) % d
// 左辺 = r3 * r2
// 右辺 = (r3 * p2) % d = r3 * r2

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }

    let mut count = 0;
    // 以下、combinationを使ってしまうとTLEしてしまう。
    // for conb in a.into_iter().combinations(5) {
    //     // // println!("{:?}", conb);
    //     // if conb.into_iter().fold(1_usize, |result, x| (result * x) % p) == q {
    //     //     count += 1;
    //     // }

    //     // 上記をfoldで使わないで書くと下記のようになる
    //     let mut product = 1;
    //     for i in conb {
    //         product = (product * i) % p;
    //     }
    //     if product % p == q {
    //         count += 1;
    //     }
    // }

    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                for l in k+1..n {
                    for m in l+1..n {
                        let remainder = a[i] * a[j] % p * a[k] % p * a[l] % p * a[m] % p;
                        if remainder == q {
                            count += 1;
                        }
                    }
                }
            }
        }
    }


    println!("{}", count);
}


