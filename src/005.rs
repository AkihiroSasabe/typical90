use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    // 参考: けんちょんの解説が一番参考になる
    // https://drken1215.hatenablog.com/entry/2021/10/08/231200
    // https://www.youtube.com/watch?v=TvUyhtSeHW8
    // https://github.com/E869120/kyopro_educational_90/blob/main/editorial/005-01.jpg
    // https://github.com/E869120/kyopro_educational_90/blob/main/editorial/005-02.jpg
    // https://github.com/E869120/kyopro_educational_90/blob/main/editorial/005-03.jpg
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }

    let MODULUS = 1_000_000_007;
    // log2(n)を求める
    let mut log2_n: usize = 0;
    let mut n_copy = n;
    while n_copy != 0 {
        n_copy /= 2;
        log2_n += 1;
    }
    // n =      0, 1, 2, 3, 4, 5
    // log_n =  0, 1, 2, 2, 3, 3
    // 真値    -∞, 0, 1, 1., 2, 2.
    log2_n += 2; // 本当は+0だけど、余分に大きくしておく

    // 10^(2^i)をbで割った余り
    let mut ten_to_the_power_of_po2 = vec![10 % b; log2_n];
    // 例: ten_to_the_power_of_po2[0] =  10 % b;    // 10^(2^0) % b = 10^1 % b
    // 例: ten_to_the_power_of_po2[1] = 100 % b;   // 10^(2^1) % b = 10^2 % b
    for i in 1..log2_n {
        // 10^(2^i) = 10^(2^(i-1)) * 10^(2^(i-1))
        // 例: 10^(2^4) = 10^(2^3) * 10^(2^3)
        ten_to_the_power_of_po2[i] = (ten_to_the_power_of_po2[i-1] * ten_to_the_power_of_po2[i-1]) % b;
    }

    // dp[桁数][余り] := 存在個数 をdp[N][0]から順番にもとめて、dp[N][0]を求めたい。
    // dp[i+1][(r*10 + c[k]) % b] += dp[i][r] という性質から、
    // dp[N] = A^(N-1) * dp[1], where A := BxBの行列 で計算できる(詳細は下の考察)。
    // 行列の積A*AはO(B^3)かかり、これをN乗すると繰り返し2乗法を使っても、
    // O(B^3*logN)でTLEしてしまう。(∵B<=10^3, N<=10^18)
    
    // 次にダブリングを考える。
    // ダブリング: 1手先, 2手先, 4手先, 8手先, ..., 2^i手先を前計算してn手先をO(log2(n))で高速に求める
    // 1234 = 12 * (10^2) + 34
    // 123456 = 1234 * (10^4) + 56
    // ダブリングの例: 
    // 3^100 = 3^64 + 3^32 * 3^4
    // 一般に 3^N を計算する場合には、N を"二進法"で表すことで求められます。
    // このような方法をダブリングと呼ぶことがあります。
    // let mut dp = vec![vec![0; b]; n];
    
    // 以下、dp[2^i][r] を dp_doubling[i][r] と書くことにする。
    // N=10^18個はメモリサイズの制約を超えるので。
    // dp[2], dp[4], dp[8] と飛び飛びの桁だけ計算すればいいのでlog2(10^18) <= 60 個 で良い
    // 10^18 <= 2^60
    let mut dp_doubling = vec![vec![0; b]; log2_n];
    // 初期化
    for kk in 0..k {
        // 2^0 = 1桁目の各余りの存在個数
        dp_doubling[0][c[kk] % b] += 1;
    }
    // ダブリング (事前計算)
    for i in 1..log2_n {
        // 2^(i-1)桁数から2^i桁数を求めていく
        // 例
        // 2^0=1桁から、2^1=2桁を求める
        // 2^1=2桁から、2^2=4桁を求める
        // 2^2=4桁から、2^3=8桁を求める
        dp_doubling[i] = mul(&dp_doubling[i-1], &dp_doubling[i-1], ten_to_the_power_of_po2[i-1], b, MODULUS);
    }

    // ダブリングした結果をもとに答えを求める
    let mut res = vec![0; b];
    res[0] = 1;
    // 全体でO(B^2log(N))
    for i in 0..log2_n {
        if (n & (1 << i)) != 0 {
            // n桁
            // 2^0 = 001
            // 2^1 = 010
            // 2^2 = 100
            // 1桁 = 001 => dp_doubling[0][0]で終わりやな。(i=0)
            // 2桁 = 010 => dp_doubling[1][0]で終わりやな。(i=1)
            // 3桁 = 011 => dp_doubling[0] * dp_doubling[1]
            // 4桁 = 100 => dp_doubling[2][0]で終わりやな。(i=2)
            // 5桁 = 101 => dp_doubling[0] * dp_doubling[2]
            // 6桁 = 110 => dp_doubling[1] * dp_doubling[2]
            // 7桁 = 111 => dp_doubling[0] * dp_doubling[1] * dp_doubling[2]
            // 8桁 = 1000 => dp_doubling[3][0]で終わりやな。(i=3)
            // 9桁 = 1001 => dp_doubling[0] * dp_doubling[3]
            // O(B^2)
            res = mul(&res, &dp_doubling[i], ten_to_the_power_of_po2[i], b, MODULUS);
        }
    }
    println!("{}", res[0]);


    // 2023-04-25 23:33 (当時の考察)
    // 入力例1
    // 3 7 3
    // 1 4 9

    // 111
    // 114
    // 119  7で割れる
    
    // 141
    // 144  
    // 149 

    // 191
    // 194
    // 199

    // 411  
    // 414
    // 419

    // 441  7で割れる
    // 444
    // 449

    // 491  
    // 494
    // 499

    // 911
    // 914
    // 919
    
    // 941
    // 944
    // 949

    // 991
    // 994  7で割れる
    // 999

    // ...
    // k^n個の組合せが得られる  (k^n=3^3=27個)
    // bの倍数は何個あるか?     (b=7の倍数)
    // 1<=k<=9
    // 1<=N<=10^18
    // 2<=B<=1_000
    // 全探索するなら、最大で9^(10^18)でとても間に合わない
    // 999...9999をBで割るのもむずそう

    // 1:      1
    // 10:     3
    // 100:    2
    // 1000:   6
    // 10000:  4
    // 100000: 5

    // %B で何事も考えたいよね。
    // 動的計画法で考えても駄目だね
    // 縦軸を桁、横軸を余り O(N * B) = O(10^18 * 10^3)の動的計画法<-TLE

    // 10をかけること <=> 10 % B をかけることと同じ(7+3)^t
    // 1 -> 3 -> 2 -> 6 -> 4 -> 5 -> 1 -> ...
    // 周期T
    // 余り
    //         0   1   2   3   4   5   6
    // 1桁目   0   1   1   0   1   0   0
    // 2桁目   
    // 3桁目

    // 行列計算に落とし込む
    // dp[i+1][(r*10+c[k])%B] += dp[i][r]
    // dp[2][0] = A_00 * dp[1][0] + A_01 * dp[1][1] + ... + A_06 * dp[1][6]
    // dp[2][1] = A_10 * dp[1][0] + A_11 * dp[1][1] + ... + A_16 * dp[1][6]
    // dp[2][2] = ...
    // dp[2][3]
    // dp[2][4]
    // dp[2][5]
    // dp[2][6] = A_60 * dp[1][0] + A_61 * dp[1][1] + ... + A_66 * dp[1][6]
    // <=>
    // dp[i+1] = A * dp[i]
    // dp[N] = A^(N-1) * dp[i]
    // ちなみに行列の積A*AはO(B^3)かかるが、これは
    // A*A =   (
    //             (O(B), O(B), ... O(B)),
    //             (O(B), O(B), ... O(B)),
    //             ...
    //             (O(B), O(B), ... O(B))
    //         )
    // のように、一つのセルを計算するのにO(B)で、それがB^2個あり、合計でO(B^3)となるからである。

}

// dp[2^i] と dp[2^j] を掛け合わせて dp[2^(i+j)] を得る関数
// tj: 10^j を B で割った余り
fn mul(dp_doubling_i: &Vec<usize>, dp_doubling_j: &Vec<usize>, tj: usize, b: usize, modulus: usize) -> Vec<usize> {
    // 例えばi=1, j=2 <=> 2^1=2桁 と 2^2=4桁ならi+j=2+4=6桁の個数を求めることになる
    // 例えば12(2桁)と3456(4桁)で123456(6桁)とするなら、下記のように表せる
    // 12 * 10^4 + 3456
    let mut dp_doubling_i_plus_j = vec![0; b];
    for p in 0..b {
        for q in 0..b {
            dp_doubling_i_plus_j[(p * tj + q) % b] += dp_doubling_i[p] * dp_doubling_j[q];
            dp_doubling_i_plus_j[(p * tj + q) % b] %= modulus;
        }
    }
    return dp_doubling_i_plus_j
}