use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use num::Complex;

// 参考実装: https://qiita.com/AngrySadEight/items/0dfde26060daaf6a2fda#11-3-%E7%AB%B6%E3%83%97%E3%83%AD%E5%85%B8%E5%9E%8B-90-%E5%95%8F14-065rgb-balls-2
// 高速フーリエ変換と数論変換の解説が丁寧で一番分かりやすい。典型65の解説もある。
fn main() {
    input! {
        R: usize,
        G: usize,
        B: usize,
        K: usize,
        X: usize,
        Y: usize,
        Z: usize
    }

    // r + g <= X
    // g + b <= Y
    // b + r <= Z

    // P = 998_244_353 = 119 × (2^23) + 1 となり、P-1を2^23で割れるので都合が良い(n + m = 23次の多項式まで対応可能)
    let modulus = 998_244_353;

    // n!を格納された配列
    let mut memo_factorial = vec![1;200000+1];
    for i in 1..(200000+1) {
        memo_factorial[i] = (i * memo_factorial[i-1]) % modulus;
    }

    // nCrを各色についてR, G, Bについて格納していく。ただし、rが定義域外にある場合はnRr=0とする
    let mut r_coff = vec![0; K+1];
    let mut g_coff = vec![0; K+1];
    let mut b_coff = vec![0; K+1];
    for r in K-Y..min(K+1, R+1) {
        r_coff[r] = combination(R, r, modulus, &memo_factorial) as isize;
    }
    for g in K-Z..min(K+1, G+1) {
        g_coff[g] = combination(G, g, modulus, &memo_factorial) as isize;
    }
    for b in K-X..min(K+1, B+1) {
        b_coff[b] = combination(B, b, modulus, &memo_factorial) as isize;
    }
    // println!("r_coff = {:?}", r_coff);
    // println!("g_coff = {:?}", g_coff);
    // println!("b_coff = {:?}", b_coff);

    let root = make_root(modulus as isize);
    let invroot: Vec<isize> = make_invroot(&root, modulus as isize);
    

    // 条件
    // {
    //     0 <= r <= R
    //     0 <= g <= G
    //     0 <= b <= B

    //     r + g <= X
    //     g + b <= Y
    //     b + r <= Z

    //     r + g + b = r 
    // }
    // <=>
    // {
    //     0 <= r <= R
    //     0 <= g <= G
    //     0 <= b <= B

    //     K - X <= b
    //     K - Y <= r
    //     K - Z <= g

    //     r + g + b = r
    // }
    // <=>
    // {
    //     K - X <= b <= B
    //     K - Y <= r <= R
    //     K - Z <= g <= G

    //     r + g + b = r
    // }



    // 最終的に求めたいのは、
      Σ[r=K-Y, K] Σ[g=K-Z, K-r] Σ[b=K-X, K-r] (RCr * GCg * BCb)
    = Σ[r=K-Y, K] Σ[g=K-Z, K-r] (RCr * GCg * BCb)

    // 数論変換(NNT)で畳み込みの計算. O(KlogK)かかる
    let mut green_blue_for_r = convolution_by_ntt(&mut g_coff, &mut b_coff, &root, &invroot, modulus as isize);
    // println!("green_blue_for_r = {:?}", green_blue_for_r);

    let mut ans = 0;
    for r in K-Y..min(R+1, K+1) {
        let g_plus_b = K - r;
        ans += (r_coff[r] * green_blue_for_r[g_plus_b]) % modulus as isize;
        ans %= modulus as isize;
    }

    // // brute-force (O(k^2)かかる)) k< 2*10^5よりTLE
    // let mut ans = 0;
    // for r in 0..K+1 {
    //     for g in 0..(K-r+1) {
    //         let b = K - r - g;
    //         if B < b || G < g || R < r {continue}
    //         if b < K -X || r < K -Y || g < K - Z {continue}
    //         ans += combination(R, r, modulus, &memo_factorial) * combination(G, g, modulus, &memo_factorial) % modulus * combination(B, b, modulus, &memo_factorial);
    //         ans %= modulus;
    //     }
    // }
    println!("{}", ans);


    // // [1]. 複素数の挙動の確認
    // let z1 = Complex::new(1.0, 2.0);  // 実部1.0、虚部2.0の複素数
    // let z2 = Complex::new(3.0, 4.0);  // 実部3.0、虚部4.0の複素数

    // let sum = z1 + z2;  // 複素数の加算
    // let product = z1 * z2;  // 複素数の乗算

    // println!("Sum: {}", sum);
    // println!("Product: {}", product);

    // // [2]. 高速フーリエ変換による畳み込みの動作確認
    // let mut a = vec![1.0, 2.0, 3.0];
    // let mut b = vec![2.0, 3.0, 4.0];
    // let c = convolution_by_fft(&a, &b);
    // let mut c_isize = vec![];

    // for i in 0..c.len() {
    //     c_isize.push(c[i].round() as isize);
    // }

    // println!("a={:?}", a); // a=[1.0, 2.0, 3.0]
    // println!("b={:?}", b); // b=[2.0, 3.0, 4.0]
    // println!("c={:?}", c); // c=[1.9999999999999982, 6.999999999999998, 15.999999999999998, 17.0, 12.000000000000002]
    // println!("c={:?}", c_isize); // c=[2, 7, 16, 17, 12]


    // // [3]. 数論変換による畳み込みのテスト
    // let mut a = vec![1, 2, 3];
    // let mut b = vec![2, 3, 4];

    // // NTT で必要となる r の累乗数を前計算しておく（これをしないと計算量が悪くなる）．
    // let modulus = 998_244_353;
    // let root = make_root(modulus);
    // let invroot = make_invroot(&root, modulus);


    // // convolution 関数で A(x) と B(x) の多項式乗算を行い，C(x) = A(x) * B(x) の係数を小さい順に並べた配列 c を返す．
    // let c = convolution_by_ntt(&mut a, &mut b, &root, &invroot, modulus);
    // println!("ntt c={:?}", c); // ntt c=[2, 7, 16, 17, 12, 0, 0, 0]
}




