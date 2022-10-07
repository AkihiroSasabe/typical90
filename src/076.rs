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
        mut a: [isize; n]
    }

    let sum: isize = a.iter().sum();

    if sum % 10 != 0 {
        println!("No");
        return;
    }

    let target_size = sum / 10;
    // println!("target_size: {}", target_size);

    // aを2倍の長さにする
    a.append(&mut (a.clone()));
    let mut acc: Vec<isize> = vec![0; 2*n];
    // acc[i]はindex=0 ~ iまでの和
    acc[0] = a[0];
    for i in 1..2*n {
        acc[i] = acc[i-1] + a[i];
    }
    
    for i in 0..n {
        if i == 0 {
            let index = acc[i..i+n].lower_bound(&target_size);
            // println!("i: {}, index: {}, acc[index+i]: {} ",i, index, acc[index+i]);
            if target_size == acc[index+i] {
                println!("Yes");
                return;
            }
        }
        else {
            let index = acc[i..i+n].lower_bound(&(target_size + acc[i-1]));
            // println!("i: {}, index: {}, acc[index+i]: {}, acc[i-1]: {}",i, index, acc[index+i], acc[i-1]);
            if target_size == acc[index+i] - acc[i-1] {
                println!("Yes");
                return;
            }
        }
        
    }
    println!("No");
}


// lower_bound=Key★以★上★のインデックス、
// upper_bound=Key★よ★り★大きいインデックス
// sorted_list.lower_bound(&x)は、x以上となる最小のインデックスを返すが、x超えがリスト内に無いときは、sorted_list.len()を返すので注意
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
