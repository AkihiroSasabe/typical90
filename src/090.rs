use num::Signed;
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;

use my_library_rs::*;
type Fps = Fps998244353;

fn main() {
    let modulus: isize = 998244353;
    let root = make_root(modulus);
    let invroot: Vec<isize> = make_invroot(&root, modulus);

    // // convolution_by_ntt() の動作確認
    // let mut a = vec![1, 2, 3, 4];
    // let mut b = vec![5, 6 ,7, 8, 9];
    // // let c_len = a.len() + b.len() - 1;

    // let mut c = convolution_by_ntt(&mut a, &mut b, &root, &invroot, modulus);
    // // c.truncate(c_len);
    // println!("c={:?}", c);
    // return;



    // 形式的冪級数 の除算を求める関数 fps_division() の動作確認
    // 例1.
    // F(x)=x^3 + 2*x^2 + 0*x^1 + 5 
    // D(x)=x^2-x+4
    // Q(x)=x+3, R(x)=-x-7

    // let f = vec![5, 0, 2, 1];
    // let d = vec![4, -1, 1];
    // println!("f(x)={:?}", f);
    // println!("d(x)={:?}", d);
    // let q = fps_division(&f, &d, &root, &invroot); 
    // println!("q(x)={:?}", q);
    // for i in 0..q.len() {
    //     print!("{} ,", (q[i] + modulus) % modulus);
    // }
    // println!("");

    // 例2.
    // F(x)=8*x^3 + 0*x^2 + 0*x^1 -1 
    // D(x)=2x-1
    // Q(x)=4x^2+2x+1, R(x)=0
    // let f = vec![-1, 0, 0, 8];
    // let d = vec![-1, 2];  
    // let q = fps_division(&f, &d, &root, &invroot); 
    // println!("f(x)={:?}", f);
    // println!("d(x)={:?}", d);
    // println!("q(x)={:?}", q);

    // 形式的冪級数 の逆元 (mod x^m) を求める関数 fps_inv() の動作確認
    // let f = vec![-1, -1, 1]; // f(x) = -1 -x + x^2、g(x) = -1 + x - 2x^2 
    // // let f = vec![-1]; // f(x) = -1, g(x) = -1 (これは正しかった)
    // // let f = vec![-1, 1]; // f(x) = -1 + x, g(x) = -1 - x - x^2 になるべき
    // // let f = vec![5, 4, 3, 2, 1]; // f(x) = 5 + 4x + 3x^2 + 2x^3 + x^4, g(x) = -1 - x - x^2 になるべき
    // // 598946612 718735934 862483121 635682004 163871793 になるべき https://judge.yosupo.jp/problem/inv_of_formal_power_series のサンプル
    // // let g = fps_inv_slow(&f, 3, &root, &invroot, modulus);
    // // let g = fps_inv_slow(&f, f.len() as isize, &root, &invroot, modulus);
    // let g = fps_inv(&f, f.len() as isize, &root, &invroot, modulus);
    // println!("g={:?}", g); // g=[-1, 1]
    // for i in 0..g.len() {
    //     print!("{} ", (g[i] + modulus) % modulus);
    //     // if g[i].abs() > modulus / 2 {
    //     //     if g[i] < 0 {
    //     //         print!("{} ", (g[i] + modulus) % modulus);            
    //     //     }
    //     //     else {
    //     //         print!("{} ", (g[i] - modulus) % modulus);            
    //     //     }
    //     // }
    //     // else {
    //     //     print!("{} ", g[i]);            
    //     // }
    // }
    // return;

    // f(x) * g(x) = (-1 -x + x^2) * (-1 + x)
    //             = 1 + x - x^2 -x -x^2 + x^3
    //             = 1 - 2*x^2 + x^3
    //             = 

    // f(x) * g(x) = (-1 + x) * (-1 - x)
    //             = 1 +x -x -x^2
    //             = 1 - x^2
    

    // bostan_mori をフィボナッチ数列のケースで実装確認
    // let a = vec![1, 1];
    // let q = vec![1, -1, -1]; // an+2 = an+1 + an
    // let i = 4;

    // // habara_k 氏の bostan_mori の確認 https://atcoder.jp/contests/typical90/submissions/30533869
    // // let q2: Vec<u32> = q.iter().map(|&x| (x + modulus as isize) as u32 % modulus as u32).collect();
    // // let ans = Fps::bostan_mori(&a, &q2, i);
    // // println!("ans = {:?}", ans);
    // // return;

    // // let fib = bostan_mori(i, a.clone(), q.clone());
    // // println!("fib[{}]={:?}",i, fib);
    // for i in 0..7 {
    //     let fib = bostan_mori(i, a.clone(), q.clone());
    //     // let q2: Vec<u32> = q.iter().map(|&x| (x + modulus as isize) as u32 % modulus as u32).collect();
    //     // let fib = Fps::bostan_mori(&a, &q2, i);
    //     println!("fib[{}]={:?}", i, fib);
    //     // fib[0]=1
    //     // fib[1]=1
    //     // fib[2]=2
    //     // fib[3]=3
    //     // fib[4]=5
    //     // fib[5]=8
    //     // fib[6]=13
    // }
    // return;

    // // Rustの負の数の剰余について動作確認
    // // let q = 7;
    // // let x = 10;
    // // println!("{} % {} == {}", x, q, x % q);
    // // let x = -10;
    // // println!("{} % {} == {}", x, q, x % q);

    // // let x = 6;
    // // println!("{} % {} == {}", x, q, x % q);
    // // let x = -6;
    // // println!("{} % {} == {}", x, q, x % q);

    // // let x = 11;
    // // println!("{} % {} == {}", x, q, x % q);
    // // let x = -11;
    // // println!("{} % {} == {}", x, q, x % q);

    // //  10 % 7 ==  3
    // // -10 % 7 == -3
    // //   6 % 7 ==  6
    // //  -6 % 7 == -6
    // //  11 % 7 ==  4
    // // -11 % 7 == -4

    // return;


    input! {
        n: usize,
        k: usize,
    }    

    // 998244353
    // 998243505

    let debug = true;
    // let debug = false;
    if debug {
        full_task(n, k);
        return 
    }
    // 小課題1: 1 <= N <= 10^5, K=1
    if n <= 100_000 && k == 1 {
        // println!("task 1");
        sub_task1(n, k);
    }
    // 小課題2: 1 <= N <= 10^11, K=1
    else if n <= 100_000_000_000 && k == 1 {
        // println!("task 2");
        sub_task2(n, k);
    }
    // 小課題3, 小課題4, 小課題5: 1 <= N <= 10^4, 1 <= K <= 10^4 
    else if n <= 10_000 && k <= 10_000 {
        // println!("task 5");
        sub_task5(n, k);
    }
    else {
        // println!("task full");
        full_task(n, k);
    }
    
    // // 無意味な考察
    // // a1 := dp[0][0]
    // // a0 := dp[0][1]

    // // an = dp[n-1][0]
    // // an-1 = dp[n-1][1]

    // // a[i] = a[i-1] + b[i-1]
    // // b[i] = a[i-1]


    // // a[i] = a[i-1] + a[i-2] <- フィボナッチ数だ...!
    // // a[i] - a[i-1] - a[i-2] = 0
    // // x^2 - x - 1 = 0
    // // x^2 - (α + β)x + αβ  = 0
    // // x = (1 ± √5) / 2
    // // (a[i] - α*a[i-1]) = β*(a[i-1] - α*a[i-2])
    // // (a[i] - β*a[i-1]) = α*(a[i-1] - β*a[i-2])
    // // a[i] - α*a[i-1] = β^(i-1) * (a[1] - α*a[0])
    // // a[i] - β*a[i-1] = α^(i-1) * (a[1] - β*a[0])
    // // (β - α)a[i] = β^i * (a[1] - α*a[0]) - α^i * (a[1] - β*a[0])
    // // a[i] = (β^i * (a[1] - α*a[0]) - α^i * (a[1] - β*a[0])) / (β - α)
    // //      = ((β^i - α^i) * a[1] - (β^i * α - α^i * β) * a[0])) / (β - α)
    // //      = ((β^i - α^i) * a[1] - βα * (β^(i-1) - α^(i-1)) * a[0]) / (β - α)
    // //      = β^

    // // (x^3 - y^3) = (x - y) * (x^2 + xy + y^2) 
    // //             = (x - y) * ((x + y)^2 - xy)
    // // (x^4 - y^4) = (x - y) * (x^3 + x^2y + xy^2 + y^3)
    // //             = (x - y) * ((x + y)^3 - 2xy(x+y))
    // // (x^5 - y^5) = (x - y) * (x^4 + x^3y + x^2y^2 + xy^3 + y^4)
    // //             = (x - y) * (x^4 + y^4 + xy(x^2 + xy + y^2))
    // //             = (x - y) * (x + y)^4 - 4xy()

    // // (4C1, 4C2, 4C3) = (4, 6, 4)


    // // {1,0,0,0}
    // // {1,0,1,0}


    // // 21:09- 解説前の考察
    // // Aの制約 
    // // 0 <= A <= K

    // // dpや累積和も使えそう

    // // 4. N <= 100, K <= 100 のケースを考えようか
    // // N = 5なら
    // // [l,r]=[0,N-1]: min(A) <= 20, 20**N % MODULUS


}


// 参考 fps_inv を高速に動かすように実装
// https://nyaannyaan.github.io/library/fps/ntt-friendly-fps.hpp
// https://github.com/NyaanNyaan/library/blob/master/fps/ntt-friendly-fps.hpp#L48
// 動作確認済み: https://judge.yosupo.jp/submission/160770
fn fps_inv(f: &Vec<isize>, m: isize, root: &Vec<isize>, invroot: &Vec<isize>, modulus: isize) -> Vec<isize> {
    if f[0] == 0 {
        println!("error i fps_inv() and f[0] == 0");
    }

    let mut res = vec![0; m as usize];

    res[0] = mod_inv(f[0], modulus);
    // println!("inv start!!");
    // println!("res = {:?}", res);

    let mut d = 1;
    let mut log_d = 1;
    while d < m as usize {
        let mut f2 = vec![0; 2*d];
        let mut g2 = vec![0; 2*d];
        // println!("--- d = {} ----", d);
        // println!("log_d = {}", log_d);
        // println!("(0) init f(2d) and g(2d)");
        // println!("f2 = {:?}", f2);
        // println!("g2 = {:?}", g2);

        for j in 0..min(f.len(), 2*d) {
            f2[j] = f[j];
        }
        for j in 0..d {
            g2[j] = res[j];
        }
        // println!("(1) f[j] = (*this)[j]");
        // println!("f2 = {:?}", f2);
        // println!("(2) g[j] = res[j] ");
        // println!("g2 = {:?}", g2);

        // 1st NTT
        f2 = ntt(&f2, log_d - 1, root, modulus); // depthがd-1かもしれない
        g2 = ntt(&g2, log_d - 1, root, modulus); // depthがd-1かもしれない
        for j in 0..f2.len() {
            f2[j] %= modulus;
        }
        // println!("(3) 1st ntt");
        // println!("f2 = {:?}", f2);
        // println!("g2 = {:?}", g2);
        for j in 0..2*d {
            f2[j] *= g2[j];
            f2[j] %= modulus;
        }
        // println!("(4) f[j] *= g[j]");
        // println!("f2 = {:?}", f2);

        // 1st Inverse-NTT
        f2 = ntt(&f2, log_d - 1, invroot, modulus); // depthがd-1かもしれない
        // INTTのときは、最後に係数を割らないといけない
        let last_devisor = mod_inv(d as isize * 2, modulus);
        for j in 0..f2.len() {
            // INTTのときは、最後に係数を割る
            f2[j] = (f2[j] *  last_devisor) % modulus;
        }
        // println!("(5) 1st intt");
        // println!("f2 = {:?}", f2);

        for j in 0..d {
            f2[j] = 0;
        }
        // println!("(6) f[j] = 0");
        // println!("f2 = {:?}", f2);

        // 2nd NTT
        f2 = ntt(&f2, log_d - 1, root, modulus); // depthがd-1かもしれない
        // println!("(7) 2nd ntt");
        // println!("f2 = {:?}", f2);
        for j in 0..2*d {
            f2[j] *= g2[j];
            f2[j] %= modulus;
        }
        // println!("(8) f[j] *= g[j]");
        // println!("f2 = {:?}", f2);

        // 2nd Inverse-NTT
        f2 = ntt(&f2, log_d - 1, invroot, modulus); // depthがd-1かもしれない
        for j in 0..f2.len() {
            // INTTのときは、最後に係数を割る
            f2[j] = (f2[j] *  last_devisor) % modulus;
        }
        // println!("(9) 2nd intt");
        // println!("f2 = {:?}", f2);

        for j in d..min(2*d, m as usize) {
            res[j] = (- f2[j] + modulus) % modulus;
        }
        // println!("(10) res[j] = -f[j]");
        // println!("res = {:?}", res);
        d <<= 1;
        log_d += 1;
    }

    res.truncate(m as usize);
    return res
}

/// 形式的冪級数の逆元を求める NlogN (出力の次元)
// 参考: https://github.com/niuez/cp-rust-library/blob/master/src/math/formal_power_series.rs
fn fps_inv_slow(f: &Vec<isize>, m: isize, root: &Vec<isize>, invroot: &Vec<isize>, modulus: isize) -> Vec<isize> {
    // f(x) = a_0 + a_1 * x^1 + a_2 * x^2 + ... + a_(d-1) * x^(d-1) + ...
    // f := [a_0, a_1, a_2, ..., a_d-1]
    // x^m を法とする。
    // 1/f(x) := b_0 + b_1 * x^1 + ... b_m-1 * x^(m-1)
    // 1/f := [b_0, b_1, ..., b_m-1]

    let mut g = vec![];
    // まず、g ≡ f_0 ^ -1 (mod x^1)である。なぜなら
    // f = f_0 + f_1 * x^1 + f_2 * x^2 + ...
    //   = f_0 + x^1 (f_1 + f_2 * x^1 + ...)
    //   ≡ f_0 (mod x^1)
    let g0 = mod_inv(f[0], modulus);
    g.push(g0);

    // m 以上で、最小の2の累乗を求める
    let mut over_m = 1;
    let mut log_m = 0;
    while m > over_m {
        over_m *= 2;
        log_m += 1;
    }
    
    // ダブリング
    for i in 1..log_m+1 {
        // g(x) ≡ g_k(x)  (mod x^k) となる g_k(x) が既知のとき、
        // g(x) ≡ g_2k(x) (mod x^2k) となる g_2k(x) は以下のようになる。
        // g_2k(x) ≡ 2 * g_k(x) - f(x) * g_k(x)^2 (mod x^2k)

        // g_k(x)^2
        let mut g_pow2 = convolution_by_ntt(&mut g.clone(), &mut g.clone(), &root, &invroot, modulus);

        // x^2k 以上の次元は無駄なので削除 (最大次数が2k-1の多項式の、vecorの長さは2k)
        g_pow2.truncate(1 << i);

        // f(x) * g_k(x)^2
        let mut fg_pow2 = convolution_by_ntt(&mut f.clone(), &mut g_pow2, &root, &invroot, modulus);

        // 2 * g_k(x) - f(x) * g_k(x)^2 (mod x^2k)
        for j in 0..g.len() {
            g[j] *= 2;
            g[j] %= modulus;
            g[j] -= fg_pow2[j];
            g[j] %= modulus;
        }
        for j in g.len()..(1 << i) {
            g.push(- fg_pow2[j]);
        }
    }

    // over_m 個 -> m個 に減らす
    g.truncate(m as usize);

    return g
}

