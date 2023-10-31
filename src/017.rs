use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut circle = vec![0; n]; // 円周上の点毎に線分の個数を格納
    let mut circle_l = vec![0; n]; // 円周上の点毎に線分のL側の点の個数を格納
    let mut position_lr = vec![]; // 各線分の端点の座標をLとRの順に格納
    let mut position_rl = vec![]; // 各線分の端点の座標をRとLの順に格納
    for i in 0..m {
        input!{
            l_i: usize,
            r_i: usize,
        }
        circle[l_i - 1] += 1;
        circle[r_i - 1] += 1;
        circle_l[l_i - 1] += 1;
        position_lr.push(vec![l_i - 1, r_i - 1]);
        position_rl.push(vec![r_i - 1, l_i - 1]);
    }
    position_lr.sort();
    position_rl.sort();

    // 方針: 
    // L1 < L2 < R1 < R2を満たす辺の組((L1, R1), (L2, R2))の個数の和が求める答え
    // M本の辺に一本ずつ着目して、全探索すればO(M*(M-1)/2)で解けるけど、M <= 3*10^5よりTLEする
    // ◆求める事象
    // L1 < L2 < R1 < R2 
    // <=> L1 < L2 かつ L2 < R1 かつ R1 < R2
    // これは条件が多くて難しいので、余事象を考える
    // [1] 端点が被るもの L1 = L2 or L1 = R2 or L2 = R1
    // 以下ではL1 < L2とする
    // [2] L1 < R1 < L2 < R2 <=> R1 < L2 (L1 < R1 と L2 < R2は定義より自明)
    // [3] L1 < L2 < R2 < R1 <=> R2 < R1 (L1 < L2 と L2 < R2は定義より自明)

    // [1]端点が被るもの辺の組を数える
    let mut complementary_event_1 = 0;
    for i in 0..n {
        let n_i = circle[i];
        if n_i == 0 {continue}
        complementary_event_1 += n_i * (n_i-1) / 2; // 円周上の点にn_i本の辺があれば、その組はn_iC2本ある。
    }

    // [2] L1 < R1 < L2 < R2 <=> R1 < L2 を満たす辺の組を数える (L1 < R1 と L2 < R2は定義より自明)
    let mut complementary_event_2 = 0;
    let mut cumulative_circle_l = vec![0; n];
    cumulative_circle_l[0] = circle_l[0];
    for i in 1..n {
        cumulative_circle_l[i] = cumulative_circle_l[i-1] + circle_l[i];
    }
    for i in 0..m {
        // let l1 = position_lr[i][0];
        let r1 = position_lr[i][1];
        // r1より大きなLが何個あるか?
        complementary_event_2 += m - cumulative_circle_l[r1];
    }

    // [3] L1 < L2 < R2 < R1 <=> R2 < R1 を満たす辺の組を数える (L1 < L2 と L2 < R2は定義より自明)
    let mut complementary_event_3 = 0;
    // BITの各ノードには、円周上の各座標における端点Lの個数が格納される
    let mut bit: BinaryIndexedTree<usize> = BinaryIndexedTree::new(n);
        // index: 円周上の座標
        // node: index上にある端点Lの個数
    for i in 0..m {
        // r1は、今までのイテレーションの中で最大のRの座標
        let r1 = position_rl[i][0];
        let l1 = position_rl[i][1];
        // l1 ~ r1の間にある線分の個数を数える (現在までに格納されているLの個数を数えれば十分)
        complementary_event_3 += bit.sum(r1) - bit.sum(l1);
        bit.add(l1, 1);
    }

    // 全事象mC2から余事象を引くと、求めたい辺の組の個数が得られる。
    println!("{}", m * (m-1) / 2 - complementary_event_1 - complementary_event_2 - complementary_event_3);

    // // BITの動作確認
    // let mut bit: BinaryIndexedTree<usize> = BinaryIndexedTree::new(7);
    // println!("------------");
    // bit.add(4, 3);
    // bit.sum(6);    
}


