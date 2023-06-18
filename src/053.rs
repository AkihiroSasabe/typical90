use proconio::input;
use itertools::Itertools;
use superslice::Ext;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    // インタラクティブな問題参考
    // abc269 e: ここに詳しく色々書いた. 基本的には下で定義したread()関数を使えば問題ない。proconioのinput!マクロは使ってはいけない。
    // abc299 d

    // フィボナッチ探索についてとても参考になるtanaka-a(A Tanaka)氏の記事: 
    // https://qiita.com/tanaka-a/items/f380257328da421c6584

    // 考察
    // (1)3分探索 <=> 1:1:1 ※各イテレーション毎に区間を3分割して、探索範囲を2/3=0.666~に絞っていく。
    //      各イテレーションで2点探索する必要があるので、計算量はO(2*log(1.5)(N)). 
    //      N=1500だと37回クエリ必要でNG.
    // (2)黄金分割探索 <=> 1.618~: 1: 1.618~ ※前回のクエリ結果をリサイクルするので、各イテレーションで1点だけ取得すればよい。ただし、探索位置が小数になってしまうので扱いが大変。
    //      探索区間は、毎回確実に(1.618 + 1.0) / (1.618 * 2 + 1.0) = 0.618倍されるので、O(log(1.618)(N))となる。a:b:aで次のa + bの区間もb:c:bに分けていく。
    //     a:b=b:c ∧ 2a+b=1 ∧ a+b=2b+c
    // <=> b^2=ac ∧ b=1-2a  ∧ c=a-b
    // <=> b^2=ac ∧ b=1-2a  ∧ c=3a-1
    // <=> (1-2a)^2=a(3a-1) ∧ b=1-2a  ∧ c=3a-1
    // <=> a^2-3a+1=0 ∧ b=1-2a  ∧ c=4a-1
    // <=> a=(3±√5)/2 ∧ b=1-2a  ∧ c=4a-1
    // 0<a<1なので、最終的にa=(3-√5)/2となる。これは黄金比(フィボナッチ数列のn->∞にしたときのFn/Fn-1)
    // (3)フィボナッチ探索 <=> Fib(n+1): Fib(n): Fib(n+1) ※探索位置が必ず整数になるように分割
    //      N=1500だけど、N以上の最小のフィボナッチ数までパディング(拡張)してN=1597とすることで、全テストケースに対して14回で回答可能
    // ちなみに2分探索で微分値の符号が負になる境界位置を探索する解法は、O(2*log2(N))となる。各イテレーションで微小区間2点のクエリが必要になるため。

    // フィボナッチ数列を格納した数列
    let n16 = 16;
    let mut fibonacci = vec![0; n16 + 1];
    get_fibonacci(n16, &mut fibonacci);
    // fibonacci[0] = 1;
    // fibonacci[1] = 1;
    // fibonacci[2] = 2;
    // fibonacci[3] = 3; <-最終的な探索範囲の長さ(1:1:1)で切る
    // ...
    // fibonacci[15] = 987;
    // fibonacci[16] = 1597; <- 1 <= N <= 1500なのでここまで拡張すれば全部網羅できる

    let over_fib_16 = fibonacci[n16] + 1; // 1597はフィボナッチ数(N<=1500なので、1501以上のフィボナッチ数であれば良い。)
    let t = read();
    for _ in 0..t {
        let n = read();
        // 数列Aの初期化 (未確定の値は-1にしておく)
        let mut a = vec![-1_isize; over_fib_16];

        // フィボナッチ探索を実行
        let mut fib_index = n16 - 2;
        let mut query_l = fibonacci[fib_index];
        fib_index -= 1;
        let mut query_r = query_l + fibonacci[fib_index];

        // 14回の探索ちょうどで最大値が確定する
        for iteration in 0..(n16 - 2) {
            // println!("query_l={} query_r={}", query_l, query_r);
            let mut left_value: isize = a[query_l];
            let mut right_value: isize = a[query_r];

            // [1]クエリを送る
            // 左側の値を求める
            // クエリを送ったことのないインデックスか?
            if a[query_l] == -1 {
                // クエリが1-nの範囲にいるか?
                left_value = if query_l != 0 && query_l <= n {
                    // クエリを送る
                    println!("? {}", query_l);
                    read() as isize
                }
                else {
                    // 拡張した探索範囲の項であれば、端に行くほど小さくなる負の値を格納
                    -1 - query_l as isize
                };
                a[query_l] = left_value;
            }
            
            // 右側の値も左側と同様に求める
            if a[query_r] == -1 {
                right_value = if query_r != 0 && query_r <= n {
                    println!("? {}", query_r);
                    read() as isize
                }
                else {
                    -1 - query_r as isize
                };
                a[query_r] = right_value;
            }
            
            // [2]クエリ位置の更新
            // 最終回(13)は更新しない
            if iteration == n16 - 3 {break}
            fib_index -= 1;
            if left_value < right_value {
                query_l = query_r;
                query_r = query_l + fibonacci[fib_index];
            }
            else {
                query_r = query_l;
                query_l = query_r - fibonacci[fib_index];
            }
        }
        let ans = max(a[query_l], a[query_r]);
        println!("! {}", ans);
    }

}

// インタラクティブな入力を受け取る関数
fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

// 0以上n以下のフィボナッチ数列をfibonacciに格納して求める関数
fn get_fibonacci(n: usize, fibonacci: &mut Vec<usize>) -> usize {
    // println!("n={}", n);
    if fibonacci[n] != 0 {
        // println!("f({})={}",n, fibonacci[n]);
        return fibonacci[n]
    }
    else if n == 0 || n == 1 {
        fibonacci[n] = 1;
        // println!("f({})={}",n, fibonacci[n]);
        return fibonacci[n]
    }
    else {
        fibonacci[n] = get_fibonacci(n-1, fibonacci) + get_fibonacci(n-2, fibonacci);
        // println!("f({})={}",n, fibonacci[n]);
        return fibonacci[n]
    }
}