/// 形式的冪級数の除算を実行する
/// deg(f) + deg(d) = N として、O(NlogN) で求まる
/// https://nyaannyaan.github.io/library/fps/formal-power-series.hpp
/// https://github.com/NyaanNyaan/library/blob/master/fps/formal-power-series.hpp
fn fps_division(f: &Vec<isize>, d: &Vec<isize>, root: &Vec<isize>, invroot: &Vec<isize>) -> Vec<isize> {
    // f(x) = q(x)d(x) + r(x) の q(x) を求める
    // f(x): 割られる多項式 f0 + f1*x^1 + f2*x^2 + f3*x^3 + ... + fi*x^i
    // d(x): 割る多項式 d0 + d1*x^1 + d2*x^2 + d3*x^3 + ... + dj*x^j
    // q(x): 商の多項式 q0 + q1*x^1 + q2*x^2 + q3*x^3 + ... + qn-1*x^n-1
    // f = [f0, f1, f2, ..., fi]
    // d = [d0, d1, d2, ..., dj]
    // q = [q0, q1, q2, ..., qk]

    // 被除数の次数が、除数の次数より小さい場合、除算の結果は0
    if f.len() < d.len() {
        let q = vec![0];
        return q
    }

    // 割り算の結果の形式的冪級数の長さnを求める(最高次数はn-1)
    let n = f.len() + 1 - d.len(); // n = i + 1 - j (例えば (x^2 - 1) / (x^1 + 1) = x^1 - 1 のとき、3+1-2=2)
    let mut f_copy = f.clone();
    let mut d_copy = d.clone();

    let modulus: isize = 998244353;

    // 除数の次元が64以下のとき、愚直に筆算した方が早いから分岐してそう
    if d.len() <= 64 {
    // if false {
    // if true {
        // 例
        //                     f3 * x   + (f2-d1)
        //                     ______________________________________________
        // x^2 + d1*x^1 + d0 | f3 * x^3   +   f2 * x^2   +   f1 * x^1 + f0
        //                     f3 * x^3   +   d1 * x^2   +   d0 * x^1
        //                     -----------------------------------------------
        //                                 (f2-d1) * x^2 + (f1-d0) *x^1 + f0   
        //                                 (f2-d1) * x^2 +(f2-d1)d1*x^1 + (f2-d1)d0
        
        // 形式的冪級数の末尾に連続するゼロの項を削除し、形式的冪級数のサイズを最小限に保つ
        while d_copy.len() != 0 && d_copy[d_copy.len()-1] == 0 {
            d_copy.pop();
        }


        // 除数の最高次数の係数の逆数を求める
        let d_coeff = mod_inv(d_copy[d_copy.len()-1], modulus);
        
        // 除数の最高次の係数が1になるように、全係数に最高次の係数の逆数を掛ける
        // d(x) = d_m-1 * x^(m-1) + ... + d_1 * x^1 + d_0 * x^0
        //      = d_m-1 * (x^(m-1) + ... + d_1 / d_m-1 * x^1 + d_0 / d_m-1 * x^0)
        for di in &mut d_copy {
            *di *= d_coeff;
            *di %= modulus;
        }

        // 割り算の結果の形式的冪級数の次数を計算
        let q_size = f_copy.len() + 1 - d_copy.len(); // deg(x^n-1 / x^m-1) = deg(x^n-m)

        // 除数の数を格納
        let d_size = d_copy.len();

        // 商（quotient）の形式的冪級数を初期化
        let mut quo = vec![0; q_size];
        for i in (0..q_size).rev() {
            quo[i] = f_copy[i + d_size - 1];
            for j in 0..d_size {
                f_copy[i + j] -= quo[i] * d_copy[j];
                f_copy[i + j] %= modulus;
            }
        }
        for i in 0..q_size {
            quo[i] *= d_coeff;
            quo[i] %= modulus;
        }
        quo.resize(n, 0);
        return quo
    }

    // 除数の次元が64より大きいとき
    else {
        // 結論
        // q(x) ≡ rev(rev(f) / rev(d)) (mod x^n-m+1)
        // 方針
        // 定義
        // f(x) = d(x)q(x) + r(x)
        // deg(f) = n - 1
        // deg(d) = m - 1
        // deg(q) = n - m
        // deg(r) <= m - 2
        // f(x) := f_0 * x^0 + f_1 * x^1 + f_2 * x^2 + ... + f_n-1 *x^(n-1)
        // rev(f) := f(1/x) * x^(n-1)
        //         = f_0 * x^(n-1) + f_1 * x^(n-2) + f_2 * x^(n-3) + ... + f_n-1 * x^0
        //         = f_n-1 * x^0   + f_n-2 * x^1   + f_n-3 * x^2   + ... + f_0 * x^(n-1)    <- rev(f)はf(x)の係数をリバースしただけになるのがわかる。
        // 結論の導出
        // (1) f(x) = d(x)q(x) + r(x)
        // (2) f(1/x) = d(1/x)q(1/x) + r(1/x)                                                           <- x=1/xを代入
        // (3) f(1/x) * x^(n-1) = d(1/x)q(1/x) * x^(n-1) + r(1/x) * x^(n-1)                             <- x^n-1 を掛ける
        // (4) f(1/x) * x^(n-1) = d(1/x) * x^(m-1) * q(1/x) * x^(n-m) + r(1/x) * x^(m-2) * x^(n-m+1)    <- 式変形
        // (5) rev(f) = rev(d) * rev(q) + rev(r) * x^(n-m+1)                                            <- rev(f) := f(1/x) * x^(n-1) を代入
        // (6) rev(f) ≡ rev(d) * rev(q) (mod x^(n-m+1))                                                 <- 合同式
        // (7) rev(q) ≡ rev(f) / rev(d) (mod x^(n-m+1))                                                 <- 式変形
        // (8) q(x) ≡ rev(rev(f) / rev(d)) (mod x^(n-m+1))                                              <- 式変形


        // f(x): 割られる多項式 f0 + f1*x^1 + f2*x^2 + f3*x^3 + ... 
        // f = [f0, f1, f2, ...fi]
        f_copy.reverse(); // f = [f0, f1, f2, ..., fi] -> f = [fi, fi-1, fi-2, ..., f0]
        // println!("n={}", n);
        // println!("f_copy.reverse()={:?}", f_copy);
        // let mut rev_f = get_prefix(&f_copy, n);
        f_copy.truncate(n);
        // println!("rev_f={:?}", rev_f);
        
        d_copy.reverse();
        // println!("d_copy.reverse()={:?}", d_copy);
        let mut inv_d = fps_inv(&d_copy, n as isize, root, invroot, modulus);
        // println!("inv_d={:?}", inv_d);

        let mut quo = convolution_by_ntt(&mut f_copy, &mut inv_d, root, invroot, modulus);
        // let mut quo = convolution_by_ntt(&mut rev_f, &mut inv_d, root, invroot, modulus);
        quo.truncate(n);
        quo.reverse();
        return quo
    }

}

// // 先頭からn-1までの要素を取り出す
// fn get_prefix(fps: &Vec<isize>, n: usize) -> Vec<isize> {
//     let end_index = min(fps.len(), n);
//     fps.iter().take(end_index).cloned().collect()
// }

// 普通に母関数の分子を求める
fn get_p(a: &Vec<isize>, q: &Vec<isize>) -> Vec<isize> {
    // Ga(x) = a_0 + a_1 * x^1 + a_2 * x^2 + ... + a_(d-1) * x^(d-1) + ...
    // a := [a_0, a_1, a_2, ..., a_d-1] (aは、A(x)のd-1次の係数までで良い)

    // Q(x) = 1 - c_1 * x^1 - c_2 * x^2 - ... - c_d * x^d
    // q := [1, -c_1, -c_2, ..., -c_d]

    // P(x) = Ga(x)Q(x)
    //      = (a0 + a1*x^1 + a2*x^2 + ...)(1 - c1 * x^1 - c2 * x^2 - ... - cd * x^d)
    //      = a0*c0 - (a0*c1 + a1*c0)x^1 - (a0*c2 +a1*c1 + a2*c0)*x^2 ... - (a0*cd-1 + a1+*cd-2 + ... + ad-1*c0)*x^(d-1) - 0*x^d - 0*x^d+1 ...
    //      = a0*c0 - (a0*c1 + a1*c0)x^1 - (a0*c2 +a1*c1 + a2*c0)*x^2 ... - (a0*cd-1 + a1+*cd-2 + ... + ad-1*c0)*x^(d-1)
    //      = a0    - (a0*c1 - a1)x^1    - (a0*c2 +a1*c1 - a2)*x^2    ... - (a0*cd-1 + a1+*cd-2 + ... - ad-1)*x^(d-1)
    
    let mut p = vec![0; q.len()-1]; // pはd-1次までの多項式
    for dim in 0..a.len() {
        for i in 0..dim+1 {
            p[dim] += a[i] * q[dim-i];
        }
    }

    // または、シンプルにNTTなどで畳み込みをして、2d次元まで求めた後に、d-1次元まで削ったものを返した方が早い
    
    return p
}


// NTTのときに母関数の分子を求める
fn get_p_ntt(a: &Vec<isize>, q: &Vec<isize>, root: &Vec<isize>, invroot: &Vec<isize>, modulus: isize) -> Vec<isize> {
    // Ga(x) = a_0 + a_1 * x^1 + a_2 * x^2 + ... + a_(d-1) * x^(d-1) + ...
    // a := [a_0, a_1, a_2, ..., a_d-1] (aは、A(x)のd-1次の係数までで良い)

    // Q(x) = 1 - c_1 * x^1 - c_2 * x^2 - ... - c_d * x^d
    // q := [1, -c_1, -c_2, ..., -c_d]

    // P(x) = Ga(x)Q(x)
    //      = (a0 + a1*x^1 + a2*x^2 + ...)(1 - c1 * x^1 - c2 * x^2 - ... - cd * x^d)
    //      = a0*c0 - (a0*c1 + a1*c0)x^1 - (a0*c2 +a1*c1 + a2*c0)*x^2 ... - (a0*cd-1 + a1+*cd-2 + ... + ad-1*c0)*x^(d-1) - 0*x^d - 0*x^d+1 ...
    //      = a0*c0 - (a0*c1 + a1*c0)x^1 - (a0*c2 +a1*c1 + a2*c0)*x^2 ... - (a0*cd-1 + a1+*cd-2 + ... + ad-1*c0)*x^(d-1)
    //      = a0    - (a0*c1 - a1)x^1    - (a0*c2 +a1*c1 - a2)*x^2    ... - (a0*cd-1 + a1+*cd-2 + ... - ad-1)*x^(d-1)
    
    // let mut p = vec![0; q.len()-1]; // pはd-1次までの多項式
    // for dim in 0..a.len() {
    //     for i in 0..dim+1 {
    //         p[dim] += (a[i] * q[dim-i]) % modulus;
    //     }
    // }

    let p_size = q.len()-1; // pはd-1次までの多項式

    // または、シンプルにNTTなどで畳み込みをして、2d次元まで求めた後に、d-1次元まで削ったものを返した方が早い
    let mut p = convolution_by_ntt(&mut a.clone(), &mut q.clone(), &root, &invroot, modulus);
    p.truncate(p_size);
    
    return p
}

// fn convolution(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
//     // 計算量は、O(a.len() x b.len())
//     let mut c = vec![0; a.len() + b.len() - 1];
//     for i in 0..a.len() {
//         for j in 0..b.len() {
//             c[i + j] = a[i] * b[j];
//         }
//     }
//     return c
// }



fn conv_fx_gmx(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    // f(x) * g(-x) を求める
    // f(x) := a[0] + a[1] * x + a[2] * x^2 + ... + a[n] * x^n
    // g(x) := b[0] + b[1] * x + b[2] * x^2 + ... + b[m] * x^m

    let mut c = vec![0; a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            // jが偶数のときは、a[i] * b[j]
            // jが奇数のときは、a[i] * (- b[j]) 
            // で計算するように調整
            c[i + j] += a[i] * b[j] * (1 - 2 * (j % 2) as isize);
            // if j % 2 == 0 {
            //     c[i + j] += a[i] * b[j];
            // }
            // else {
            //     c[i + j] -= a[i] * b[j];
            // }
        }
    }
    return c
}

fn get_even(a: &Vec<isize>) -> Vec<isize> {
    // 数列aの偶数項だけ抜き出す
    let mut a_even = vec![];
    for i in 0..((a.len()+1)/2) {
        a_even.push(a[i*2]);
    }
    return a_even
}

fn get_odd(a: &Vec<isize>) -> Vec<isize> {
    // 数列aの奇数項だけ抜き出す
    let mut a_odd = vec![];
    for i in 0..(a.len()/2) {
        a_odd.push(a[2*i+1]);
    }
    return a_odd
}

fn get_qmx(qx: &Vec<isize>) -> Vec<isize> {
    let mut qmx = vec![];
    if qx.len() % 2 == 0 {
        // 偶数個のとき
        for i in 0..(qx.len()/2) {
            qmx.push(qx[2*i]);
            qmx.push(-qx[2*i+1]);
        }
    }
    else {
        // 奇数個のとき
        for i in 0..(qx.len()/2) {
            qmx.push(qx[2*i]);
            qmx.push(-qx[2*i+1]);
        }
        // 最後の1個を追加
        qmx.push(qx[qx.len()-1]);
    }
    return qmx
}

