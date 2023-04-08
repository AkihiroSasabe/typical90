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
        d: usize,
        a: [i128; n]
    }
    
    // 包除原理: 
    // ∣A1​∪A2​∪...∪An​∣ = (-1)^(0) * ∑​∣Ai​∣ + (-1)^(1) * ∑​∣Ai​ ∩ Aj​∣ + (-1)^(2) * ∑​∣Ai​ ∩ Aj ​∩ Ak​∣ − ... +(−1)^(n−1) * ∣A1​ ∩ A2... An-1 ∩ An​∣
    // 例: 
    // A∪B∪C∪D = (A+B+C+D) - (A∩B + A∩C + A∩D + B∩C + B∩D + C∩D) + (A∩B∩C + B∩C∩D + C∩D∩A + D∩A∩B) - A∩B∩C∩D
    // 包除原理の各項目はnCr個ある。上の例なら第一項, 第二項, 第三項, 第四項はそれぞれ 4C1, 4C2, 4C3, 4C4 = 4, 6, 4, 1個ある。
    // 一般化すると、全項目の和の計算量はO((1+1)^n - 1) = O(2^n - 1)となる。

    // 入力例1
    // N, D = 4, 3
    // A1, A2, A3, A4 = 1, 3, 4, 5
    
    // Ai & x == 0 を満たすxを数える
    // 1: 001 =>  1&x == 0 を満たす条件は、x=**0 => 2^2通り
    // 3: 011 =>  3&x == 0 を満たす条件は、x=*00 => 2^1通り
    // 4: 100 =>  4&x == 0 を満たす条件は、x=0** => 2^2通り
    // 5: 101 =>  5&x == 0 を満たす条件は、x=0*0 => 2^1通り
    // よって 2^(各ビットの0の個数)通りとなる。

    // 次にAiもAjも a &x == 0 を満たす条件(Ai ∩ Aj)を考える。
    // 1と3の場合、*00 だが、これは !(1or3) で、2^1通り
    // 1と4の場合、0*0 で、2^1通り
    // 1と5の場合、0*0 で、2^1通り
    // 3と4の場合、000 で、2^0通り
    // 3と5の場合、000 で、2^0通り
    // 4と5の場合、0*0 で、2^1通り
    // よって一般化すると、 2^( !(Ai | Aj) の各ビットの0の個数 ) 通りとなる。

    // xの個数の初期化 (0 <= x < 2^D)
    let mut ans: i128 = 1 << d;

    // ansから条件を満たさないx(<=> 全iについて、Ai & x = 0)の集合の個数を引いていく
    // 全計算量は、O(ΣnCr * (n+d)) = O(2^n * (n+d)) = O(10^6 * (20+60)) <= O(10^8) ~ 1secぐらいで解ける
    // 1番目と2番目のforループで合わせて、O(ΣnCr)=O(2^n)
    for r in 1..(n+1) {
        // diff := n個のAの中からr個の要素全てについて、ビット積が0にならないxの個数
        let mut diff = 0;
        // O(nCr)
        for comb in (0..n).combinations(r) {
            // println!("comb={:?}", comb);

            // 計算量O(r). r<=n<=20より定数倍
            // or := r個の要素についてビット和を取ったもの
            let mut or = 0;
            for j in 0..comb.len() {
                // print!("{:03b}, ", a[comb[j]]);
                or |= a[comb[j]];
            }
            // println!("or: {:03b}", or);

            // 計算量: O(d). d <= 60より定数倍
            // zero_num := orの各ビットの内、値が0であるビットの個数
            let mut zero_num = 0;
            for digit in 0..d {
                // 桁が0の桁数を数える
                if (or & (1 << digit)) == 0 {
                    zero_num += 1;
                }
            }
            diff += 1 << zero_num;
        }
        // 包除原理よりrが奇数ならマイナス
        if r % 2 == 1 {
            ans = ans - diff;
        }
        // rが偶数ならプラス
        else {
            ans = ans + diff;
        }
    }
    println!("{}", ans);

}