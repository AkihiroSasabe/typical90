use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        mut n: usize,
    }
    let prime_num_list = prime_factorize(n.clone());
    // println!("{:?}", prime_num_list);
    
    let mut count = 0;
    for i in 0..prime_num_list.len() {
        count += prime_num_list[i][1];
    }

    let answer = get_times_devisible_by2(count);
    println!("{}", answer);
    
    // 例
    // 48 = 2**4 * 3**1                         指数の合計が5=>2,3に分ける
    // 48 = (2**2) * (2**2 * 3**1)              最大指数3を3=>1,2に分ける
    // 48 = 2**1 * 2**1 * 2**1 * 3**1 * 2**1    最大指数2を2=>1,1に分ける

}


// 素因数分解
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

fn get_times_devisible_by2(mut x: usize) -> usize {
    // 例
    // 101 を均等になるように分配するとき、何回2で割る必要があるか?
    // 101     101/2 +101%2 = 51
    // 50 51   51/2 + 51%2 = 26
    // 25 26   26/2 + 26%2 = 13
    // 13 13   13/2 + 13%2 = 7
    // 6 7     7/2 + 7%2 = 4
    // 3 4     4/2 + 4%2 = 2
    // 2 2     2/2 + 2%2 = 1
    // 1 1     1/2 == 0なので終了
    let mut count = 0;
    while x / 2 != 0{
        x = x / 2 + x % 2;
        count += 1;
    }
    return count;
}