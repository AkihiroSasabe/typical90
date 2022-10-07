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
        mut a: [isize; n],
        q: usize,
        b: [isize; q]
    }

    a.sort();
    for i in 0..q {
        let index = a.lower_bound(&b[i]);
        // println!("index is {}", index);
        let mut answer;
        if index == n {
            answer = (a[index-1]-b[i]).abs();
        }
        else if index == 0 {
            answer = (a[index] - b[i]).abs();
        }
        else {
            answer = min((a[index] - b[i]).abs(), (a[index-1]-b[i]).abs());
        } 
        println!("{}", answer);
    }
}



// lower_bound=Key★以★上★のインデックス、upper_bound=Key★よ★り★大きいインデックス
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
