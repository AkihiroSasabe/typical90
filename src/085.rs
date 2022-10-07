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
        k: usize,
    }
    let mut x = k.clone();

    // kを素因数分解する(k = a0^e0 * a1^e1 * a2^e2 * ... * an*en)
    let prime_factorized_list = prime_factorize(x);
    
    let mut yakusu_map = HashMap::new();
    let mut keys = vec![];
    let mut values = vec![];
    for i in 0..prime_factorized_list.len() {
        keys.push(prime_factorized_list[i][0]);
        values.push(prime_factorized_list[i][1]);
    }

    // 約数を列挙する (公式解答のやり方の方がスマートなので、そちらを参考にすると良い)
    saiki(0, &keys, &values, 1, &mut yakusu_map);
    // println!("{:?}", yakusu_map);
    let mut yakusu_list = vec![];
    for (k, v) in &yakusu_map {
        yakusu_list.push(*k);
    }
    yakusu_list.sort();
    yakusu_list.reverse();
    // println!("{:?}", yakusu_list);
    
    let mut answer = 0;
    for i in 0..yakusu_list.len() {
        for j in i..yakusu_list.len() {
            let a = yakusu_list[i];
            let b = yakusu_list[j];
            let c = k / a / b;
            if a * b * c == k && a >= b && b >= c {
                answer += 1;
            }
        }
    }
    println!("{}", answer);


    // k = (x1)**e1 * (x2)**e2 * (x3)**e3
    // 約数の組み合わせは、(e1+1)*(e2+1)*(e3+1)*...
    // ソートする
    // a >= b >= c の関係を常に作っておく
    
    // // 84
    // a, b, c = 2**2, 3**1, 7**1
    // 84, 1, 1
    // 42, 2, 1
    // 28, 3, 1
    // 21, 4, 1
    // 21, 2, 2
    // ....
    
    // // 42
    // a, b, c = 2, 3, 7
    // 42, 1, 1
    // 21, 2, 1
    // 14, 3, 1
    // 7, 6, 1
    // 7, 3, 2

    // 42の約数一覧
    // {1, 14, 7, 3, 42, 2, 21, 6}
    
}

fn saiki(current_depth: usize, keys: &Vec<usize>, values: &Vec<usize>, yakusu: usize, yakusu_list: &mut HashMap<usize, usize>) {
    if current_depth == keys.len() {
        yakusu_list.insert(1, 1);
        return
    }

    for i in 0..(values[current_depth]+1) {
        let new_yakusu = yakusu * (keys[current_depth].pow(i as u32));
        yakusu_list.insert(new_yakusu, 1);
        saiki(current_depth+1, keys, values, new_yakusu, yakusu_list);
    }
}

// 素因数分解 (No.75の使いまわし)
fn prime_factorize(mut x: usize) -> Vec<Vec<usize>> {
    // let root_x = (x as f64).sqrt() as usize;
    let mut prime_num_list = vec![];
    let mut i = 1;
    while i * i <= x {
    // for i in 2..(root_x+1) {
        i += 1;
        let mut exponent = 0;
        while x % i == 0 {
            x /= i;
            exponent += 1;
        }
        if exponent != 0 {
            prime_num_list.push(vec![i, exponent]);
        }
    }
    if x != 1 {
        prime_num_list.push(vec![x, 1]);
    }
    return prime_num_list
}