/// https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a
/// 線形漸化的数列A={a0, a1, ..., a∞} の 第n項anを計算量O(M(d)logN)で求める。NTTが使えれば、O(d*logd*logN)で求まる。
/// M(d)は、d次多項式同士の積の計算量で、定義通りにやるとO(d^2). NTTならO(d*log(d))
/// d+1項間漸化式: a_n = c1 * a_n-1 + c2 * a_n-2 + ... + cd * a_n-d
fn bostan_mori(mut n: usize, mut p: Vec<isize>, mut q: Vec<isize>) -> isize{
// fn bostan_mori(mut n: usize, a: Vec<isize>, mut q: Vec<isize>) -> isize{
    // n: 求めたい数列Aの項番号 (0-indexed)
    // a := [a_0, a_1, a_2, ..., a_d-1] (aは、Ga(x)のd-1次の係数までで良い)
    // q := [1, -c_1, -c_2, ..., -c_d]

    // Ga(x): 数列Aの母関数
    // Ga(x) := a_0 + a_1 * x^1 + a_2 * x^2 + ... + a_(d-1) * x^(d-1) + ... 
    //        = P(x) / Q(x)

    // P(x): 次数d-1以下。母関数の分子。
    // P(x) := Ga(x)Q(x) <- 循環した定義に思えるが、Ga(x)をマクローリン展開したd-1次の係数まで分かっていればP(x)を計算可能で、それは既知。（※）
    //       = (a0 + a1*x^1 + a2*x^2 + ...)(1 - c1 * x^1 - c2 * x^2 - ... - cd * x^d)
    //       = a0*c0 - (a0*c1 + a1*c0)x^1 - (a0*c2 +a1*c1 + a2*c0)*x^2 ... - (a0*cd-1 + a1+*cd-2 + ... + ad-1*c0)*x^(d-1) - 0*x^d - 0*x^d+1 ...
    //       = a0    - (a0*c1 - a1)x^1    - (a0*c2 +a1*c1 - a2)*x^2    ... - (a0*cd-1 + a1+*cd-2 + ... - ad-1)*x^(d-1)
    // （※）n >= dの次元の係数は、a_n - (c1 * a_n-1 + c2 * a_n-2 + ... + cd * a_n-d) =  0 になってくれる。

    // Q(x): 次数dで、Q(0)=1。母関数の分母で、漸化式の係数から得られる。
    // Q(x) := 1 - c1 * x^1 - c2 * x^2 - ... - cd * x^d
    // an = [x^n]Ga(x) 
    //    = [x^n](P(x)/Q(x)) 
    //    = [x^n]((P(x)Q(-x)) / (Q(x)Q(-x)))
    //    = [x^n]((Ueven(x^2) + x*Uodd(x^2)) / V(x^2)) ∵Q(x)Q(-x)は偶関数になる。f(x)=f(-x)となるので。Q(x)Q(-x) = Q(-x)Q(-(-x))
    // となる。
    // 繰り返し二乗法
    // nが偶数: a[n] = [x^n]Ueven(x^2)/V(x^2) 
    //               = [x^n/2]Ueven(x)/V(x)
    // nが奇数: a[n] = [x^n]x*Uodd(x^2)/V(x^2) 
    //               = [x^(n-1)/2]Uodd(x)/V(x)]
    let modulus: isize = 998244353;
    let root = make_root(modulus);
    let invroot: Vec<isize> = make_invroot(&root, modulus);

    // println!("A(x)={:?}", a);

    let ntt_flag = true;

    // let mut p = if ntt_flag {
    //     get_p_ntt(&a, &q, &root, &invroot, modulus)
    // } else {
    //     get_p(&a, &q)
    // };
    // println!("p(x) = {:?}", p);
    // println!("n = {}", n);
    // println!("q.len() = {}", q.len());


    while n > 0 {
        // U(x) := P(x)Q(-x) =: Ueven(x^2) + x*Uodd(x^2) を求める

        let mut qmx = get_qmx(&q); // nttのときだけ。
        let u: Vec<isize> = if ntt_flag {
            convolution_by_ntt(&mut p, &mut qmx.clone(), &root, &invroot, modulus)
        } else {
            conv_fx_gmx(&p, &q)
        };
        // let u: Vec<isize> = conv_fx_gmx(&p, &q);
        // let mut qmx = get_qmx(&q); // nttのときだけ。
        // let u = convolution_by_ntt(&mut p.clone(), &mut qmx.clone(), &root, &invroot, modulus);
        // println!("--------- n = {} ---------", n);
        // println!("P(x)  = {:?}", p);
        // println!("Q(x)  = {:?}", q);
        // println!("Q(-x) = {:?}", qmx);
        // println!("U(x)  = P(x)Q(-x) = {:?}", u);

        // 次の分子を求める。つまり、
        // U(x) = Ueven(x^2) + x*Uodd(x^2) から、Ueven(x)またはUodd(x)を取得する
        match n % 2 {
            0 => p = get_even(&u),  // Ueven(x^2) の係数を格納
            _ => p = get_odd(&u)    // xUodd(x^2) の係数を格納
        }
        // 次の分母を求める。つまり、
        // V(x^2) := Q(x)Q(-x) を求める (Q(x)Q(-x)の奇数次元の係数は打ち消されるので0)
        let v: Vec<isize> = if ntt_flag {
            convolution_by_ntt(&mut q, &mut qmx, &root, &invroot, modulus)
        } else {
            conv_fx_gmx(&q, &q)
        };

        // let v = conv_fx_gmx(&q, &q);
        // let v = convolution_by_ntt(&mut p.clone(), &mut qmx, &root, &invroot, modulus);
        // println!("V(x^2) = Q(x)Q(-x) = {:?}", v);

        q = get_even(&v);
        n /= 2;
    }
    // return p[0]
    return (p[0] + modulus) % modulus // 負の数を正として出力する

}


// mod を法とする x の逆元を計算する．
fn mod_inv(x: isize, modulus: isize) -> isize {
    return my_pow(x, modulus - 2, modulus);
}

// 繰り返し二乗法で x ^ n を mod で割った余りを求める．
fn my_pow(x: isize, n: isize, modulus: isize) -> isize{
    let mut ret = 0;
    if n == 0 {
        ret = 1;
    }
    else if n % 2 == 1 {
        ret = (x * my_pow((x * x) % modulus, n / 2, modulus)) % modulus;
    }
    else {
        ret = my_pow((x * x) % modulus, n / 2, modulus);
    }
    return ret;
}

// NTT に必要となる r の累乗数を求める．
fn make_root(modulus: isize) -> Vec<isize> {
    let mut ret = vec![];
    let mut r = my_pow(3, 119, modulus);
    for i in 0..23 {
        ret.push(r);
        r = (r * r) % modulus;
    }
    ret.reverse();
    return ret;
}

// NTT に必要となる r の累乗数の逆元を求める．
fn make_invroot(root: &Vec<isize>, modulus: isize) -> Vec<isize> {
    let mut ret = vec![];
    for i in 0..root.len() {
        ret.push(mod_inv(root[i], modulus));
    }
    return ret
}


// 数論変換 (Number Theoretic Transform: NTT)
fn ntt(a: &Vec<isize>, depth: isize, root: &Vec<isize>, modulus: isize) -> Vec<isize>{
    // inv = 1 ならば普通の NTT，
    // inv = -1 ならば INTT になるようにする（今回は，呼び出す root が逆元かそうでないかによって調整する）．
    let n = a.len();
    // println!("depth == {}", depth);
    // a のサイズが 1 であるときは，それがそのまま DFT である．
    if n == 1{
        return a.clone()
    }
    else{
        let mut ret = vec![];
        let mut even = vec![];
        let mut odd = vec![];
        for i in 0..n {
            if i % 2 == 0 {
                even.push(a[i]);
            }
            else {
                odd.push(a[i]);
            }
        }

        // even と odd の DFT を，再帰的に求める．
        let d_even = ntt(&even, depth - 1, root, modulus);
        let d_odd = ntt(&odd, depth - 1, root, modulus);

        let index = if depth >= 0 {
            depth as usize
        }
        else {
            root.len() - depth as usize
        };
        let r = root[index];
        
        let mut now = 1;
        for i in 0..n {
            ret.push((d_even[i % (n / 2)] + (now * d_odd[i % (n / 2)]) % modulus) % modulus);
            // ret.push((d_even[i % (n / 2)] + (now * d_odd[i % (n / 2)]) % modulus + 2*modulus) % modulus); // 負だった場合に、正にする <- この処理は要らない (Rustの負の数の余りは、- |r| % m となってくれるので。)
            now = (now * r) % modulus;
        }
        return ret;
    }
}


fn convolution_by_ntt(a: &mut Vec<isize>, b: &mut Vec<isize>, root: &Vec<isize>, invroot: &Vec<isize>, modulus: isize) -> Vec<isize> {
    // 配列 a, b は，それぞれ A(x) と B(x) の係数を次数の小さい順に並べたもの．
    let len_a = a.len();
    let len_b = b.len();
    let len_c = len_a + len_b - 1; // len_c は A(x) * B(x) の次数
    let mut n = 1;
    // len_c より大きい最小の 2 べきの数を求める
    while n <= len_c {
        n *= 2;
    }

    // 配列の長さが n になるまで，配列の末尾に 0 を追加する
    while a.len() < n {
        a.push(0);
    }
    while b.len() < n {
        b.push(0);
    }

    let mut log_2n = 1;
    while (1 << log_2n) < n {
        log_2n += 1;
    }

    // A(x) の NTT DA(t), b(x) の NTT DB(t) を求める．
    // 配列 da, db は，それぞれ DA(t), DB(t) の係数を次数の小さい順に並べたもの．
    let da = ntt(a, log_2n as isize - 1, root, modulus);
    let db = ntt(b, log_2n as isize - 1, root, modulus);

    // C(x) の NTT DC(t). これの k 次の係数は， DA(t) と DB(t) の k 次の係数を掛け合わせれば求まる．
    let mut dc = vec![0; n];
    for i in 0..n {
        dc[i] = (da[i] * db[i]) % modulus;
        // dc[i] = (dc[i] + modulus) % modulus; // 負だった場合に、正にする <- この処理は要らない (Rustの負の数の余りは、- |r| % m となってくれるので。)
    }

    // C(x) は DC(t) を INTT すれば求まる．このようにしてできた配列 c は，C(x) の係数を次数の小さい順に並べたものとなっている．
    let c = ntt(&dc, log_2n as isize - 1, invroot, modulus);
    // INTT の後は最後に n で割ることを忘れずに．(ここは、nで割るとき、ループ毎に毎回mod_inv計算するのは勿体ない)
    let mut ret: Vec<isize> = vec![];
    let inverse_n = mod_inv(n as isize, modulus);
    // for i in 0..n {
    for i in 0..len_c {
        ret.push((c[i] * inverse_n) % modulus);
        // ret.push(((c[i] * inverse_n) % modulus + modulus) % modulus); // 負だった場合に、正にする <- この処理は要らない (Rustの負の数の余りは、- |r| % m となってくれるので。)
    }

    return ret
}


