use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];
    let mut w = vec![];
    for i in 0..q {
        input! {
            x_i: usize,
            y_i: usize,
            z_i: usize,
            w_i: usize,
        }
        x.push(x_i - 1);
        y.push(y_i - 1);
        z.push(z_i - 1);
        w.push(w_i);
    }

    let MODULO = 1_000_000_007;

    // q個のクエリ
    let mut w_bin = vec![];
    for i in 0..q {
        let w_bin_i = decimal_to_binary(w[i]);
        // println!("{:?}", w_bin_i);
        w_bin.push(w_bin_i);
    }

    // 60桁を1桁ずつ独立に条件を満たすか判定
    let mut ans: usize = 1;
    for i in 0..60 {

        // n個の要素を持つ数列の、i桁目の数字の組み合わせは、2^n通り。この内、何個が条件を満たしているか?
        let mut ans_per_digit: usize = 0;
        for bit in 0..(1 << n) {
            // 全てのクエリの条件を満たすか?
            let mut flag = true;
            for q_i in 0..q {
                // Ax, Ay, Azのi桁目の値
                let x_num: usize;
                let y_num: usize;
                let z_num: usize;
                match bit & (1 << x[q_i]) != 0 {
                    true => x_num = 1,
                    false => x_num = 0,
                }
                match bit & (1 << y[q_i]) != 0 {
                    true => y_num = 1,
                    false => y_num = 0,
                }
                match bit & (1 << z[q_i]) != 0 {
                    true => z_num = 1,
                    false => z_num = 0,
                }

                // let mut an_pattern: usize = bit.clone();
                // let an_pattern = ten_to_base(an_pattern, 2);
                // let x_pattern = ten_to_base(1 << x[q_i], 2);
                // let y_pattern = ten_to_base(1 << y[q_i], 2);
                // let z_pattern = ten_to_base(1 << z[q_i], 2);
                // println!("Query: {}, AnPattern: {}, x:{}, y:{}, z:{}, w:{}", q_i, an_pattern, x_pattern, y_pattern, z_pattern, w_bin[q_i][i]);
                
                if w_bin[q_i][i] != (x_num | y_num | z_num) {
                    // println!("Miss Match !!");
                    flag = false;
                    break
                }
            }
            if flag {
                let mut unko: usize = bit.clone();
                let tinko = ten_to_base(unko, 2);
                // println!("OK!! keta:{} bit:{}, bit_2bin:{}", i, bit, tinko);
                ans_per_digit += 1;
                // return;
            }
        ans_per_digit = ans_per_digit % MODULO;
        }
        // println!("{} {}", ans, ans_per_digit);
        ans = (ans * (ans_per_digit % MODULO)) % MODULO;
    }
    println!("{}", ans);

}



// 10進数の数字xを、2進数法で返す。
fn decimal_to_binary(mut x: usize) -> Vec<usize> {
    // 2^61 - 1の数字を格納出来る。
    let MAX_DIGIT = 61;
    let mut answer = vec![0; MAX_DIGIT];

    let mut i = 0;
    // answer[0] = x % 2;
    // answer[1] = (x / 2) % 2;
    // answer[2] = (x / 2/ 2) % 2;

    while x!=0 {
        answer[i] = x % 2;
        x /= 2;
        i += 1;
    }
    return answer
}


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