// 数論変換 (Number Theoretic Transform: NTT)
fn ntt(a: &Vec<isize>, depth: isize, root: &Vec<isize>, modulus: isize) -> Vec<isize>{
    // inv = 1 ならば普通の NTT，
    // inv = -1 ならば INTT になるようにする（今回は，呼び出す root が逆元かそうでないかによって調整する）．
    let n = a.len();
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
    let mut da = ntt(a, log_2n as isize - 1, root, modulus);
    let mut db = ntt(b, log_2n as isize - 1, root, modulus);

    // C(x) の NTT DC(t). これの k 次の係数は， DA(t) と DB(t) の k 次の係数を掛け合わせれば求まる．
    let mut dc = vec![0; n];
    for i in 0..n {
        dc[i] = (da[i] * db[i]) % modulus;
    }

    // C(x) は DC(t) を INTT すれば求まる．このようにしてできた配列 c は，C(x) の係数を次数の小さい順に並べたものとなっている．
    let c = ntt(&dc, log_2n as isize - 1, invroot, modulus);

    // INTT の後は最後に n で割ることを忘れずに．
    let mut ret = vec![];
    for i in 0..n {
        ret.push((c[i] * mod_inv(n as isize, modulus)) % modulus);
    }
    return ret
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

// mod を法とする x の逆元を計算する．
fn mod_inv(x: isize, modulus: isize) -> isize {
    return my_pow(x, modulus - 2, modulus);
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



// 畳み込み演算を高速化するために必要
// Fast Fourier Transform
fn fft(a: &Vec<Complex<f64>>, inv: f64) -> Vec<Complex<f64>> {
    // inv = 1 ならば普通の FFT (フーリエ変換), 
    // inv = -1 ならば IFFT (逆フーリエ変換) になるようにする．

    // フーリエ変換の場合の出力は、"F(k)の係数"を返していく。W := e^i(2pi/N)
    // F(k) := Σ[x=0,N-1] f(W^x) * W^(kx) 
    //       = f(W^0) * W^(k*0) + f(W^1) * W^(k*1) + f(W^2) * W^(k*2) + ... + f(W^(N-1)) * W^(k*(N-1))
    // つまり、出力は、[f(W^0), f(W^1), f(W^2), ..., f(W^N-1)] 

    // フーリエ変換の場合の入力は、"f(x)の係数"である。
    // f(x) := (1 / N) * Σ[k=0,N-1]F(W^k) * W^(-kx)
    //       = (1 / N) * (F(W^0) * W^(0*x) + F(W^1) * W^(1*x) + F(W^2) * W^(2*x) + ... + F(W^(N-1)) * W^((N-1)*x))
    // つまり、入力は、(1 / N) * [F(W^0), F(W^1), F(W^2), ..., F(W^N-1)]
    // (f(x) = a0 + a1*x + a2*x^2 + ... + a(n-1)*x^(n-1) の係数 [a0, a1, ...., a(n-1)] である)



    // 逆フーリエ変換の場合の出力は、"N * f(x)の係数" を返していく。(逆変換の場合は、N倍された値になっているので、後で1/N倍する必要があるので注意)
    // つまり、出力は、[F(W^0), F(W^1), F(W^2), ..., F(W^N-1)]
    
    // 逆フーリエ変換の場合の入力は、"F(k)の係数"であるから、
    // [f(W^0), f(W^1), f(W^2), ..., f(W^N-1)] となる


    let n = a.len();

    if n == 1 {
        // aのサイズが1であるときは、それがそのままDFT(離散フーリエ変換)である
        return a.clone()
    }
    else {
        // f(x)  := c0 + c1 * x + c2 * x^2 + c3 * x^3 + c4 * x^4 + c5 * x^5 + ...   + cn-2 * x^n-2       + cn-1 * x^n-1
        // fe(x) := c0          + c2 * x              + c4 * x^2 + ...              + cn-2 * x^(n/2 - 1)
        // fo(x) :=      c1                + c3 * x              + c5 * x^2 + ...                        + cn-1 * x^(n/2 - 1)
        // f(x) = fe(x^2) + x*fo(x^2)
        let mut even: Vec<Complex<f64>> = vec![];
        let mut odd: Vec<Complex<f64>> = vec![];
        for i in 0..n {
            if i % 2 == 0 {
                even.push(a[i]);
            }
            else {
                odd.push(a[i]);
            }
        }
        // evenとoddのDFTを再帰的に求める
        let d_even: Vec<Complex<f64>> = fft(&even, inv);
        let d_odd: Vec<Complex<f64>> = fft(&odd, inv);

        let n_f64 = n as f64;
        
        // zetaを求める。IFFT(逆フーリエ変換)のときは、偏角を-1倍する
        let angle = 2.0 * PI * inv / n_f64;
        let zeta = Complex::new(angle.cos(), angle.sin());

        let mut now = Complex::new(1.0, 0.0); // f(x)に代入していく引数のこと。x=now=zeta^0, zeta^1, zeta^2, ..., zeta^N

        let mut ret: Vec<Complex<f64>> = vec![];
        for i in 0..n {
            // f(x) = fe(x^2) + x*fo(x^2)なので、
            // f(now) = fe(now^2) + now*fo(now^2)
            ret.push(d_even[i % (n / 2)] + now * d_odd[i % (n / 2)]);
            now *= zeta;
        }
        return ret
    }
}

// 畳み込み演算f(x)*g(x) (畳み込みの正確な定義は積分で∫f(t)(T-t)dtとなる。)
// 参考実装: https://qiita.com/AngrySadEight/items/0dfde26060daaf6a2fda
// h(x) = f(x) * g(x) の係数を求めたい。(fとgはxに関する多項式)
// 愚直にやると、O(N^2)かかる
// h(x)を求めるには、xにN個の数字を代入すれば良い。
// その数字を1のN乗根を代入していく。

// h(x)の離散フーリエ変換H(x)　が　分かれば、　元の関数h(x)も計算可能
// 離散フーリエ変換はO(Nlog(N))で求まる

// ・フーリエ変換でf(W^i), g(W^i)を求める (i=0,1,2,..., 2*N)
// f(W^0), f(W^1), f(W^2), ..., f(W^2N) の値が知りたい
// f(W^0), f(W^1), f(W^2), ..., f(W^2N) は、f(x)のフーリエF(x)の係数に等しい。
// F(x)のフーリエ係数は、

// ・h(W^i) = f(W^i) * g(W^i) を求める (i=0,1,2,..., 2*N)

// ・h(W^i) からフーリエ逆変換を使って、h(x)の係数を求める。(i=0,1,2,..., 2*N)
// h(x)の係数は、
// H(W^0), H(W^1), H(W^2), ..., H(W^2N)
// である。
fn convolution_by_fft(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    // f(x) := a0 + a1 * x + a2 * x^2 + a3 * x^3 + ... +  + al-1 * x^l-1
    // g(x) := b0 + b1 * x + b2 * x^2 + b3 * x^3 + ... +  + bm-1 * x^m-1
    // h(x) := f(x) * g(x) 
    //       = c0 + c1 * x + c2 * x^2 + c3 * x^3 + ... +  + cn-1 * x^n-1    (n-1 := l+m-2)

    // aとbを複素数のベクトルに変換する
    let mut a: Vec<Complex<f64>> = a.iter().map(|&x| Complex::new(x as f64, 0.0)).collect();
    let mut b: Vec<Complex<f64>> = b.iter().map(|&x| Complex::new(x as f64, 0.0)).collect();

    // 配列a, bはそれぞれA(x)とB(x)の係数を次数の小さい順に並べたもの
    let len_a = a.len();
    let len_b = b.len();
    let len_c = len_a + len_b;  // len_c は A(x) * B(x) の次数 (正確には len_a + len_b - 2が次数だと思う。)

    // len_cより大きい最小の2の累乗を求める
    let mut n = 1;
    while n <= len_c {
        n *= 2;
    }

    // 配列の長さがnになるまで、配列の末尾に0を追加する
    while a.len() < n {
        a.push(Complex::new(0.0, 0.0));
    }
    while b.len() < n {
        b.push(Complex::new(0.0, 0.0));
    }

    // A(x) の FFT DA(t), B(x) の FFT DB(t) を求める
    // 配列 da, db は，それぞれ DA(t), DB(t) の係数を次数の小さい順に並べたもの．
    let mut da: Vec<Complex<f64>> = fft(&a, 1.0);
    let mut db: Vec<Complex<f64>> = fft(&b, 1.0);

    // C(x) の FFT DC(t). これの k 次の係数は， DA(t) と DB(t) の k 次の係数を掛け合わせれば求まる．
    let mut dc: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); n];
    for i in 0..n {
        dc[i] = da[i] * db[i];
    }

    //  C(x) は DC(t) を IFFT すれば求まる．このようにしてできた配列 c は，C(x) の係数を次数の小さい順に並べたものとなっている．
    let c: Vec<Complex<f64>> = fft(&dc, -1.0);

    // IFFT の後は最後に n で割ることを忘れずに．
    let mut ret = vec![];
    for i in 0..(len_a + len_b - 1) {
        let n_f64 = n as f64;
        ret.push(c[i].re / n_f64);
        // ret.push((c[i].re / n_f64).round() as isize); // 四捨五入するならこのコメントアウトを外して、1行上をコメントアウトする
    }

    return ret

}