// 小課題1: 1 <= N <= 10^5, K=1
fn sub_task1(n: usize, k: usize) {
    let modulus = 998244353;

    // 1が連続しなければ、何でもいい
    // => シンプルにDPで解くO(N)
    // dp[i][0] := a[i]==0の状態数。 ただし、使えるAは閉区間[0, i]
    // dp[i][1] := a[i]==1の状態数。 ただし、使えるAは閉区間[0, i]
    let mut dp = vec![vec![0,0]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 1..n {
        dp[i][0] = dp[i-1][0] + dp[i-1][1];
        dp[i][1] = dp[i-1][0];
        dp[i][0] %= modulus;
        dp[i][1] %= modulus;
    }
    // debug
    // println!("dp[n-1][0] = {}", dp[n-1][0]);
    // println!("dp[n-1][1] = {}", dp[n-1][1]);
    println!("{}", (dp[n-1][0] + dp[n-1][1]) % modulus);
}


// 小課題2: 1 <= N <= 10^11, K=1
fn sub_task2(n: usize, k: usize) {
    let modulus = 998244353;

    // 小課題1のDPの遷移がフィボナッチ数列であることに気づけば、
    // 行列の累乗であるので、繰り返し二乗法を使ってO(logN)で計算可能
    let coefficient_matrix = vec![
        vec![1, 1],
        vec![1, 0],
    ];
    // a[n-1] = [1, 1] ^(n-2) a[1]
    // a[n-2]   [1, 0]        a[0]
    let final_coefficient_matrix = power_of_matrix(coefficient_matrix, n-1, modulus);

    let a1_a0 = vec![
        vec![1],
        vec![1],
    ];
    let anm1_anm2 = multiply_matrix(&final_coefficient_matrix, &a1_a0, modulus);
    // println!("anm1_anm2 = {:?}", anm1_anm2);
    println!("{}", (anm1_anm2[0][0] + anm1_anm2[1][0]) % modulus);
}

fn sub_task5(n: usize, k: usize) {
    let modulus: isize = 998244353;

    // bostan_mori(mut n, a: Vec<isize>, mut q: Vec<isize>)
    // まずは、
    // Hが高い奴から
    // dp[L][H] := 区間Lの最小値がHな数列の個数 by かつっぱ　<- この定義は駄目な気がする

    // E8定義のdp
    // dp[h][m] := 条件を満たす長さmの数列で、値がすべてh以上であるものの個数 <- E8の定義。最終的に知りたいのは、dp[0][n]の値が知りたい...!
    // dp[h][m] = (Ai=hとなる最小のiが0のケース) + (Ai=hとなる最小のiが1のケース) + ... + (Ai=hとなる最小のiがm-1のケース) + (m個が全てh+1以上)
    //          = dp[h][m-1] + dp[h+1][1] * dp[h][m-2] + ... + dp[h+1][m-1] * dp[h][0] + dp[h+1][m]
    //          = (m個が全てh+1以上) + (Ai=hとなる最小のiがm-1のケース) + (Ai=hとなる最小のiがm-2のケース) + ... + (Ai=hとなる最小のiが1のケース) + (Ai=hとなる最小のiが0のケース)
    //          = dp[h+1][m] + dp[h+1][m-1] * dp[h][0] + dp[h+1][m-2] * dp[h][1] + ... + dp[h+1][1] * dp[h][m-2] + dp[h][m-1]
    //          = dp[h+1][m] * dp[h][-1] + dp[h+1][m-1] * dp[h][0] + dp[h+1][m-2] * dp[h][1] + ... + dp[h+1][1] * dp[h][m-2] + dp[h][m-1] // dp[h][-1]=1を仮に導入。本当は、dp[h][-1]=0だけどね。
    //          = dp[h+1][m] * dp[h][-1] + dp[h+1][m-1] * dp[h][0] + dp[h+1][m-2] * dp[h][1] + ... + dp[h+1][1] * dp[h][m-2] + dp[h+1][0] * dp[h][m-1] // dp[h+1][0] = 1 を仮に導入。本当は、dp[h+1][0]=0だけどね。
    // dp[h][m] = a_m, dp[h+1][m+1] = c_m  と置くと...
    //      a_m = dp[h+1][m] *a_-1 + dp[h+1][m-1] *a_0 + dp[h+1][m-2] * a_1 + dp[h+1][m-3] * a_2 + ... + dp[h+1][2] * a_m-3 + dp[h+1][1] * a_m-2 + dp[h+1][0] * a_m-1 // m+2項間漸化式と同じ
    //      a_m = c_m-1 * a_-1 + c_m-2 * a_0 + c_m-3 * a_1 + c_m-4 * a_2 + ... + c_1 * a_m-3 + c_0 * a_m-2 +  c_-1 * a_m-1 // m+2項間漸化式と同じ
    //      a_m = Σ[i=1, m+1] c_m-i * a_i
    // dp[h][m] = Σ[i=0, m] dp[h+1][m-i] * dp[h][i-1]

    // E8定義のdp
    // hは。最大でkとなる。
    // mは、m * h <= k ⇔ m <= k / h 。k*log(k)*k個...?
    let mut dp = vec![vec![]; k+1];
    

    // E8定義 dp[h][m]
    // dp[k][0] = 1; // 本当は0
    // dp[k][1] = 1;
    // dp[k][2] = 0; // m > 1のとき、k*m > k となるので、全部0. 一般に、hのとき、m <= k / h とならないといけない。

    // 具体例
    // n=10, k=7 のとき
    // h=kのときは、m=0,1のときだけ1になるので、自分で初期化する
    // dp[7][-1] = 1 // 本当は定義できないけど、1にしておくと計算の都合がいい
    // dp[7][0] = 1 // 本当は0だけど、1にしておくと計算の都合がいい
    // dp[7][1] = 1 ({7}, )
    // m >= 2 のとき、h * m >= 7 * 2 = 14 > k = 7 となるので。 dp[7][m] = dp[7+1][m] = dp[8][m] = 0
    // dp[7][2] = 0 ( 2 <= m のとき、dp[7][m]=0)

    // dp[6][-1] = 1 // 本当は定義できないけど、1にしておくと計算の都合がいい
    // dp[6][0] = 1 // 本当は0だけど、1にしておくと計算の都合がいい
    // dp[6][1] = dp[7][1] * dp[6][-1] + dp[7][0] * dp[6][0] = 1*1 + 1*1 =2 ({6}, {7})
    //          = 全部7以上(i=1で初6)   + i=0で初めて6が来るケース
    // m >= 2 のとき、h * m >= 6 * 2 = 12 > k となるので、dp[6][m] = dp[6+1][m]  = dp[7][m] = 0
    // dp[6][2] = 0

    // dp[5][-1] = 1
    // dp[5][0] = 1
    // dp[5][1] = dp[6][1] * dp[5][-1] + dp[6][0] * dp[5][0] = 2*1 + 1*1 = 3 ({5}, {6}, {7})
    // m >= 2 のとき、h * m >= 5 * 2 = 10 > k となるので、dp[5][m] = dp[5+1][m]  = dp[6][m] = 0
    // dp[5][2] = 0

    // dp[4][-1] = 1
    // dp[4][0] = 1
    // dp[4][1] = dp[5][1] * dp[4][-1] + dp[6][0] * dp[5][0] = 3*1 + 1*1 = 4 ({4}, {5}, {6}, {7})
    // m >= 2 のとき、h * m >= 4 * 2 = 8 > k となるので、dp[4][m] = dp[4+1][m] = dp[5][m] = 0
    // dp[4][2] = 0

    // dp[3][-1] = 1
    // dp[3][0] = 1
    // dp[3][1] = dp[4][1] * dp[3][-1] + dp[4][0] * dp[3][0] = 4*1 + 1*1* = 5 ({3}, {4}, {5}, {6}, {7})
    // m = 2 のとき、 h * m = 3 * 2 = 6 <= k=7なので、ok
    // dp[3][2] = dp[4][2] * dp[3][-1] + dp[4][1] * dp[3][0] + dp[4][0] * dp[3][1] = 0 + 4*1 + 1*5 = 9 = ({4,3}, {5,3}, {6,3}, {7,3}, {3,3}, {3,4}, {3,5}, {3,6}, {3,7})
    // m >= 3 のとき、 h * m >= 3 * 3 = 9 > k=7 となるので、dp[3][m] = dp[3+1][m] = dp[4][m] = 0

    // m <= k/h = 7/2= 3.5
    // dp[2][-1] = 1
    // dp[2][0] = 1
    // dp[2][1] = dp[3][1] * dp[2][-1] + dp[3][0] * dp[2][0] = 5*1 + 1*1 = 6 ({2}, {3}, {4}, {5}, {6}, {7})
    // m = 2 のとき、 h * m = 2 * 2 = 4 <= k=7なので、ok
    // dp[2][2] = dp[3][2] * dp[2][-1] + dp[3][1] * dp[2][0] + dp[3][0] * dp[2][1]
    //          = 全部が3以上(i=2で初2) + i=1で初めて2が来る   + i=0で初めて2が来る 
    // m = 3 のとき、 h * m = 2 * 3 = 6 <= k=7なので、ok
    // dp[2][3] = dp[3][3] * dp[2][-1] + dp[3][2] * dp[2][0] + dp[3][1] * dp[2][1] + dp[3][0] * dp[2][2]
    // m >= 4 のとき、 h * m >= 2 * 4 = 8 > k=7なので、dp[2][m] = 0

    // 制約: h * m <= k
    // m <= k/h = 7/1= 7
    // dp[h][m], h=1
    // dp[1][-1] = 1
    // dp[1][0] = 1
    // dp[1][1] = dp[2][1] * dp[1][-1] + dp[2][0] * dp[1][0] = 6*1 + 1*1 = 7 ({1}, {2}, {3}, {4}, {5}, {6}, {7})
    // dp[1][2] = dp[2][2] * dp[1][-1] + dp[2][1] * dp[1][0] + dp[2][0] * dp[1][1] = 
    // dp[1][3] = dp[2][3] * dp[1][-1] + dp[2][2] * dp[1][0] + dp[2][1] * dp[1][1] + dp[2][0] * dp[1][2]
    // dp[1][4] = dp[2][4] * dp[1][-1] + dp[2][3] * dp[1][0] + dp[2][2] * dp[1][1] + dp[2][1] * dp[1][2] + dp[2][0] * dp[1][3]
    // dp[1][5] = dp[2][5] * dp[1][-1] + dp[2][4] * dp[1][0] + dp[2][3] * dp[1][1] + dp[2][2] * dp[1][2] + dp[2][1] * dp[1][3] + dp[2][0] * dp[1][4]
    // dp[1][6] = dp[2][6] * dp[1][-1] + dp[2][5] * dp[1][0] + dp[2][4] * dp[1][1] + dp[2][3] * dp[1][2] + dp[2][2] * dp[1][3] + dp[2][1] * dp[1][4] + dp[2][0] * dp[1][5]
    // dp[1][7] = dp[2][7] * dp[1][-1] + dp[2][6] * dp[1][0] + dp[2][5] * dp[1][1] + dp[2][4] * dp[1][2] + dp[2][3] * dp[1][3] + dp[2][2] * dp[1][4] + dp[2][1] * dp[1][5]+ dp[2][0] * dp[1][6]
    // m >= 8 のとき、 h * m >= 1 * 8 = 8 > k = 7なので、dp[1][m] = 0

    // h = 1 についてもう一度考えると実は、、、
    // dp[1][m] = 0 (m < -1) とすれば以下のようになる。
    // dp[1][-1] = 1
    // dp[1][0] = 1
    // dp[1][1] = dp[2][7] * dp[1][-7] + dp[2][6] * dp[1][-6] + dp[2][5] * dp[1][-5] + dp[2][4] * dp[1][-4] + dp[2][3] * dp[1][-3] + dp[2][2] * dp[1][-2] + dp[2][1] * dp[1][-1] + dp[2][0] * dp[1][0]
    // dp[1][2] = dp[2][7] * dp[1][-6] + dp[2][6] * dp[1][-5] + dp[2][5] * dp[1][-4] + dp[2][4] * dp[1][-3] + dp[2][3] * dp[1][-2] + dp[2][2] * dp[1][-1] + dp[2][1] * dp[1][0] + dp[2][0] * dp[1][1] 
    // dp[1][3] = dp[2][7] * dp[1][-5] + dp[2][6] * dp[1][-4] + dp[2][5] * dp[1][-3] + dp[2][4] * dp[1][-2] + dp[2][3] * dp[1][-1] + dp[2][2] * dp[1][0] + dp[2][1] * dp[1][1] + dp[2][0] * dp[1][2]
    // dp[1][4] = dp[2][7] * dp[1][-4] + dp[2][6] * dp[1][-3] + dp[2][5] * dp[1][-2] + dp[2][4] * dp[1][-1] + dp[2][3] * dp[1][0] + dp[2][2] * dp[1][1] + dp[2][1] * dp[1][2] + dp[2][0] * dp[1][3]
    // dp[1][5] = dp[2][7] * dp[1][-3] + dp[2][6] * dp[1][-2] + dp[2][5] * dp[1][-1] + dp[2][4] * dp[1][0] + dp[2][3] * dp[1][1] + dp[2][2] * dp[1][2] + dp[2][1] * dp[1][3] + dp[2][0] * dp[1][4]
    // dp[1][6] = dp[2][7] * dp[1][-2] + dp[2][6] * dp[1][-1] + dp[2][5] * dp[1][0] + dp[2][4] * dp[1][1] + dp[2][3] * dp[1][2] + dp[2][2] * dp[1][3] + dp[2][1] * dp[1][4] + dp[2][0] * dp[1][5]
    // dp[1][7] = dp[2][7] * dp[1][-1] + dp[2][6] * dp[1][0] + dp[2][5] * dp[1][1] + dp[2][4] * dp[1][2] + dp[2][3] * dp[1][3] + dp[2][2] * dp[1][4] + dp[2][1] * dp[1][5]+ dp[2][0] * dp[1][6]
    // つまりこれは、dp[1][m-1] = a_m とおくと、
    // a_8 = dp[2][7] * a_0 + dp[2][6] * a_1 + ... + dp[2][0] * a_7
    // a_m+1 = dp[2][m] * a_0 + dp[2][m-1] * a_1 + ... + dp[2][0] * a_m

    // h=0, dp[h][m]
    // dp[0][-1] = 1
    // dp[0][0] = 1
    // dp[0][1] = dp[1][1] * dp[0][-1] + dp[1][0] * dp[0][0] = 7*1 + 1*1 = 8 ({0}, {1}, {2}, {3}, {4}, {5}, {6}, {7})
    // dp[0][2] = dp[1][2] * dp[0][-1] + dp[1][1] * dp[0][0] + dp[1][0] * dp[0][1]
    // dp[0][3] = dp[1][3] * dp[0][-1] + dp[1][2] * dp[0][0] + dp[1][1] * dp[0][1] + dp[1][0] * dp[0][2]
    // dp[0][4] = dp[1][4] * dp[0][-1] + dp[1][3] * dp[0][0] + dp[1][2] * dp[0][1] + dp[1][1] * dp[0][2] + dp[1][0] * dp[0][3]
    // dp[0][5] = dp[1][5] * dp[0][-1] + dp[1][4] * dp[0][0] + dp[1][3] * dp[0][1] + dp[1][2] * dp[0][2] + dp[1][1] * dp[0][3] + dp[1][0] * dp[0][4]
    // dp[0][6] = dp[1][6] * dp[0][-1] + dp[1][5] * dp[0][0] + dp[1][4] * dp[0][1] + dp[1][3] * dp[0][2] + dp[1][2] * dp[0][3] + dp[1][1] * dp[0][4] + dp[1][0] * dp[0][5]
    // dp[0][7] = dp[1][7] * dp[0][-1] + dp[1][6] * dp[0][0] + dp[1][5] * dp[0][1] + dp[1][4] * dp[0][2] + dp[1][3] * dp[0][3] + dp[1][2] * dp[0][4] + dp[1][1] * dp[0][5] + dp[1][0] * dp[0][6]
    // if k => n ならばdp[0][n]まで計算できればいい。
    
    // else if k < n ならば、m=nまで続ける必要がある。
    // ただし、m > kのときは、dp[0][m]はm+1個の多項式ではなく、k+1個の多項式になってくれる。(m <= kのときは、dp[h][m]はm+1個の多項式だった)
    // dp[0][8] = dp[1][8] * dp[0][-1] + dp[1][7] * dp[0][0] + dp[1][6] * dp[0][1] + dp[1][5] * dp[0][2] + dp[1][4] * dp[0][3] + dp[1][3] * dp[0][4] + dp[1][2] * dp[0][5] + dp[1][1] * dp[0][6] + dp[1][0] * dp[0][7]
    //            ~~~~(=0)
    //          = dp[1][7] * dp[0][0] + dp[1][6] * dp[0][1] + dp[1][5] * dp[0][2] + dp[1][4] * dp[0][3] + dp[1][3] * dp[0][4] + dp[1][2] * dp[0][5] + dp[1][1] * dp[0][6] + dp[1][0] * dp[0][7]
    //            k個の多項式になる
    // dp[0][9] = dp[1][9] * dp[0][-1] + dp[1][8] * dp[0][0] + dp[1][7] * dp[0][1] + dp[1][6] * dp[0][2] + dp[1][5] * dp[0][3] + dp[1][4] * dp[0][4] + dp[1][3] * dp[0][5] + dp[1][2] * dp[0][6] + dp[1][1] * dp[0][7] + dp[1][0] * dp[0][8]
    //            ~~~~(=0)               ~~~~(=0)
    //          = dp[1][7] * dp[0][1] + dp[1][6] * dp[0][2] + dp[1][5] * dp[0][3] + dp[1][4] * dp[0][4] + dp[1][3] * dp[0][5] + dp[1][2] * dp[0][6] + dp[1][1] * dp[0][7] + dp[1][0] * dp[0][8]
    // ....
    // dp[0][n] = dp[1][n] * dp[0][-1] + dp[1][n-1] * dp[0][0] + dp[1][n-2] * dp[0][1] + ... + dp[1][0] * dp[0][n-1]
    //          = dp[1][7] * dp[0][n-8] + dp[1][6] * dp[0][n-7] + ... + dp[1][0] * dp[0][n-1]
    
    // m == -1のとき、dp[k][-1] = 1; // ({}) 存在しないけど、こう定義しておくと計算が楽
    // m == 0 のとき、dp[k][0] = 1; // ({})
    // m == 1 のとき、dp[k][1] = 1; // ({k})
    // m > 1のとき、dp[k][m] = 0; k * m > k * 1 = kなので。
    // p = m + 1 で定義しておく。
    dp[k].push(1); // m = -1 のとき。 ここまでの議論の dp[k][-1] を、実装上はdp[k][0] でアクセスする。(-1のインデックスがないので、1個ずらしてm -> m+1とする)
    dp[k].push(1); // m =  0 のとき。 ここまでの議論の dp[k][0]  を、実装上はdp[k][1] でアクセスする。
    dp[k].push(1); // m =  1 のとき。 ここまでの議論の dp[k][1]  を、実装上はdp[k][2] でアクセスする。
    for h in (0..k).rev() {
        dp[h].push(1); // m = -1 のとき。 dp[h][-1]は、実装上はdp[h][0]となるので注意。
        if h != 0 {
            for m in 0..k/h+1 {
                let mut dp_h_m = 0;
                for i in 0..m+1 {
                    if dp[h+1].len() <= m-i+1 {continue}
                    // dp[h][m] にアクセスしたいとき、実装上はdp[h][m+1]にアクセスしないといけないことに注意。
                    dp_h_m += dp[h+1][m-i+1] * dp[h][i];
                    dp_h_m %= modulus;
                }
                dp[h].push(dp_h_m);
            }
        }
        // h = 0のときは、k/h が 0 除算になってしまうなので場合分けした。k/h+1回分ではなく、n+1回分の計算が必要。
        else if h == 0 {
            for m in 0..n+1 {
                let mut dp_h_m = 0;
                // 本当は、m <= k と m > k で場合分けしたいが。m > k のときは、k回の計算で済むので。
                for i in 0..m+1 {
                    if dp[h+1].len() <= m-i+1 {continue}
                    dp_h_m += dp[h+1][m-i+1] * dp[h][i];
                    dp_h_m %= modulus;
                }
                dp[h].push(dp_h_m);
            }
        }
    }
    // debug
    // for i in (0..dp.len()).rev() {
    //     println!("dp[{}]={:?}", i, dp[i]);
    // }
    // 回答
    println!("{}", dp[0][n+1]);

    // bostan-mori法をどこに適応することを考える。
    // bostan-mori法は漸化式の一般項を解く
    //     dp[L][H] = a_Lとする (Hは無視. H+1の項は係数だと思え)
    //     aL = a-1 + a0 + a1 + .... aL-1 * H
    //     漸化式やな。。。 L+2項間漸化式やん
    //     k項間漸化式は、　O(d*logd*logN)　で解ける

    
}

fn full_task(n: usize, k: usize) {
    let modulus: isize = 998244353;
    let root: Vec<isize> = make_root(modulus);
    let invroot: Vec<isize> = make_invroot(&root, modulus);

    // dp[h][m] := 条件を満たす長さmの数列で、値がすべてh以上であるものの個数
    // 最終的に知りたいのは、dp[0][n]の値が知りたい...!
    let mut dp = vec![vec![]; k+1];

    // 初期化
    dp[k].push(1); // m = -1 のとき。 ここまでの議論の dp[k][-1] を、実装上はdp[k][0] でアクセスする。(-1のインデックスがないので、1個ずらしてm -> m+1とする)
    dp[k].push(1); // m =  0 のとき。 ここまでの議論の dp[k][0]  を、実装上はdp[k][1] でアクセスする。
    dp[k].push(1); // m =  1 のとき。 ここまでの議論の dp[k][1]  を、実装上はdp[k][2] でアクセスする。

    let mut ans = 0;
    for h in (0..k).rev() {
        // println!("---- h = {} ----", h);
        // (1) h != 0 のときは、NTT(数論変換) を使う。各hに対して、O(k/h)log(k/h)で計算可能
        if h != 0 {
            // E8の理論(k+1項間漸化式が与えられていた時、初項から第m項(a0, a1, ..., am)までの係数がx^(m+k)を商で求まる。)
            let m = k/h; // m+2項間漸化式を求める
            // dp[h][m] = dp[h+1][m] * dp[h][-1] + ... + dp[h+1][0] * dp[h][m-1]
            let mut q_x = vec![0; m+2];
            // Q(x) = x^(m+1) - dp[h+1][m] * x^m - dp[h+1][m-1] * x^(m-1) - ... - dp[h+1][0] * x^0  <- dp[h+1]は初項がdp[h+1][-1]なことに注意
            // q_x  = [- dp[h+1][m], - dp[h+1][m-1], ..., - dp[h+1][1], - dp[h+1][0], 1]            <- 長さm+2個の多項式
            q_x[m+1] = 1; // 末項は最高次の係数で常に1
            for i in 1..dp[h+1].len() {
                q_x[m+1-i] -= dp[h+1][i]; // ※ q_x の後ろから2番目の項は、概念上dp[h+1][0]だが、実装上dp[h+1][1]なことに注意
            }
            // // m+2項間漸化式の、a_0~a_mまで求めたいのだから、
            // // k = m + 1
            // // m = m
            // // x^(m+k) = x^(2m + 1)を q_x で割った商が知りたい
            let mut numerator = vec![0; 2*m + 3]; // 分子
            let last_index = numerator.len() - 1;
            numerator[last_index] = 1;
            // println!("numerator = {:?}", numerator);
            // println!("q(x) = {:?}", q_x);
            dp[h] = fps_division(&numerator, &q_x, &root, &invroot); // [am, am-1, ..., a0]
            
            // dp[h] = fps_inv(&q_x, m as isize + 2, &root, &invroot, modulus); <- 他の人は、割り算ではなく、ただの逆元をかけていたので参考にしたが全然答え合わない..
            dp[h].reverse(); // 知りたいのは、[dp[h][-1], dp[h][0], ..., dp[h][m]] = [a0, a1, ..., am]
            // println!("dp[{}] = {:?}", h, dp[h]);
        }
        // h = 0のときは、k/h が 0 除算になってしまうなので場合分けした。k/h+1回分ではなく、n+1回分の計算が必要。
        // (2) h == 0 のときは、Bostan-Mori法を使う
        else if h == 0 {
            let m = k;
            let mut q_x = vec![0; m+2];
            q_x[m+1] = 1; // 末項は1
            for i in 1..dp[h+1].len() {
                q_x[m+1-i] -= dp[h+1][i]; // ※ q_x の後ろから2番目の項は、概念上dp[h+1][0]だが、実装上dp[h+1][1]なことに注意
            }
            // let mut numerator = vec![0; 2*m + 3];
            // let last_index = numerator.len() - 1;
            // numerator[last_index] = 1;
            // dp[h] = fps_division(&numerator, &q_x, &root, &invroot);
            // dp[h].reverse();
            // dp[h].truncate(q_x.len() -1); // bostan_moriに入力する数列は、[a_0, ..., a_d-1]で良い。(dp[0]は[dp[0][-1], ..., dp[0][m]]でm+2個格納されているので1個多い。)
            // println!("dp[{}] = {:?}", h, dp[h]);
            q_x.reverse(); // bostan_moriのq_xはx^0の係数が1, E8の定理はx^kの係数が1で逆なので注意
            // println!("q(x) = {:?}", q_x);
            
            // 他の人のbostan mori
            // let dph_2: Vec<u32> = dp[h].iter().map(|&x| ((x + modulus) % modulus) as u32).collect();
            // let q_x2: Vec<u32> = q_x.iter().map(|&x| ((x + modulus) % modulus) as u32).collect();
            // let ans = Fps::bostan_mori(&vec![1], &q_x2, n as u64 + 1);
            
            // debug
            // for i in 0..q_x.len() {
            //     if q_x[i] < 0 {
            //         q_x[i] = modulus + q_x[i];
            //     }
            // }
            // println!("q_x = {:?}", q_x);
            let mut p_x = vec![1];
            ans = bostan_mori(n+1, p_x, q_x);
            // ans = bostan_mori(n+1, dp[h].clone(), q_x);
            println!("{}", ans);
        }
    }
    // for i in (0..dp.len()).rev() {
    //     println!("dp[{}]={:?}", i, dp[i]);
    // }
    // println!("{}", ans);
    
}

// 小課題3: 1 <= N <= 6, 1 <= K <= 6
// fn subtask3(n: usize, k: usize) {
// }

fn multiply_matrix(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, modulus: usize) -> Vec<Vec<usize>> {
    let h = a.len();
    let w = b[0].len();
    let common = a[0].len(); // == b.len()

    let mut c = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            for k in 0..common {
                c[i][j] += a[i][k] * b[k][j];
                c[i][j] %= modulus;
            }
        }
    }
    return c
}

