use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize
    }
    let modulus = 1_000_000_007;

    use crate::combination::Combination;
    let comb = Combination::new(n, modulus);

    // 計算量は、O(n + n/2 + n/3 + ...) = O(nlogn)
    for k in 1..(n+1) {
        let mut count = 0;
        // x := 選ぶボールの数
        // n個のボールの中から、x個選ぶとき、選んだボールのどの2つもk以上離れているとき、
        // 1 <=> 1+k <=> 1+2k みたいになってないといけない。
        // (例: k=3, n=12) 1 <=> 4 <=> 7 <=> 10... 
        // そうすると、n以下の中で区間3をどれだけ詰め込めるかって話で、1から始まるので、-1すれば、(n-1) / k 個
        // ボールの数は、区間の数+1だから、(n-1) / k + 1 (ループでは開区間なので、+1する必要がある。)
        for x in 1..((n-1)/k+2) {
            // N個の中から、差がk以上で、x個取ることを考える。
            // 例:(N, k, x) = (9, 3, 3) の場合
            // 1,2,3,4,5,6,7,8,9 の中で、
            // 1,_,_,4,5,6,7,8,9 <= 1を選択した場合の残りの候補
            // 1,_,_,4,5,_,_,8,9 <= 1,5,を選択した場合の残りの候補
            // 1,_,_,4,5,_,_,8,9 <= 1,5,9を選択した場合の残りの候補
            // となる。
            // あるpを取ると、p + k - 1 までの数字を選べなくなるが、それがx-1個あるので、
            // (k-1) * (x-1) 個を除いた部分から、x個選ぶことと同義になる。つまり、
            // 1,2,3,4,5 から3個取るのと同義。
            // 1,_,_,2,3,_,_,4,5 (1,3,5を取った場合)
            // 1,_,_,4,5,_,_,8,9 
            count += comb.get_comb(n - (x-1) * (k-1), x);
            count %= modulus;
        }
        println!("{}", count % modulus);
    }

    // for k in 1..(n+1) {
    //     let mut count = 0;
    //     if k== 1 {
    //         println!("{}", 2_u32.pow(n as u32) - 1);
    //     }
    //     else {
    //         for x in 0..n {
    //             dfs(x, n, k, &mut count, modulus);
    //         }
    //         println!("{}", count % modulus);
    //     }
    // }
}


pub mod combination {
    #[derive(Debug, Clone)]
    pub struct Combination {
        modulus: usize,
        factorial: Vec<usize>,
        inv_factorial: Vec<usize>
    }
    impl Combination {
        pub fn new(max_n: usize, modulus: usize) -> Self {
            let (factorial, inv_factorial) = get_factorial(max_n, modulus);
            Combination {
                modulus,
                factorial,
                inv_factorial
            }
        }
        // nCrを求める
        pub fn get_comb(&self, n: usize, r: usize) -> usize {
            // nCr = n! / ((n-r)! * r!) % modulus ;
            // n!は事前にメモ化して計算済み
            // 分母の逆数(逆元)は、フェルマーの小定理により求める
            let top = self.factorial[n];
            let bottom = self.inv_factorial[n-r] * self.inv_factorial[r] % self.modulus;
            let ncr = (top * bottom) % self.modulus;
            return ncr
        }
    }

    // フェルマーの小定理x^(p-1) = 1 (mod p)により逆元を求める x^(-1) = x ^ (p - 2) (mod p)
    pub fn get_inverse(x: usize, modulus: usize) -> usize {
        // x^(p-2)はO(p-2)の計算量がかかってしまうが、繰り返し二乗法で、O(log2(p-2))まで落とせる。
        let inverse =  iterative_square_method(x, modulus - 2, modulus);
        return inverse;
    }

    fn get_factorial(max_n: usize, modulus: usize) -> (Vec<usize>, Vec<usize>) {
        // n!を格納した配列
        let mut factorial = vec![1; max_n+1];
        let mut inv_factorial = vec![1; max_n+1];
        for i in 1..(max_n+1) {
            factorial[i] = (i * factorial[i-1]) % modulus;
        }
        inv_factorial[max_n] = get_inverse(factorial[max_n], modulus);
        for i in 1..max_n {
            inv_factorial[max_n - i] = inv_factorial[max_n - i + 1] * ((max_n - i + 1) % modulus);
            inv_factorial[max_n - i] %= modulus;
        }

        return (factorial, inv_factorial)
    }
    
    // 繰り返し2乗法 a^xを求める
    fn iterative_square_method(mut a: usize, mut x: usize, modulus: usize) -> usize {
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
    
        a %= modulus;
        let mut answer = 1;
        while x >= 1 {
            if x % 2 == 1 {
                answer = (answer * a) % modulus;
            }
            x = x / 2;
            a = a * a % modulus;
        }
    
        return answer;
    }
}


// N = 7
// k = 2

// {1 2 3 4 5 6 7}

// {1 _ _ _ _ _ _}

// {1 _ 3 _ _ _ _}
// {1 _ _ 4 _ _ _}
// {1 _ _ _ 5 _ _}
// {1 _ _ _ _ 6 _}
// {1 _ _ _ _ _ 7}

// {1 _ 3 _ 5 _ _}
// {1 _ 3 _ _ 6 _}
// {1 _ 3 _ _ _ 7}

// {1 _ 3 _ 5 _ 7}

// fn dfs(mut x: usize, n: usize, k: usize, count: &mut usize, modulus: usize) {
//     *count += 1;
//     *count %= modulus;

//     x += k;
//     while x < n {
//         dfs(x, n, k, count, modulus);
//         x += 1;
//     }
// }