use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: isize,
        q: usize,
        mut a: [usize; n],
        txy: [[isize; 3]; q]
    }

    // Ti=2になった回数を数える
    let mut count_2: isize = 0;
    for i in 0..q { 
        // println!("count_2: {}, {:?}, txy[i]: {:?})", count_2, a, txy[i]);

        // Rustでは負の余りを正しく計算してくれないので、自分で割られる数に割る数の整数倍を足して正にする必要がある。
        let mut x = (txy[i][1] - 1 - count_2);
        while x < 0 {
            x += n;
        }
        x %= n;

        let mut y = (txy[i][2] - 1 - count_2);
        while y < 0 {
            y += n;
        }
        y %= n;
        match txy[i][0] {
            1 => {  // tyy[i][1]とtyy[i][2]を入れ替える
                    
                    let tmp = a[x as usize];
                    a[x as usize] = a[y  as usize];
                    a[y as usize] = tmp;
                },
            2 => count_2 += 1,
            3 => println!("{}", a[x as usize]),
            _ => ()
        }
    }

}