// nCrを求める
fn combination(n: usize, r: usize, MODULO: usize, memo_factorial: &Vec<usize>) -> usize {
    // nCr = n! / ((n-r)! * r!) % MODULO ;
    // n!は事前にメモ化して計算済み
    // 分母の逆数(逆元)は、フェルマーの小定理により求める
    let top = memo_factorial[n];
    let bottom = ((memo_factorial[n-r]) * (memo_factorial[r])) % MODULO;
    let ncr = (top * get_inverse(bottom, MODULO)) % MODULO;
    return ncr
}

// フェルマーの小定理x^(p-1) = 1 (mod p)により逆元を求める x^(-1) = x ^ (p - 2) (mod p)
fn get_inverse(x: usize, MODULO: usize) -> usize {
    // x^(p-2)はO(p-2)の計算量がかかってしまうが、繰り返し二乗法で、O(log2(p-2))まで落とせる。
    let inverse =  iterative_square_method(x, MODULO - 2, MODULO);
    return inverse;
}

// 繰り返し2乗法 a^xを求める
fn iterative_square_method(mut a: usize, mut x: usize, MODULO: usize) -> usize {
    // answer = a ^ x を得たいとき
    //        = (a^2)^(x/2) * a^(x%2)

    // answer = 3 ^3 を得たいとき
    //        = (3^2)^(3/2) * 3^(3%2)
    //        = 9^1 * 3^1

    // answer = 3 ^ 4 を得たいとき
    //        = (3^2)^(4/2) * (3^2)^(4%2)
    //        = 9^2 * 3^0
    //        = (9^2)^(2/2) * 9^(2&2) * 3^0
    //        = 81^1 * 9^0 * 3^0

    // answer = 3 ^ 5を得たいとき
    // answer = (3^2)^(5/2) * 3^(5%2)
    //        = (3^2)^2 * 3^1
    //        = ((3^2)^2)^(2/2) * (3^2)^(2%2) * 3^1
    //        = ((3^2)^2)^1 * (3^2)^0 * 3^1
    //        = (3^4)^1 * (3^2)^0 * 3^1

    // answer = 3 ^ 7を得たいとき
    // answer = (3^2)^(7/2) * 3^(7%2)
    //        = (3^2)^3 * 3^1
    //        = 9^3 * 3^1
    //        = (9^2)^(3/2) * 9^(3%2) * 3^1
    //        = 81^1 * 9^1 * 3^1

    a %= MODULO;
    let mut answer = 1;
    while x >= 1 {
        if x % 2 == 1 {
            answer = (answer * a) % MODULO;
        }
        x = x / 2;
        a = a * a % MODULO;
    }

    return answer;
}
