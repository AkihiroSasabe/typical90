use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;

// 8進法 -> 10進法
fn base8_to_base10(n: usize) -> usize {
    let mut x_10: usize = 0;

    let n_char: Vec<char> = n.to_string().as_str().chars().rev().collect();
    for i in 0..n_char.len() {
        let digit = n_char[i] as usize - 48;
        x_10 = digit * 8_usize.pow(i as u32)
    }

    return x_10
}

fn base10_to_base9(x_10: usize) -> usize {
    //10進法 -> 9進法
    let mut i: u32 = 0;
    let mut x_9 = 0;
    loop {
        let amari =  x_10 % 9_usize.pow(i + 1) ;
        x_10 = x_10 - amari;
        x_9 += (amari / 9_usize.pow(i)) * 10_usize.pow(i);
        if x_10 == 0 {
            break
        }
        i += 1;
    }
    
    // println!("x_9: {}", x_9);
    return x_9
}


fn main() {
    input! {
        a: usize
    }

    let b = base8_to_base10(a);
    println!("{}", b);
}

// fn main() {
//     input! {
//         n: Chars,
//         k: usize
//     }

//     let mut x_9 = 0;
//     let mut x_10 = 0;

//     // 先頭から1の位が始まるように。
//     let mut n = n.into_iter().rev();
//     for _ in 0..k {
//         // 8進法 -> 10進法
//         for (i, digit_c) in n.enumerate() {
//             let digit_u: usize = digit_c as usize - 48;
//             // println!("digit_u: {}", digit_u);
//             x_10 += digit_u * 8_usize.pow(i as u32);
//         }
//         // println!("x_10: {}", x_10);

        //10進法 -> 9進法
        let mut i = 0;
        loop {
            let amari =  x_10 % 9_usize.pow(i as u32 + 1) ;
            x_10 = x_10 - amari;
            x_9 += (amari / 9_usize.pow(i as u32)) * 10_usize.pow(i as u32);
            if x_10 == 0 {
                break
            }
            i += 1;
        }
        // println!("x_9: {}", x_9);

//         // '8'を'5'に書き写す
//         let x_9_chars: Vec<char> = x_9.to_string().as_str().chars().map(|x: char| if x == '8' {'5'} else {x}).collect();

//         // 先頭から1の位が始まるように。
//         n = x_9_chars.into_iter().rev();

//         // 初期化
//         x_9 = 0;
//         x_10 = 0;
//     }

//     let mut answer = 0;
//     for (i, c) in n.enumerate() {
//         answer += (c as usize - 48) * 10_usize.pow(i as u32);
//     }
//     println!("{}", answer);



//     // ■8進数 -> 10進数
//     // 8進数法で1は、10進数では1
//     // 8進数法で7は、10進数では7
//     // 8進数法で10は、10進数では8
//     // 8進数法で100は、10進数では77_base8 + 1 = 7 * 8 + 7 + 1 = 8 * 8 = 64

//     // ■8進数 -> 10進数
//     // 8進数で1330なものを10進数に変換する。
//     // x_8 = 1330
//     // 8進数の定義より
//     // x_10 = 0*(8**0) + 3 * (8**1) + 3 * (8 **2) + 1*(8**3) = 728
    
//     // ■10進数 -> 8進数
//     // 逆に、これを8進数に変換する。
//     // 1の位: r1 = x_10 % (8 ** 1) = 0
//     //        d1 = r2 / (8 ** 0)
//     // 2の位: r2 = (x_10 - r1) % (8 ** 2) = 24
//     //        d2 = r2 / (8 ** 1) = 3
//     // 3の位: r3 = (x_10 - r1 - r2) % (8 ** 3) = 192
//     //        d3 = r3 / (8 ** 2) = 3
//     // 4の位: r4 = (x_10 - r1 - r2 -r3) % (8 ** 4) = 512
//     //        d4 = r4 / (8 ** 3) = 1
//     // 5の位: r5 = (x_10 - r1 - r2 - r3 - r4) % (8 ** 5) = 0 % (8 ** 5)となるが、
//     // x_10 - r1 - r2 - r3 - r4 == 0になった時点で終了
    
// }

// 10進法のxをn(base)進法に変換
fn ten_to_base(mut x: usize, base: usize) -> usize {
    // ==== How to use ====
    // let answer = ten_to_base(11, 8);
    // println!("{}", answer); // 13
    // let answer = ten_to_base(2021, 9);
    // println!("{}", answer); // 2685   
    // ==== Theory ====
    // The number x can be expressed in n-decimal notation as follows:
    // (where 0 <= ai <= n-1)
    // x   = a0 * (n^0) + a1 * (n^1) * a2 * (n^2) + ... + amax * (n^max)
    // ==== Example ====
    // 11  = 2*(8**0) + 3*(8**1) 
    // a0 = x % 8
    // a1 = (x / 8) % 8
    // a2 = (x / 8^2) % 8 but (x / 8^2) == 0 => max = 1
    // ... 
    let mut answer = 0;
    let mut digit: u32 = 0;
    while x != 0 {
        let amari = x % base;
        answer += amari * 10_usize.pow(digit);
        x /= base;
        digit += 1;
    }
    return answer;
}