/// Binary Indexed Tree (BIT)
/// 参考: https://algo-logic.info/binary-indexed-tree/
/// (1)構築: O(N)
/// (2)加算: O(logN): 数列Anのi番目の項にxを足す (区間加算じゃないので注意)
/// (3)区間和: O(logN): 数列Anの先頭からi番目までの項の和を求める (閉区間だからiも含めるので注意)
/// セグメント木より機能が限定的だが、実装が簡単 & 定数倍で高速 & 省メモリ
#[derive(Debug, Clone, PartialEq, Eq)]
struct BinaryIndexedTree<T> {
    n: isize,       // 配列の要素数(数列の要素数+1)
    bit: Vec<T>    // データの格納先(1-indexed)。初期値は0
    // 0 始まり(0-indexed) ではなく 1 から番号を始めている(1-indexed)
    // また半開区間ではなく閉区間で考える。
    // これは後で計算をする際に楽になるため。
}

impl<T: Default + Copy + std::ops::AddAssign + std::ops::SubAssign + std::fmt::Debug + std::ops::Sub<Output = T>> BinaryIndexedTree<T> {
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            n: (n + 1) as isize,
            bit: vec![T::default(); n + 1] // 例えばTがusizeならdefault()は0を返す
        }
    }

    // add のインターフェースとしてindexは元の数列のindexを採用している(内部で+1している)
    fn add(&mut self, index: usize, x: T) {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
        while i < self.n {
            self.bit[i as usize] += x;
            // println!("i={}, i={:05b} -i={:05b}", i, i, -i);

            // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
            // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
            i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる

            // Rustでは、負の数は2の補数表現で保持される。
            // 補数の定義: N進法において自然数xを表現するのに必要な最小の桁数をnとしたとき
            // xのNの補数はN^n - x となる
            // 例： 5(10進数)=101(2進数)の2の補数は、2^3-5(10進法) = 1000 - 101 (2進法) = 011(2進法)となる
            // 参考1: http://www.cc.kyoto-su.ac.jp/~kbys/kiso/number/negative.html
            // 参考2: http://www.f.waseda.jp/takezawa/math/joho/hosuu.pdf
            // もっと端的に言うと、
            // 0の定義を、そのデータ型のビット数の限界に1桁左から1を追加したものとする
            // 例: 3bitだけ使える場合、下記のように考える
            // -3: 0101
            // -2: 0110
            // -1: 0111
            //  0: 1000 <- 3bitの0の定義
            //  1: 0001
            //  2: 0010
            //  3: 0011
            // また、isizeの場合、-1は
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111111111
            // 1111 となる (1が64個)。
        }
    }

    // Tが非負整数型(usizeなど)のときに、除算を行う
    fn subtract(&mut self, index: usize, x: T) {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも半開区間にできる
        while i < self.n {
            self.bit[i as usize]  -= x;
            i += (i & - i); // iにi の最後の1のビットを足すと、親のインデックスに移れる
        }
    }

    // a_1 + a_2 + ... + a_i を計算する (sumのインターフェースは半開区間ではなく閉区間を採用。a[index]は足される)
    fn sum(&self, index: usize) -> T {
        let mut i = (index + 1) as isize;
        // let mut i = index as isize; // こっちを採用すると、インターフェースも開区間にできる
        let mut sum = T::default(); // 例えばTがusizeならdefault()は0を返す
        while i > 0 {
            // println!("i={}, sum={:?}", i, sum);
            sum += self.bit[i as usize];
            // i の最後の1のビット = i & -i (∵負の数は「ビット反転+1」で表現される)
            // 例: 6 & -6 = (00…0110)_2 & (11…1010)_2 = (00…0010)_2
            i -= (i & - i); // iにi の最後の1のビットを引くと、1個左上のノードのインデックスに移れる
            // println!("i={}, sum={:?}", i, sum);
            // println!("==== ==== ==== ====");
        }
        return sum
    }
    // 閉区間[left, right]を取得する
    fn sum_range(&self, left: usize, right: usize) -> T {
        let right_sum: T = self.sum(right);
        let left_sum: T = match left == 0 {
            true => Default::default(), // 0のこと
            false => self.sum(left-1)
        };
        let range_sum: T = right_sum - left_sum;
        return range_sum
    }
    // index番目の値を取得する (sum()は累積和を取得するメソッド)
    fn get_element(&self, index: usize) -> T {
        let element = match index == 0 {
            true => self.sum(index),
            false => self.sum(index) - self.sum(index - 1)
        };
        return element
    }
    fn print_all_cum(&self) {
        // デバッグ用に、各インデックスにおける、累積和を標準出力に print
        print!("bit = ");
        for i in 0..self.n-1 {
            let sum_i = self.sum(i as usize);
            print!("{:?} ", sum_i);
        }
        println!("");
    }
}

