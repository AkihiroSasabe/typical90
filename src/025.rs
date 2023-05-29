use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        b: usize
    }
    // 方針:
    // E8氏の解説とほぼ同じ。
    // 1 <= m <= N < 10^11
    // なので愚直に全探索するとO(10^11)でTLEする。
    // 各数字の桁の数の組合せが等しいものの積f(m)は等しいことに着目すると、計算量を減らせる。
    // 例えば、
    // m=112とm=121とm=211は桁の数の組合せが等しいので、f(m)の値は全て等しく2となる
    // このうちm - f(m) = bとなるようなmはただ一つしか存在しない。例えばb=110ならm=112だけ。
    // よって重複組合せの分だけ全探索すればいいので、計算量は
    // log10(n)H10 = (log10(n)+10)C10 <= 21C10となる
    // 重複組合せ分の全探索は、dfsで行う。詳細はdfs()関数の定義に記述した。

    // 自然数nの桁数を求める
    let number_of_digits_of_n = get_numer_of_digits(n);
    let mut ans = 0;
    // 全桁数について条件を満たすmを数える
    for number_of_digits in 1..number_of_digits_of_n+1 {
        dfs(number_of_digits, 0, &mut vec![], &mut ans, b, n);
    }
    println!("{}", ans);

    // 考察実験用
    // f(2) = 2
    // f(22) = 2*2=4
    // f(222) = 2*2*2=16
    // f(99) = 9*9=81
    // // m - f(m) = b となる1<=m<=Nの個数は?
    // let m = 2;
    // println!("f({})={}", m, function(m));
    // let m = 22;
    // println!("f({})={}", m, function(m));    
    // let m = 222;
    // println!("f({})={}", m, function(m));
    // let m = 10;
    // println!("f({})={}", m, function(m));
    // let m = 11;
    // println!("f({})={}", m, function(m));
    // let m = 12;
    // println!("f({})={}", m, function(m));
    // let m = 102;
    // println!("f({})={}", m, function(m));
    // for i in 1..1000 {
    //     println!("m={:3}, f(m)={:3}, m - f(m) = {:3}", i, function(i), i - function(i));
    // }
}

// 指定した桁数について、値の重複組合せの全探索をDFSで行う
// 計算量はnumber_of_digits+10_C_10
// 例えば3桁で使える数字が0-2なら(0,0,0),(0,0,1),(0,0,2),(0,1,1),(0,1,2),(0,2,2),(1,1,1),(1,1,2),(1,2,2),(2,2,2)
// 0の境界|, 1の境界|を各桁_ _ _のどこに配置するかは、2+3C2=5C2=10通りで↑で列挙した通り。
// 例えば(0,2,2)なら _ | | _ _ みたいな配置になる。
// 例えば(0,1,2)なら _ | _ | _ みたいな配置になる。 5個の中から2個の境界の場所を選ぶので5C2
fn dfs(
    number_of_digits: usize,            // 所望の桁数
    current_digit_value: usize,         // 現在、桁に格納を検討している値(0-9)
    digit_value_list: &mut Vec<usize>,  // 桁毎の値を格納
    ans: &mut usize,                    // 求める答え
    b: usize,                           // 入力B
    n: usize                            // 入力N
) {
    // 終了時の処理
    if digit_value_list.len() == number_of_digits {
        // 判定
        let mut digit_product: usize = 1; // 各位の数字の積
        for digit_value in digit_value_list.iter() {
            digit_product *= *digit_value;
        }

        let mut m = digit_product + b;
        if m > n {return}
        // mを桁毎にバラしたとき、その配列の組合せがdigit_value_listにあるか?
        // mを桁毎にバラす作業
        let mut m_digit_value_list = vec![];
        while m / 10 != 0 {
            m_digit_value_list.push(m % 10);
            m /= 10;
        }
        m_digit_value_list.push(m % 10);
        m_digit_value_list.sort();
        // digit_value_listとm_digit_value_listが同じであれば
        if *digit_value_list == m_digit_value_list {
            *ans += 1;
        }
        return
    }
    
    // その値を追加する
    digit_value_list.push(current_digit_value);
    dfs(number_of_digits, current_digit_value, digit_value_list, ans, b, n);
    digit_value_list.pop();

    // その値を追加しない
    if current_digit_value == 9 {return} // 9を追加しないとき、次に10を追加するかしないかとなってしまうので、ここで探索終わり
    dfs(number_of_digits, current_digit_value + 1, digit_value_list, ans, b, n);

}

// 0以上の整数nの桁数を求める
fn get_numer_of_digits(mut n: usize) -> usize {
    let mut numer_of_digits = 1;
    while n / 10 != 0 {
        n /= 10;
        numer_of_digits += 1;
    }
    return numer_of_digits
}

// 考察用
fn function(mut m: usize) -> usize {
    let mut ans = 1; 
    while m / 10 != 0 {
        ans *= m % 10;
        m /= 10;
    }
    ans *= m % 10;
    return ans
}