fn power_of_matrix(mut matrix: Vec<Vec<usize>>, mut exponent: usize, modulus: usize) -> Vec<Vec<usize>> {
    let mut answer = vec![
        vec![1, 0],
        vec![0, 1],
    ];
    while exponent >= 1 {
        if exponent % 2 == 1 {
            answer = multiply_matrix(&matrix, &answer, modulus);
        }
        matrix = multiply_matrix(&matrix, &matrix, modulus);
        exponent = exponent /2;
    }
    return answer
}


// habara_k 氏の bostan_mori の確認 https://atcoder.jp/contests/typical90/submissions/30533869
// my_library_rs {{{
#[allow(dead_code)]
mod my_library_rs {
    mod algebra {
        use std::marker::PhantomData;
        // ac-library-rs と同じ形式
        pub trait Monoid {
            type S: Clone;
            fn identity() -> Self::S;
            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
        }
        pub trait MapMonoid {
            type M: Monoid;
            type F: Clone;
            // type S = <Self::M as Monoid>::S;
            fn identity_element() -> <Self::M as Monoid>::S {
                Self::M::identity()
            }
            fn binary_operation(
                a: &<Self::M as Monoid>::S,
                b: &<Self::M as Monoid>::S,
            ) -> <Self::M as Monoid>::S {
                Self::M::binary_operation(a, b)
            }
            fn identity_map() -> Self::F;
            fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
            fn composition(f: &Self::F, g: &Self::F) -> Self::F;
        }
        pub struct NullMapMonoid<M> {
            _phantom: PhantomData<M>,
        }
        impl<M: Monoid> MapMonoid for NullMapMonoid<M> {
            type M = M;
            type F = ();
            fn identity_map() -> Self::F {
                ()
            }
            fn mapping(_f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
                x.clone()
            }
            fn composition(_f: &Self::F, _g: &Self::F) -> Self::F {
                ()
            }
        }
    }
    mod convolution {
        use crate::my_library_rs::modulo::*;
        fn ceil_log2(n: usize) -> usize {
            debug_assert!(n > 0);
            32 - (n as u32 - 1).leading_zeros() as usize
        }
        const BLOCK: usize = 2;
        pub fn butterfly<M: Modulus>(a: &mut [u32]) {
            let n = a.len();
            debug_assert!(n.is_power_of_two());
            let h = ceil_log2(n);
            for len in 0..h {
                let p = 1 << h - 1 - len;
                // for j in 0..p%BLOCK {
                //     let (x, y) = (a[j], a[j + p]);
                //     a[j] = M::add(x, y);
                //     a[j + p] = M::sub(x, y);
                // }
                // for j in (p%BLOCK..p).step_by(BLOCK) {
                //     let (x, y) = (a[j], a[j + p]);
                //     a[j] = M::add(x, y);
                //     a[j + p] = M::sub(x, y);
                //     let (x, y) = (a[j+1], a[j+1 + p]);
                //     a[j+1] = M::add(x, y);
                //     a[j+1 + p] = M::sub(x, y);
                // }
                for j in 0..p {
                    let (x, y) = (a[j], a[j + p]);
                    a[j] = M::add(x, y);
                    a[j + p] = M::sub(x, y);
                }
                let mut rot = M::ROT[0];
                for s in 1..1 << len {
                    let offset = s << h - len;
                    // for j in 0..p%BLOCK {
                    //     let (x, y) = (a[j + offset], M::mul(a[j + offset + p], rot));
                    //     a[j + offset] = M::add(x, y);
                    //     a[j + offset + p] = M::sub(x, y);
                    // }
                    // for j in (p%BLOCK..p).step_by(BLOCK) {
                    //     let (x, y) = (a[j + offset], M::mul(a[j + offset + p], rot));
                    //     a[j + offset] = M::add(x, y);
                    //     a[j + offset + p] = M::sub(x, y);
                    //     let (x, y) = (a[j+1 + offset], M::mul(a[j+1 + offset + p], rot));
                    //     a[j+1 + offset] = M::add(x, y);
                    //     a[j+1 + offset + p] = M::sub(x, y);
                    // }
                    for j in offset..offset + p {
                        let (x, y) = (a[j], M::mul(a[j + p], rot));
                        a[j] = M::add(x, y);
                        a[j + p] = M::sub(x, y);
                    }
                    rot = M::mul(rot, M::ROT[(!s).trailing_zeros() as usize]);
                }
            }
        }
        pub fn butterfly_inv<M: Modulus>(a: &mut [u32]) {
            let n = a.len();
            debug_assert!(n.is_power_of_two());
            let h = ceil_log2(n);
            for len in (0..h).rev() {
                let p = 1 << h - 1 - len;
                for j in 0..p {
                    let (x, y) = (a[j], a[j + p]);
                    a[j] = M::add(x, y);
                    a[j + p] = M::sub(x, y);
                }
                let mut rot = M::INV_ROT[0];
                for s in 1..1 << len {
                    let offset = s << h - len;
                    for j in offset..offset + p {
                        let (x, y) = (a[j], a[j + p]);
                        a[j] = M::add(x, y);
                        a[j + p] = M::mul(M::sub(x, y), rot);
                    }
                    rot = M::mul(rot, M::INV_ROT[(!s).trailing_zeros() as usize]);
                }
            }
        }
        pub fn butterfly_doubling<M: Modulus>(a: &mut Vec<u32>) {
            let n = a.len();
            let h = ceil_log2(n) + 1;
            let mut b = a.clone();
            butterfly_inv::<M>(&mut b);
            let mut rot = M::inv(n as u32);
            for e in b.iter_mut() {
                *e = M::mul(*e, rot);
                rot = M::mul(rot, M::BASE[h]);
            }
            butterfly::<M>(&mut b);
            a.extend(b);
        }
        pub fn convolution<M: Modulus>(a: &[u32], b: &[u32]) -> Vec<u32> {
            if a.is_empty() || b.is_empty() {
                return vec![];
            }
            let (n, m) = (a.len(), b.len());
            let (mut a, mut b) = (a.to_owned(), b.to_owned());
            let z = 1 << ceil_log2(n + m - 1);
            a.resize(z, 0);
            b.resize(z, 0);
            butterfly::<M>(&mut a);
            butterfly::<M>(&mut b);
            for (e, &x) in a.iter_mut().zip(b.iter()) {
                *e = M::mul(*e, x);
            }
            butterfly_inv::<M>(&mut a);
            a.resize(n + m - 1, 0);
            let iz = M::inv(z as u32);
            for e in a.iter_mut() {
                *e = M::mul(*e, iz);
            }
            a
        }
    }
    mod fps {
        //use std::arch::x86_64::*;
        use std::cmp::min;
        use std::marker::PhantomData;
        use std::ops::{Deref, DerefMut, DivAssign, MulAssign};
        use crate::my_library_rs::convolution::{butterfly, butterfly_doubling, butterfly_inv};
        use crate::my_library_rs::modulo::{Mod998244353, Modulus};
        pub struct Fps<M: Modulus>(Vec<u32>, PhantomData<M>);
        pub type Fps998244353 = Fps<Mod998244353>;
        impl<M: Modulus> From<Vec<u32>> for Fps<M> {
            fn from(v: Vec<u32>) -> Self {
                Self(v, PhantomData::<M>)
            }
        }
        impl<M: Modulus> Deref for Fps<M> {
            type Target = Vec<u32>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<M: Modulus> DerefMut for Fps<M> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<M: Modulus> Clone for Fps<M> {
            fn clone(&self) -> Self {
                Self(self.0.clone(), PhantomData::<M>)
            }
        }
        impl<M: Modulus> MulAssign<u32> for Fps<M> {
            fn mul_assign(&mut self, rhs: u32) {
                self.iter_mut().for_each(|e| *e = M::mul(*e, rhs));
            }
        }
        impl<M: Modulus> DivAssign<u32> for Fps<M> {
            fn div_assign(&mut self, mut rhs: u32) {
                rhs = M::inv(rhs);
                self.iter_mut().for_each(|e| *e = M::mul(*e, rhs));
            }
        }
        fn ceil_log2(n: usize) -> usize {
            debug_assert!(n > 0);
            32 - (n as u32 - 1).leading_zeros() as usize
        }
        impl<M: Modulus> Fps<M> {
            pub fn inv(&self, d: usize) -> Self {
                debug_assert!(d > 0);
                debug_assert!(self.len() > 0);
                let mut inv = vec![M::inv(self[0])];
                for m in (0..).map(|i| 1 << i).take_while(|&m| m < d) {
                    let mut f = self[0..min(2 * m, self.len())].to_owned();
                    let mut g = inv.clone();
                    f.resize(2 * m, 0);
                    g.resize(2 * m, 0);
                    butterfly::<M>(&mut f);
                    butterfly::<M>(&mut g);
                    for (a, &b) in f.iter_mut().zip(&g) {
                        *a = M::mul(*a, b);
                    }
                    butterfly_inv::<M>(&mut f);
                    f.drain(0..m);
                    f.resize(2 * m, 0);
                    butterfly::<M>(&mut f);
                    for (a, &b) in f.iter_mut().zip(&g) {
                        *a = M::mul(*a, b);
                    }
                    butterfly_inv::<M>(&mut f);
                    let iz = M::inv(2 * m as u32);
                    let iz = M::neg(M::mul(iz, iz));
                    for &a in &f[0..min(d - inv.len(), m)] {
                        inv.push(M::mul(a, iz));
                    }
                }
                debug_assert_eq!(inv.len(), d);
                inv.into()
            }
            pub fn bostan_mori(p: &[u32], q: &[u32], mut k: u64) -> u32 {
                let (mut p, mut q): (Self, Self) = (p.to_owned().into(), q.to_owned().into());
                let n = 1 << ceil_log2(p.len() + q.len() - 1);
                p.resize(n, 0);
                q.resize(n, 0);
                butterfly::<M>(&mut p);
                butterfly::<M>(&mut q);
                while k >= n as u64 {
                    butterfly_doubling::<M>(&mut p);
                    butterfly_doubling::<M>(&mut q);
                    if (k & 1) == 0 {
                        for s in 0..n {
                            p[s] = M::div2(M::add(
                                M::mul(p[2 * s], q[2 * s + 1]),
                                M::mul(p[2 * s + 1], q[2 * s]),
                            ));
                        }
                    } else {
                        let mut rot = M::inv(2);
                        for s in 0..n {
                            p[s] = M::mul(
                                M::sub(
                                    M::mul(p[2 * s], q[2 * s + 1]),
                                    M::mul(p[2 * s + 1], q[2 * s]),
                                ),
                                rot,
                            );
                            rot = M::mul(rot, M::INV_ROT[(!s).trailing_zeros() as usize]);
                        }
                    }
                    p.truncate(n);
                    for i in 0..n {
                        q[i] = M::mul(q[2 * i], q[2 * i + 1]);
                    }
                    q.truncate(n);
                    k >>= 1;
                }
                butterfly_doubling::<M>(&mut p);
                butterfly_inv::<M>(&mut q);
                q = Self::inv(&q, n);
                q.resize(2 * n, 0);
                butterfly::<M>(&mut q);
                for (e, &x) in q.iter_mut().zip(p.iter()) {
                    *e = M::div2(M::mul(*e, x));
                }
                butterfly_inv::<M>(&mut q);
                q[k as usize]
            }
        }
    }
    mod geometry {
        use std::iter::Sum;
        use std::ops::{Mul, Sub};
        #[derive(Clone, Copy, Eq, PartialEq, Debug)]
        pub struct Point<T: Copy> {
            pub x: T,
            pub y: T,
        }
        impl<T: Copy> Point<T> {
            pub fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }
        impl<T: Copy + Sub<Output = T>> Sub for Point<T> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
        pub fn cross<T>(a: Point<T>, b: Point<T>) -> T
        where
            T: Copy + Mul<Output = T> + Sub<Output = T>,
        {
            a.x * b.y - a.y * b.x
        }
        pub fn convex_hull<T>(ps: &[Point<T>]) -> Vec<Point<T>>
        where
            T: Copy + Ord + From<i32> + Sub<Output = T> + Mul<Output = T>,
        {
            if ps.len() <= 1 {
                return ps.to_vec();
            }
            let mut order: Vec<usize> = (0..ps.len()).collect();
            order.sort_by_key(|&i| (ps[i].x, ps[i].y));
            let mut ch = vec![];
            for &i in order.iter() {
                while ch.len() >= 2
                    && cross(
                        ps[i] - ch[ch.len() - 1],
                        ch[ch.len() - 1] - ch[ch.len() - 2],
                    ) >= T::from(0)
                {
                    ch.pop();
                }
                ch.push(ps[i]);
            }
            let n = ch.len();
            for &i in order.iter().rev().skip(1) {
                while ch.len() > n
                    && cross(
                        ps[i] - ch[ch.len() - 1],
                        ch[ch.len() - 1] - ch[ch.len() - 2],
                    ) >= T::from(0)
                {
                    ch.pop();
                }
                ch.push(ps[i]);
            }
            ch.pop();
            ch
        }
        pub fn area_x2<T>(ps: &[Point<T>]) -> T
        where
            T: Copy + Mul<Output = T> + Sub<Output = T> + Sum,
        {
            let n = ps.len();
            (0..n).map(|i| cross(ps[i], ps[(i + 1) % n])).sum::<T>()
        }
    }
    mod graph {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::ops::Add;
        #[derive(Clone)]
        pub struct Edge<T> {
            pub to: usize,
            pub cost: T,
        }
        pub fn dijkstra<T>(graph: &[Vec<Edge<T>>], s: usize, max: T) -> Vec<T>
        where
            T: Copy + From<usize> + Ord + Add<Output = T>,
        {
            let mut dist = vec![max; graph.len()];
            dist[s] = T::from(0);
            let mut heap = BinaryHeap::new();
            heap.push((Reverse(dist[s]), s));
            while let Some((Reverse(d), u)) = heap.pop() {
                if d > dist[u] {
                    continue;
                }
                for e in graph[u].iter() {
                    if dist[e.to] > d + e.cost {
                        dist[e.to] = d + e.cost;
                        heap.push((Reverse(dist[e.to]), e.to));
                    }
                }
            }
            dist
        }
    }
    mod macros {
        // https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7
        #[macro_export]
        macro_rules! chmin {
                ($base:expr, $($cmps:expr),+ $(,)*) => {{
                    let cmp_min = min!($($cmps),+);
                    if $base > cmp_min {
                        $base = cmp_min;
                        true
                    } else {
                        false
                    }
                }};
            }
        #[macro_export]
        macro_rules! chmax {
                ($base:expr, $($cmps:expr),+ $(,)*) => {{
                    let cmp_max = max!($($cmps),+);
                    if $base < cmp_max {
                        $base = cmp_max;
                        true
                    } else {
                        false
                    }
                }};
            }
        #[macro_export]
        macro_rules! min {
                ($a:expr $(,)*) => {{
                    $a
                }};
                ($a:expr, $b:expr $(,)*) => {{
                    std::cmp::min($a, $b)
                }};
                ($a:expr, $($rest:expr),+ $(,)*) => {{
                    std::cmp::min($a, min!($($rest),+))
                }};
            }
        #[macro_export]
        macro_rules! max {
                ($a:expr $(,)*) => {{
                    $a
                }};
                ($a:expr, $b:expr $(,)*) => {{
                    std::cmp::max($a, $b)
                }};
                ($a:expr, $($rest:expr),+ $(,)*) => {{
                    std::cmp::max($a, max!($($rest),+))
                }};
            }
    }
    mod math {
        use std::convert::From;
        use std::ops::{AddAssign, Div, Mul, SubAssign};
        pub struct Combination<T> {
            pub fact: Vec<T>,
            pub finv: Vec<T>,
        }
        impl<T> Combination<T>
        where
            T: Copy + From<u32> + Mul<Output = T> + Div<Output = T>,
        {
            pub fn new(n: usize) -> Self {
                let (mut fact, mut finv) = (Vec::with_capacity(n + 1), Vec::with_capacity(n + 1));
                fact.push(T::from(1));
                for i in 0..n {
                    fact.push(fact[i] * T::from((i + 1) as u32));
                }
                finv.push(T::from(1) / fact[n]);
                for i in 0..n {
                    finv.push(finv[i] * T::from((n - i) as u32));
                }
                finv.reverse();
                Self { fact, finv }
            }
            #[allow(non_snake_case)]
            pub fn C(&self, n: usize, r: usize) -> T {
                if n < r {
                    return T::from(0);
                }
                self.fact[n] * self.finv[r] * self.finv[n - r]
            }
            #[allow(non_snake_case)]
            pub fn P(&self, n: usize, r: usize) -> T {
                if n < r {
                    return T::from(0);
                }
                self.fact[n] * self.finv[n - r]
            }
        }
        fn bitwise_transform<T>(a: &mut [T], f: fn(*mut T, *mut T)) {
            let n = a.len();
            assert_eq!(n & (n - 1), 0);
            let ptr = a.as_mut_ptr();
            for block in (0..).map(|k| 1 << k).take_while(|&b| b < n) {
                for l in (0..n).step_by(block << 1) {
                    for i in l..l + block {
                        unsafe {
                            f(ptr.add(i), ptr.add(i + block));
                        }
                    }
                }
            }
        }
        pub fn subset_zeta_transform<T>(a: &mut [T])
        where
            T: Copy + AddAssign,
        {
            bitwise_transform(a, |x: *mut T, y: *mut T| unsafe {
                *y += *x;
            });
        }
        pub fn lagrange_polynomial<
            T: Clone
                + Copy
                + From<i64>
                + From<u32>
                + Mul<Output = T>
                + Div<Output = T>
                + SubAssign
                + AddAssign,
        >(
            f: &[T],
            t: i64,
        ) -> T {
            let n = f.len() - 1;
            if t <= n as i64 {
                return f[t as usize];
            }
            let fact = Combination::new(n);
            let mut lp = vec![T::from(1u32); n + 1];
            let mut rp = vec![T::from(1u32); n + 1];
            for i in 0..n {
                lp[i + 1] = lp[i] * T::from(t - i as i64);
            }
            for i in (1..=n).rev() {
                rp[i - 1] = rp[i] * T::from(t - i as i64);
            }
            let mut ans = T::from(0u32);
            for i in 0..=n {
                let x = f[i] * fact.finv[i] * fact.finv[n - i] * lp[i] * rp[i];
                if ((n - i) & 1) == 1 {
                    ans -= x;
                } else {
                    ans += x;
                }
            }
            ans
        }
    }
    mod modulo {
        pub trait Modulus {
            const VALUE: u32;
            const PRIMITIVE_ROOT: u32;
            const BASE: [u32; 30];
            const ROT: [u32; 30];
            const INV_ROT: [u32; 30];
            fn mul(a: u32, b: u32) -> u32 {
                debug_assert!(a < Self::VALUE);
                debug_assert!(b < Self::VALUE);
                ((a as u64 * b as u64) % Self::VALUE as u64) as u32
            }
            fn add(mut a: u32, b: u32) -> u32 {
                debug_assert!(a < Self::VALUE);
                debug_assert!(b < Self::VALUE);
                a += b;
                if a >= Self::VALUE {
                    a -= Self::VALUE;
                }
                a
            }
            fn neg(a: u32) -> u32 {
                debug_assert!(a < Self::VALUE);
                if a == 0 {
                    0
                } else {
                    Self::VALUE - a
                }
            }
            fn sub(mut a: u32, b: u32) -> u32 {
                debug_assert!(a < Self::VALUE);
                debug_assert!(b < Self::VALUE);
                a += Self::VALUE - b;
                if a >= Self::VALUE {
                    a -= Self::VALUE;
                }
                a
            }
            fn pow(mut a: u32, mut n: u32) -> u32 {
                debug_assert!(a < Self::VALUE);
                let mut r = 1;
                while n > 0 {
                    if n & 1 == 1 {
                        r = Self::mul(r, a);
                    }
                    a = Self::mul(a, a);
                    n >>= 1;
                }
                r
            }
            fn inv(a: u32) -> u32 {
                debug_assert!(a < Self::VALUE);
                Self::pow(a, Self::VALUE - 2)
            }
            fn div2(a: u32) -> u32 {
                debug_assert!(a < Self::VALUE);
                (a + (a & 1) * Self::VALUE) >> 1
            }
        }
        pub struct Mod998244353 {}
        impl Modulus for Mod998244353 {
            const VALUE: u32 = 998244353;
            const PRIMITIVE_ROOT: u32 = 3;
            const BASE: [u32; 30] = [
                1, 998244352, 911660635, 372528824, 929031873, 452798380, 922799308, 781712469, 476477967,
                166035806, 258648936, 584193783, 63912897, 350007156, 666702199, 968855178, 629671588,
                24514907, 996173970, 363395222, 565042129, 733596141, 267099868, 15311432, 0, 0, 0, 0, 0,
                0,
            ];
            // root = pow(3, (998244353-1) >> 23)
            // BASE[0] = pow(root, 1<<23);
            // BASE[1] = pow(root, 1<<22);
            // ...
            // BASE[23] = pow(root, 1<<0);
            const ROT: [u32; 30] = [
                911660635, 509520358, 369330050, 332049552, 983190778, 123842337, 238493703, 975955924,
                603855026, 856644456, 131300601, 842657263, 730768835, 942482514, 806263778, 151565301,
                510815449, 503497456, 743006876, 741047443, 56250497, 867605899, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            // ROT[0] = pow(root, 1<<21)
            // ROT[1] = pow(root, (1<<20) - (1<<21))
            // ...
            // ROT[21] = pow(root, (1<<0) - (1<<1) - ... - (1<<21))
            const INV_ROT: [u32; 30] = [
                86583718, 372528824, 373294451, 645684063, 112220581, 692852209, 155456985, 797128860,
                90816748, 860285882, 927414960, 354738543, 109331171, 293255632, 535113200, 308540755,
                121186627, 608385704, 438932459, 359477183, 824071951, 103369235, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            // INV_ROT[i] * ROT[i] = 1, 0 <= i <= 21
        }
    }
    mod rbtree {
        use std::mem::replace;
        use std::rc::Rc;
        use crate::my_library_rs::algebra::{MapMonoid, Monoid};
        const RED: bool = false;
        const BLACK: bool = true;
        pub struct RbNode<F: MapMonoid> {
            l: Option<Rc<Self>>,
            r: Option<Rc<Self>>,
            black: bool,
            height: usize,
            size: usize,
            val: <F::M as Monoid>::S,
            lazy: F::F,
            rev: bool,
        }
        impl<F: MapMonoid> RbNode<F> {
            fn new(l: Rc<Self>, r: Rc<Self>, black: bool) -> Rc<Self> {
                Rc::new(Self {
                    black,
                    height: l.height + black as usize,
                    size: l.size + r.size,
                    val: F::binary_operation(&l.val, &r.val),
                    lazy: F::identity_map(),
                    rev: false,
                    l: Some(l),
                    r: Some(r),
                })
            }
            fn new_leaf(val: <F::M as Monoid>::S) -> Rc<Self> {
                Rc::new(Self {
                    black: true,
                    height: 1,
                    size: 1,
                    val,
                    lazy: F::identity_map(),
                    rev: false,
                    l: None,
                    r: None,
                })
            }
            fn detach(p: Rc<Self>) -> (Rc<Self>, Rc<Self>) {
                let (mut l, mut r) = (p.l.clone().unwrap(), p.r.clone().unwrap());
                Rc::make_mut(&mut l).val = F::mapping(&p.lazy, &l.val);
                Rc::make_mut(&mut r).val = F::mapping(&p.lazy, &r.val);
                Rc::make_mut(&mut l).lazy = F::composition(&p.lazy, &l.lazy);
                Rc::make_mut(&mut r).lazy = F::composition(&p.lazy, &r.lazy);
                if p.rev {
                    Rc::make_mut(&mut l).rev ^= true;
                    Rc::make_mut(&mut r).rev ^= true;
                    return (r, l);
                }
                (l, r)
            }
            fn make_root(mut p: Rc<Self>) -> Rc<Self> {
                if !p.black {
                    Rc::make_mut(&mut p).black = true;
                    Rc::make_mut(&mut p).height += 1;
                }
                p
            }
            fn val(&self) -> <F::M as Monoid>::S {
                self.val.clone()
            }
            pub fn merge_sub(a: Rc<Self>, b: Rc<Self>) -> Rc<Self> {
                debug_assert!(a.black);
                debug_assert!(b.black);
                if a.height < b.height {
                    let (l, r) = Self::detach(b);
                    //      b(BLACK,h+1)
                    //       /     \
                    //   l(*,h)    r(*,h)
                    if l.black {
                        // Connect directly:
                        //               (BLACK,h+1)
                        //                /        \
                        //   merge_sub(a,l)(*,h)   r(*,h)
                        return Self::new(Self::merge_sub(a, l), r, BLACK);
                    }
                    let (ll, lr) = Self::detach(l);
                    //           b(BLACK,h+1)
                    //           /      \
                    //       l(RED,h)   r(*,h)
                    //       /       \
                    //   ll(BLACK,h)  lr(BLACK,h)
                    let c = Self::merge_sub(a, ll);
                    if c.black {
                        // Connect directly:
                        //             (BLACK,h+1)
                        //             /    \
                        //         (RED,h)   r(BLACK,h)
                        //         /    \
                        //   c(BLACK,h)  lr(BLACK,h)
                        return Self::new(Self::new(c, lr, RED), r, BLACK);
                    }
                    return if r.black {
                        // Rotate tree:
                        //             (BLACK,h+1)                (BLACK,h+1)
                        //             /    \                     /     \
                        //         (RED,h)  r(BLACK,h)   =>   c(RED,h)  (RED,h)
                        //        /    \                                /    \
                        //   c(RED,h)   lr(BLACK,h)              lr(BLACK,h)  r(BLACK,h)
                        Self::new(c, Self::new(lr, r, RED), BLACK)
                    } else {
                        // Change color:
                        //             (BLACK,h+1)                   (RED,h+1)
                        //             /    \                        /       \
                        //         (RED,h)  r(RED,h)     =>      (BLACK,h+1)  r(BLACK,h+1)
                        //        /    \                          /     \
                        //   c(RED,h)   lr(BLACK,h)           c(RED,h)   lr(BLACK,h)
                        Self::new(Self::new(c, lr, BLACK), Self::make_root(r), RED)
                    };
                }
                if a.height > b.height {
                    // Do the reverse of the above procedure.
                    let (l, r) = Self::detach(a);
                    if r.black {
                        return Self::new(l, Self::merge_sub(r, b), BLACK);
                    }
                    let (rl, rr) = Self::detach(r);
                    let c = Self::merge_sub(rr, b);
                    if c.black {
                        return Self::new(l, Self::new(rl, c, RED), BLACK);
                    }
                    return if l.black {
                        Self::new(Self::new(l, rl, RED), c, BLACK)
                    } else {
                        Self::new(Self::make_root(l), Self::new(rl, c, BLACK), RED)
                    };
                }
                // Connect directly:
                //         (RED,h)
                //         /     \
                //   a(BLACK,h)  b(BLACK,h)
                Self::new(a, b, RED)
            }
            pub fn split_sub(p: Rc<Self>, k: usize) -> (Rc<Self>, Rc<Self>) {
                debug_assert!(0 < k && k < p.size);
                let (l, r) = Self::detach(p);
                if k < l.size {
                    let (a, b) = Self::split_sub(l, k);
                    return (a, Self::merge_sub(Self::make_root(b), Self::make_root(r)));
                    // 左側の返り値の検証
                    // (1) p の根が黒のとき
                    // split_sub の性質より
                    //     a.height ≦ l.height + 1 == p.height. OK
                    // (2) p の根が赤のとき
                    // l の根が黒なので, split_sub の性質より
                    //     a.height ≦ l.height == p.height. OK
                    //
                    // 右側の返り値の検証
                    // (1) p の根が黒のとき
                    // split_sub の性質より make_root(b.height) ≦ l.height + 1 == p.height
                    // これと make_root(r).height ≦ p.height より,
                    //     merge_sub(make_root(b), make_root(r)).height ≦ p.height. OK
                    //
                    // (2) p の根が赤のとき
                    // l の根が黒なので, split_subの性質より b.height ≦ l.height == p.height
                    //
                    // (2.1) make_root(b).height ≦ p.height のとき
                    // make_root(r).height == p.height と合わせて
                    //     merge_sub(make_root(b), make_root(r)).height ≦ p.height. OK
                    //
                    // (2.2) make_root(b).height == p.height + 1 のとき
                    // make_root(r).height == p.height < make_root(b).height
                    // これと b の左右の子が黒となることから, merge_sub の実装を読むと
                    // merge_sub(make_root(b), make_root(r)) の根は黒となることがわかる.
                    //     merge_sub(make_root(b), make_root(r)).height == p.height + 1 かつ
                    //     merge_sub(make_root(b), make_root(r)) の根は黒. OK
                }
                if k > l.size {
                    let (a, b) = Self::split_sub(r, k - l.size);
                    return (Self::merge_sub(Self::make_root(l), Self::make_root(a)), b);
                }
                // l.height ≦ p.height && r.height ≦ p.height. OK
                (l, r)
            }
            fn len(p: &Option<Rc<Self>>) -> usize {
                p.as_ref().map_or(0, |p| p.size)
            }
            fn merge(a: Option<Rc<Self>>, b: Option<Rc<Self>>) -> Option<Rc<Self>> {
                if a.is_none() {
                    return b;
                }
                if b.is_none() {
                    return a;
                }
                Some(Self::make_root(Self::merge_sub(a.unwrap(), b.unwrap())))
            }
            fn merge3(a: Option<Rc<Self>>, b: Option<Rc<Self>>, c: Option<Rc<Self>>) -> Option<Rc<Self>> {
                Self::merge(Self::merge(a, b), c)
            }
            fn split(p: Option<Rc<Self>>, k: usize) -> (Option<Rc<Self>>, Option<Rc<Self>>) {
                debug_assert!(k <= Self::len(&p));
                if k == 0 {
                    return (None, p);
                }
                if k == Self::len(&p) {
                    return (p, None);
                }
                let (l, r) = Self::split_sub(p.unwrap(), k);
                (Some(Self::make_root(l)), Some(Self::make_root(r)))
            }
            fn split3(
                p: Option<Rc<Self>>,
                l: usize,
                r: usize,
            ) -> (Option<Rc<Self>>, Option<Rc<Self>>, Option<Rc<Self>>) {
                debug_assert!(l <= r && r <= Self::len(&p));
                let (p, c) = Self::split(p, r);
                let (a, b) = Self::split(p, l);
                (a, b, c)
            }
            fn insert(p: Option<Rc<Self>>, k: usize, val: <F::M as Monoid>::S) -> Option<Rc<Self>> {
                debug_assert!(k <= Self::len(&p));
                let (a, b) = Self::split(p, k);
                Self::merge3(a, Some(Self::new_leaf(val)), b)
            }
            fn remove(p: Option<Rc<Self>>, k: usize) -> (Option<Rc<Self>>, <F::M as Monoid>::S) {
                debug_assert!(k < Self::len(&p));
                let (a, b, c) = Self::split3(p, k, k + 1);
                (Self::merge(a, c), b.unwrap().val())
            }
            fn build(v: &[<F::M as Monoid>::S], l: usize, r: usize) -> Option<Rc<Self>> {
                debug_assert!(l <= r && r <= v.len());
                if l == r {
                    return None;
                }
                if l + 1 == r {
                    return Some(Self::new_leaf(v[l].clone()));
                }
                Self::merge(
                    Self::build(v, l, (l + r) / 2),
                    Self::build(v, (l + r) / 2, r),
                )
            }
            fn is_leaf(&self) -> bool {
                self.black && self.height == 1
            }
            fn collect(mut p: Rc<Self>, v: &mut Vec<<F::M as Monoid>::S>) -> Rc<Self> {
                if !p.is_leaf() {
                    let black = p.black;
                    let (mut l, mut r) = Self::detach(p);
                    l = Self::collect(l, v);
                    r = Self::collect(r, v);
                    p = Self::new(l, r, black);
                } else {
                    v.push(p.val());
                }
                p
            }
            fn min_left<G: Fn(&<F::M as Monoid>::S) -> bool>(
                p: Rc<Self>,
                g: G,
                k: &mut usize,
                sm: <F::M as Monoid>::S,
            ) -> Rc<Self> {
                if p.is_leaf() {
                    if g(&F::binary_operation(&p.val(), &sm)) {
                        *k -= 1;
                    }
                    return p;
                }
                let black = p.black;
                let (mut l, mut r) = Self::detach(p);
                let nxt = F::binary_operation(&r.val(), &sm);
                if g(&nxt) {
                    *k -= r.size;
                    l = Self::min_left(l, g, k, nxt);
                } else {
                    r = Self::min_left(r, g, k, sm);
                }
                Self::new(l, r, black)
            }
            fn max_right<G: Fn(&<F::M as Monoid>::S) -> bool>(
                p: Rc<Self>,
                g: G,
                k: &mut usize,
                sm: <F::M as Monoid>::S,
            ) -> Rc<Self> {
                if p.is_leaf() {
                    if g(&F::binary_operation(&sm, &p.val())) {
                        *k += 1;
                    }
                    return p;
                }
                let black = p.black;
                let (mut l, mut r) = Self::detach(p);
                let nxt = F::binary_operation(&sm, &l.val());
                if g(&nxt) {
                    *k += l.size;
                    r = Self::max_right(r, g, k, nxt);
                } else {
                    l = Self::max_right(l, g, k, sm);
                }
                Self::new(l, r, black)
            }
            fn apply(mut p: Rc<Self>, f: F::F) -> Rc<Self> {
                Rc::make_mut(&mut p).val = F::mapping(&f, &p.val);
                Rc::make_mut(&mut p).lazy = F::composition(&f, &p.lazy);
                p
            }
            fn reverse(mut p: Rc<Self>) -> Rc<Self> {
                Rc::make_mut(&mut p).rev ^= true;
                p
            }
        }
        impl<F: MapMonoid> Clone for RbNode<F> {
            fn clone(&self) -> Self {
                Self {
                    l: self.l.clone(),
                    r: self.r.clone(),
                    black: self.black.clone(),
                    height: self.height.clone(),
                    size: self.size.clone(),
                    val: self.val.clone(),
                    lazy: self.lazy.clone(),
                    rev: self.rev.clone(),
                }
            }
        }
        pub struct RbTree<F: MapMonoid> {
            root: Option<Rc<RbNode<F>>>,
        }
        impl<F: MapMonoid> RbTree<F> {
            fn new(root: Option<Rc<RbNode<F>>>) -> Self {
                Self { root }
            }
            pub fn len(&mut self) -> usize {
                self.root.as_ref().map_or(0, |p| p.size)
            }
            pub fn merge(&mut self, other: &mut Self) {
                self.root = RbNode::<F>::merge(
                    replace(&mut self.root, None),
                    replace(&mut other.root, None),
                );
            }
            pub fn merge3(&mut self, b: &mut Self, c: &mut Self) {
                self.root = RbNode::<F>::merge3(
                    replace(&mut self.root, None),
                    replace(&mut b.root, None),
                    replace(&mut c.root, None),
                );
            }
            pub fn split(mut self, k: usize) -> (Self, Self) {
                debug_assert!(k <= self.len());
                let (l, r) = RbNode::<F>::split(replace(&mut self.root, None), k);
                (Self::new(l), Self::new(r))
            }
            pub fn split3(mut self, l: usize, r: usize) -> (Self, Self, Self) {
                debug_assert!(l <= r && r <= self.len());
                let (a, b, c) = RbNode::<F>::split3(replace(&mut self.root, None), l, r);
                (Self::new(a), Self::new(b), Self::new(c))
            }
            pub fn insert(&mut self, k: usize, val: <F::M as Monoid>::S) {
                debug_assert!(k <= self.len());
                self.root = RbNode::<F>::insert(replace(&mut self.root, None), k, val);
            }
            pub fn remove(&mut self, k: usize) -> <F::M as Monoid>::S {
                debug_assert!(k < self.len());
                let (root, val) = RbNode::<F>::remove(replace(&mut self.root, None), k);
                self.root = root;
                val
            }
            pub fn get(&mut self, k: usize) -> <F::M as Monoid>::S {
                debug_assert!(k < self.len());
                let val = self.remove(k);
                self.insert(k, val.clone());
                val
            }
            pub fn collect(&mut self) -> Vec<<F::M as Monoid>::S> {
                if self.len() == 0 {
                    return vec![];
                }
                let mut v = vec![];
                self.root = Some(RbNode::<F>::collect(
                    replace(&mut self.root, None).unwrap(),
                    &mut v,
                ));
                v
            }
            pub fn min_left<G: Fn(&<F::M as Monoid>::S) -> bool>(&mut self, r: usize, g: G) -> usize {
                debug_assert!(g(&<F::M as Monoid>::identity()));
                debug_assert!(r <= self.len());
                if r == 0 {
                    return r;
                }
                let (mut a, b) = RbNode::<F>::split(replace(&mut self.root, None), r);
                let mut k = r;
                a = Some(RbNode::<F>::min_left(
                    a.unwrap(),
                    g,
                    &mut k,
                    <F::M as Monoid>::identity(),
                ));
                self.root = RbNode::<F>::merge(a, b);
                k
            }
            pub fn max_right<G: Fn(&<F::M as Monoid>::S) -> bool>(&mut self, l: usize, g: G) -> usize {
                debug_assert!(g(&<F::M as Monoid>::identity()));
                debug_assert!(l <= self.len());
                if l == self.len() {
                    return l;
                }
                let (a, mut b) = RbNode::<F>::split(replace(&mut self.root, None), l);
                let mut k = l;
                b = Some(RbNode::<F>::max_right(
                    b.unwrap(),
                    g,
                    &mut k,
                    <F::M as Monoid>::identity(),
                ));
                self.root = RbNode::<F>::merge(a, b);
                k
            }
            pub fn prod(&mut self, l: usize, r: usize) -> <F::M as Monoid>::S {
                debug_assert!(l <= r && r <= self.len());
                if l == r {
                    return <F::M as Monoid>::identity();
                }
                let (a, b, c) = RbNode::<F>::split3(replace(&mut self.root, None), l, r);
                let val = b.as_ref().unwrap().val();
                self.root = RbNode::<F>::merge3(a, b, c);
                val
            }
            pub fn apply_range(&mut self, l: usize, r: usize, f: F::F) {
                debug_assert!(l <= r && r <= self.len());
                if l == r {
                    return;
                }
                let (a, mut b, c) = RbNode::<F>::split3(replace(&mut self.root, None), l, r);
                b = Some(RbNode::<F>::apply(b.unwrap(), f));
                self.root = RbNode::<F>::merge3(a, b, c);
            }
            pub fn reverse_range(&mut self, l: usize, r: usize) {
                debug_assert!(l <= r && r <= self.len());
                if l == r {
                    return;
                }
                let (a, mut b, c) = RbNode::<F>::split3(replace(&mut self.root, None), l, r);
                b = Some(RbNode::<F>::reverse(b.unwrap()));
                self.root = RbNode::<F>::merge3(a, b, c);
            }
        }
        impl<F: MapMonoid> Clone for RbTree<F> {
            fn clone(&self) -> Self {
                Self::new(self.root.clone())
            }
        }
        impl<F: MapMonoid> From<Vec<<F::M as Monoid>::S>> for RbTree<F> {
            fn from(v: Vec<<F::M as Monoid>::S>) -> Self {
                Self::new(RbNode::<F>::build(&v, 0, v.len()))
            }
        }
    }
    mod rolling_hash {
        use std::cell::RefCell;
        use std::time::{SystemTime, UNIX_EPOCH};
        pub struct RollingHash {
            hash: Vec<u64>,
            pow: Vec<u64>,
        }
        impl RollingHash {
            pub fn new(s: &[u8]) -> Self {
                let n = s.len();
                let (mut hash, mut pow) = (Vec::with_capacity(n + 1), Vec::with_capacity(n + 1));
                hash.push(0);
                pow.push(1);
                ROLLINGHASH_BASE.with(|b| {
                    let base = *b.borrow();
                    for i in 0..n {
                        hash.push(modulo(mul(hash[i], base) + s[i] as u64));
                        pow.push(mul(pow[i], base));
                    }
                });
                Self { hash, pow }
            }
            pub fn get(&self, l: usize, r: usize) -> u64 {
                modulo(self.hash[r] + MOD - mul(self.hash[l], self.pow[r - l]))
            }
        }
        const MOD: u64 = (1 << 61) - 1;
        fn mul(x: u64, y: u64) -> u64 {
            let t = x as u128 * y as u128;
            let t = (t >> 61) + (t & MOD as u128);
            modulo(t as u64)
        }
        fn modulo(x: u64) -> u64 {
            assert!(x < 2 * MOD);
            if x >= MOD {
                x - MOD
            } else {
                x
            }
        }
        thread_local!(static ROLLINGHASH_BASE: RefCell<u64> = {
            let t = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let t = (t >> 61) + (t & MOD as u128);
            RefCell::new(modulo(t as u64))
        });
    }
    mod tree {
        use std::mem::swap;
        pub struct HLD {
            pub visit: Vec<usize>, // DFSの行きがけ順. 0..n の並べ替え
            pub leave: Vec<usize>, // DFSの帰りがけ順. 0..n の並べ替え
            pub order: Vec<usize>, // order[k] := DFSでk番目に訪れる頂点. order[visit[v]] = v, forall v
            pub head: Vec<usize>,  // head[v] := 頂点v を含む heavy path の先頭
            pub size: Vec<usize>,  // 部分木の頂点数
            pub par: Vec<usize>,   // 親頂点
            pub depth: Vec<usize>, // 根からの深さ
        }
        impl HLD {
            pub const NULL: usize = std::usize::MAX;
            pub fn new(g: &[Vec<usize>], root: usize) -> Self {
                let n = g.len();
                let mut hld = HLD {
                    visit: vec![0; n],
                    leave: vec![0; n],
                    order: vec![0; n],
                    head: vec![0; n],
                    size: vec![1; n],
                    par: vec![0; n],
                    depth: vec![0; n],
                };
                hld.build(g, root);
                hld
            }
            fn build(&mut self, g: &[Vec<usize>], root: usize) {
                self.dfs(g, root, Self::NULL, 0);
                self.hld(g, root, root, &mut 0);
            }
            fn dfs(&mut self, g: &[Vec<usize>], u: usize, p: usize, d: usize) {
                self.par[u] = p;
                self.depth[u] = d;
                g[u].iter().filter(|&v| *v != p).for_each(|&v| {
                    self.dfs(g, v, u, d + 1);
                    self.size[u] += self.size[v];
                });
            }
            fn hld(&mut self, g: &[Vec<usize>], u: usize, h: usize, t: &mut usize) {
                self.head[u] = h;
                self.visit[u] = *t;
                self.order[*t] = u;
                *t += 1;
                let p = self.par[u];
                let heavy = *g[u]
                    .iter()
                    .filter(|&v| *v != p)
                    .max_by_key(|&v| self.size[*v])
                    .unwrap_or(&Self::NULL);
                if heavy != Self::NULL {
                    self.hld(g, heavy, self.head[u], t);
                }
                g[u].iter()
                    .filter(|&v| *v != p && *v != heavy)
                    .for_each(|&v| self.hld(g, v, v, t));
            }
            pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
                loop {
                    if self.visit[u] > self.visit[v] {
                        swap(&mut u, &mut v);
                    }
                    if self.head[u] == self.head[v] {
                        return u;
                    }
                    v = self.par[self.head[v]];
                }
            }
            pub fn dist(&self, u: usize, v: usize) -> usize {
                self.depth[u] + self.depth[v] - 2 * self.depth[self.lca(u, v)]
            }
        }
    }
    mod wavelet {
        use std::mem::swap;
        #[derive(Clone)]
        struct BitVector {
            n: usize,
            bit: Vec<u64>,
            sum: Vec<usize>,
        }
        impl BitVector {
            pub fn new(n: usize) -> Self {
                Self {
                    n,
                    bit: vec![0; (n >> 6) + 1],
                    sum: vec![0],
                }
            }
            pub fn set(&mut self, k: usize) {
                assert!(k < self.n);
                self.bit[k >> 6] |= 1u64 << (k & 63);
            }
            pub fn build(&mut self) {
                for i in 0..self.bit.len() {
                    self.sum.push(self.sum[i] + self.bit[i].count_ones() as usize);
                }
            }
            pub fn get(&self, k: usize) -> bool {
                assert!(k < self.n);
                (self.bit[k >> 6] >> (k & 63) & 1) == 1
            }
            fn rank1(&self, k: usize) -> usize {
                assert!(k <= self.n);
                self.sum[k >> 6] + (self.bit[k >> 6] & ((1u64 << (k & 63)) - 1)).count_ones() as usize
            }
            pub fn rank(&self, k: usize, f: bool) -> usize {
                assert!(k <= self.n);
                if f {
                    self.rank1(k)
                } else {
                    k - self.rank1(k)
                }
            }
            pub fn select(&self, k: usize, f: bool) -> Option<usize> {
                if self.rank(self.n, f) <= k {
                    return None;
                }
                let mut l = 0;
                let mut r = self.n;
                while r - l > 1 {
                    let m = (l + r) / 2;
                    if self.rank(m, f) >= k+1 {
                        r = m;
                    } else {
                        l = m;
                    }
                }
                Some(l)
            }
            pub fn select_after(&self, k: usize, f: bool, l: usize) -> Option<usize> {
                self.select(k + self.rank(l, f), f)
            }
        }
        pub struct WaveletMatrix {
            n: usize,
            bits: Vec<BitVector>,
            mid: [usize; 64],
        }
        impl WaveletMatrix {
            pub fn new(a: &[u64]) -> Self {
                let n = a.len();
                let mut a = a.to_owned();
                let mut bits = vec![BitVector::new(n); 64];
                let mut mid = [0; 64];
                for level in (0..64).rev() {
                    let mut l = vec![];
                    let mut r = vec![];
                    for i in 0..a.len() {
                        if (a[i] >> level) & 1 == 1 {
                            bits[level].set(i);
                            r.push(a[i]);
                        } else {
                            l.push(a[i]);
                        }
                    }
                    mid[level] = l.len();
                    bits[level].build();
                    l.extend(r);
                    swap(&mut a, &mut l);
                }
                Self { n, bits, mid }
            }
            fn succ(&self, f: bool, k: usize, level: usize) -> usize {
                self.bits[level].rank(k, f) + self.mid[level] * f as usize
            }
            pub fn access(&self, mut k: usize) -> u64 {
                let mut x = 0u64;
                for level in (0..64).rev() {
                    let f = self.bits[level].get(k);
                    if f {
                        x |= 1u64 << level;
                    }
                    k = self.succ(f, k, level);
                }
                x
            }
            pub fn rank(&self, mut r: usize, x: u64) -> usize {
                let mut l = 0;
                for level in (0..64).rev() {
                    let f = (x >> level & 1) == 1;
                    l = self.succ(f, l, level);
                    r = self.succ(f, r, level);
                }
                r - l
            }
            pub fn select(&self, mut k: usize, x: u64) -> Option<usize> {
                let mut l = [0; 64];
                let mut r = [self.n; 64];
                for level in (1..64).rev() {
                    let f = (x >> level & 1) == 1;
                    l[level-1] = self.succ(f, l[level], level);
                    r[level-1] = self.succ(f, r[level], level);
                }
                for level in 0..64 {
                    let f = (x >> level & 1) == 1;
                    let s = self.bits[level].select_after(k, f, l[level]);
                    if s.is_none() {
                        return None;
                    }
                    k = s.unwrap();
                    if k >= r[level] {
                        return None;
                    }
                    k -= l[level];
                }
                Some(k)
            }
            pub fn select_after(&self, k: usize, x: u64, l: usize) -> Option<usize> {
                self.select(k + self.rank(l, x), x)
            }
            pub fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> u64 {
                assert!(l <= r);
                assert!(k < r - l);
                let mut x = 0u64;
                for level in (0..64).rev() {
                    let cnt = self.bits[level].rank(r, false) - self.bits[level].rank(l, false);
                    let f = cnt <= k;
                    if f {
                        x |= 1u64 << level;
                        k -= cnt;
                    }
                    l = self.succ(f, l, level);
                    r = self.succ(f, r, level);
                }
                x
            }
            pub fn kth_largest(&self, l: usize, r: usize, k: usize) -> u64 {
                self.kth_smallest(l, r, r - l - k - 1)
            }
            pub fn _range_freq(&self, mut l: usize, mut r: usize, upper: u64) -> usize {
                let mut s = 0;
                for level in (0..64).rev() {
                    let f = (upper >> level & 1) == 1;
                    if f {
                        s += self.bits[level].rank(r, false) - self.bits[level].rank(l, false);
                    }
                    l = self.succ(f, l, level);
                    r = self.succ(f, r, level);
                }
                s
            }
            pub fn range_freq(&self, l: usize, r: usize, lower: u64, upper: u64) -> usize {
                self._range_freq(l, r, upper) - self._range_freq(l, r, lower)
            }
            pub fn prev_value(&self, l: usize, r: usize, upper: u64) -> Option<u64> {
                let cnt = self._range_freq(l, r, upper);
                if cnt == 0 {
                    None
                } else {
                    Some(self.kth_smallest(l, r, cnt-1))
                }
            }
            pub fn next_value(&self, l: usize, r: usize, lower: u64) -> Option<u64> {
                let cnt = self._range_freq(l, r, lower);
                if cnt == r - l {
                    None
                } else {
                    Some(self.kth_smallest(l, r, cnt))
                }
            }
        }
    }
    pub use algebra::{MapMonoid, Monoid};
    pub use convolution::{butterfly, butterfly_doubling, butterfly_inv, convolution};
    pub use fps::{Fps, Fps998244353};
    pub use geometry::{area_x2, convex_hull, cross, Point};
    pub use graph::{dijkstra, Edge};
    pub use math::{lagrange_polynomial, subset_zeta_transform, Combination};
    pub use modulo::{Mod998244353, Modulus};
    pub use rbtree::RbTree;
    pub use rolling_hash::RollingHash;
    pub use tree::HLD;
    pub use wavelet::WaveletMatrix;
}
