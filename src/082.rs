use proconio::input;
use itertools::Itertools;
use core::num;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let modulo = 1_000_000_007;

    let l_digit_num = get_digit_num(l.clone()) as u32;
    let r_digit_num = get_digit_num(r.clone()) as u32;

    let mut sum = 0;
    // 例
    // L=8
    // R=12
    // 88_888_888_999_999_999_10_101010_101010_101010_1111_111111_111111_111111_121212_121212_121212_121212
    // 8*1 + 9*1 + 10*2 + 11*2 + 12*3
    // 1桁の数字の和 = (初項+末項) * 項数 / 2 = 1 * (8+9) * (9-8+1) / 2
    // 2桁の数字の和 = 2 * (初項+末項) * 項数 / 2 = 2 * (10+12) * (12-10+1) / 2


    // Lと同じ桁数の文字数の和
    if l == 10_usize.pow(l_digit_num ) - 1 {
        sum += l_digit_num as usize * l % modulo;
        sum %= modulo;
    }
    else {
        sum += l_digit_num as usize * ((l + min((10_usize.pow(l_digit_num) - 1), r)) % modulo) % modulo * ((min(r, (10_usize.pow(l_digit_num)) - 1) - l + 1) % modulo) % modulo * mod_inverse(2, modulo) % modulo;
        sum %= modulo;
    }

    // Lより1桁多い数字 ~ Rより1桁小さい数字の文字数の和
    for i_digit_num in (l_digit_num+1)..(r_digit_num) {
        sum += i_digit_num as usize * ((10_usize.pow(i_digit_num - 1) + (10_usize.pow(i_digit_num) - 1)) % modulo) % modulo * (((10_usize.pow(i_digit_num)  - 1) - 10_usize.pow(i_digit_num - 1) + 1) % modulo) % modulo * mod_inverse(2, modulo) % modulo;
        sum %= modulo;
    }

    // Rと同じ桁数の文字数の和
    if l_digit_num != r_digit_num {
        if 10_usize.pow(r_digit_num - 1) == r {
            sum += (r % modulo * r_digit_num as usize) % modulo;
            sum %= modulo;
        }
        else {
            sum += r_digit_num as usize * ((10_usize.pow(r_digit_num - 1) + r) % modulo) % modulo * ((r - 10_usize.pow(r_digit_num - 1) + 1) % modulo) % modulo * mod_inverse(2, modulo) % modulo;
            sum %= modulo;
        }
    }

    println!("{}", sum % modulo);

}

fn get_digit_num(mut x: usize) -> usize {
    let mut number_of_digits = 0;
    while x != 0  {
        number_of_digits += 1;
        x /= 10;
    }
    return number_of_digits;
}

// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: usize, b: usize, modulo: usize) -> usize {
    return a % modulo * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: usize, modulo: usize) -> usize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // ただし、法pは素数で、aはpの倍数ではない整数。
    // aがpの倍数だと、a^(p-1)=0 (mod p)となる。

    return mod_pow(a % modulo, modulo - 2, modulo)
}

// mod p を法とした時の累乗
// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
// No.69参照
fn mod_pow(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    base %= modulo;
    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder
}



// https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
// 「1000000007 で割ったあまり」の求め方を総特集！ 〜 逆元から離散対数まで 〜 by けんちょん
// 【導入】
// 逆元の計算例
// 9 / 4 = 12 (mod 13)　が成り立つことを示す。：
// <=> 9 = 4 * 12 (mod 13)
// <=> 9 = 48 (mod 13)
// <=> 9 = 3 * 13 + 9 (mod 13)

// 【定理】
// p: 素数
// b: pで割れない切れない整数
// bx = a (mod p)
// を満たすxが一意に存在する

// 【mod pにおける逆元】
// 	  a / b (mod p)
// 	= a * (1 / b) (mod p)
// つまり、1 / b (mod p)が計算出来ればよい。
// 【定義】(1 / b)は「mod pにおけるbの逆元」という。
// 【定義】bをかけると、1になる数 (mod pの元で)

// ★逆元の求め方は2つ。計算量はO(logP)
// 1. Fermatの小定理を利用		（法pが素数でないと使えない。実装は楽）
// 素数pについて、aをpの倍数ではない整数として、下記が成立する
// 		a^(p-1) = 1 (mod p)
// ===========================================================
// 	<=>	a * a^(p-2) = 1(mod p)
// 	<=>	a^(p-2)がmod pにおけるaの逆元
// よってa^(p-2) mod pを求めれば良い。愚直にやると、O(p-2)かかるが、
// 繰り返し二乗法(90選のNo.69)でO(log2(p-2))の時間で計算できる

// 2. 拡張Euclidの互除法を利用	（法pが素数でなくても、逆元存在条件を満たせばok）
// 		a * x = 1 (mod p)
// 	<=>	a * x + p * y = 1 を満たす整数yが存在する
// 上記を満たすxを拡張Euclidの互除法で求める
// 	ax + by = 1	(a,bは互いに素。整数x,yを求める)

// 一般に
// 	ax + by = c　が解をもつとき、cがgcd(a,b)で割り切れる。	(*1)
// そこでc = c'gcd(a,b)とおくと、
// 	ax + by = c'gcd(a,b)
// また、一般に
// 	ax' + by' = gcd(a,b)					(*2)
// も成り立つ。


// ★逆元が存在する条件
// 逆元は常に存在しない。
// mod p でのaの逆元が存在する条件は、pとaとが互いに素であること。
	