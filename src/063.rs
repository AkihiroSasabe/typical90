use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h]
    }
    // 1 << x は、2^xであり、2進数で考えると001をx桁左にずらすことと同義。
    // println!("{}", 1 << 0); // 001 = 1
    // println!("{}", 1 << 1); // 010 = 2
    // println!("{}", 1 << 2); // 100 = 4


    let mut answer = 0;
    for bit in 0..(1 << h) {
        let mut y_list = vec![];
        for digit in 0..h {
            if bit & 1 << digit != 0 {
                y_list.push(digit);
            }
        }

        let mut permitted = vec![0; h*w+1];
        for x in 0..w {
            let mut count = 0;
            let mut pre = 0;
            for y in y_list.iter() {
                if count == 0 {
                    pre = p[*y][x];
                    count += 1;
                    continue
                }
                if pre != p[*y][x] {
                    break
                }
                count += 1;
            }
            if count == y_list.len() && y_list.len() != 0 {
                permitted[pre] += y_list.len();
            }
        }
        permitted.sort();
        let before = answer;
        answer = max(answer, permitted[h*w]);
        // if answer != before {
        //     println!("updated!");
        //     println!("y_list: {:?}", y_list);
        // }
        
    }
    println!("{}", answer);
}