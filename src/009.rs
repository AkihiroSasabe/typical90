use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
use std::f64::consts::PI;

fn main() {
    // // f64型の比較x.partial_cmp(&y)の確認: xはyとくらべて...となる。
    // let a = 1.0;
    // let b = 10.0;
    // let c = a.partial_cmp(&b).unwrap();
    // println!("a.partial_cmp(&b).unwrap(): {:?}", c); // Less
    // return;

    // // y.atan2(x)のテスト
    // // 第1象限
    // let x1: f64 = 1.0;
    // let y1: f64 = 1.73;
    // let angle1 = y1.atan2(x1)/ PI * 180.0 ;
    // println!("{}", angle1); // 60

    // // 第2象限
    // let x2: f64 = - 1.0;
    // let y2: f64 =  1.73;
    // let angle2 = y2.atan2(x2)/ PI * 180.0 ;
    // println!("{}", angle2); // 120

    // // 第3象限
    // let x3: f64 = - 1.0;
    // let y3: f64 = - 1.73;
    // let angle3 = y3.atan2(x3)/ PI * 180.0 ;
    // println!("{}", angle3); // -120

    // // 第4象限
    // let x4: f64 = 1.0;
    // let y4: f64 = - 1.73;
    // let angle4 = y4.atan2(x4)/ PI * 180.0 ;
    // println!("{}", angle4); // -60

    input! {
        n: usize,
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input! {
            x_i: f64,
            y_i: f64,
        }
        x.push(x_i);
        y.push(y_i);
    }

    
    let mut max_angle = 0.0;
    for i in 0..n {
        let mut angles = vec![];
        for j in 0..n {
            if i == j {continue}
            // 第3, 第4象限が負の角度になる。全ての角度が[-pi, pi]で表現される。
            let mut angle = (y[j] - y[i]).atan2((x[j] - x[i])) * 180.0 / PI;
            if angle < 0.0 {
                angle = 360.0 + angle;
            }
            
            // cosで計算
            // let mut angle = ((x[j] - x[i]) / ((x[j] - x[i]) * (x[j] - x[i]) + (y[j] - y[i]) * (y[j] - y[i])).sqrt()).acos() / PI * 180.0; 
            // if y[j] - y[i] < 0.0 {
            //     angle = 360.0 - angle;
            // }
            
            angles.push(angle);
        }

        // f64型のソートは非常に面倒臭い。
        // angles.sort();
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // println!("angles: {:?}", angles);

        for j in 0..(n-1) {
            // 
            let angle = angles[j];
            let mut key_angle = 180.0 + angle;
            if angle > 180.0 {
                key_angle = angle - 180.0;
            }
            let mut k_candidate_0 = angles.lower_bound(&key_angle);
            let mut k_candidate_1;
            if k_candidate_0 == 0 {
                k_candidate_1 = n - 2;
            }
            else {
                k_candidate_1 = k_candidate_0 - 1;
            }

            if k_candidate_0 == n - 1 {
                k_candidate_0 = 0;
            }
            let mut angle_candidate_0 = angles[k_candidate_0] - angle;
            let mut angle_candidate_1 = angles[k_candidate_1] - angle;
            if angle > 180.0 {
                angle_candidate_0 = - angle_candidate_0;
                angle_candidate_1 = - angle_candidate_1;
            }
            if angle_candidate_0 > 180.0 {
                angle_candidate_0 = 360.0 - angle_candidate_0;
            }
            if angle_candidate_1 > 180.0 {
                angle_candidate_1 = 360.0 - angle_candidate_1;
            }
            
            // println!("max_angle: {}", max_angle);
            // println!("angle_candidate_0: {}", angle_candidate_0);
            // println!("angle_candidate_1: {}", angle_candidate_1);

            if max_angle.partial_cmp(&angle_candidate_0).unwrap() == Ordering::Less {
                max_angle = angle_candidate_0;
            }
            if max_angle.partial_cmp(&angle_candidate_1).unwrap() == Ordering::Less {
                max_angle = angle_candidate_1;
            }
        }
    }
    println!("{}", max_angle);


    // 下記は3重ループでTLEするコード
    // let mut lines = vec![];
    // for i in 0..n {
    //     for j in i+1..n {
    //         lines.push(vec![x[i] - x[j], y[i] - y[j]]);
    //     }
    // }

    // let mut ans = 0.0;
    // for i in 0..n {
    //     for j in i+1..n {
    //         for k in j+1..n {
    //             if i == j || j ==k || k == i {continue}
    //             let angle_j = get_angle(i, j, k, &x, &y);
    //             let angle_i = get_angle(k, i, j, &x, &y);
    //             let angle_k = get_angle(j, k, i, &x, &y);

    //             // println!("inner_product, size, angle: {}, {}, {}", inner_product, size, angle);

    //             if ans.partial_cmp(&angle_j).unwrap() == Ordering::Less {
    //                 ans = angle_j;
    //             }
    //             if ans.partial_cmp(&angle_i).unwrap() == Ordering::Less {
    //                 ans = angle_i;
    //             }
    //             if ans.partial_cmp(&angle_k).unwrap() == Ordering::Less {
    //                 ans = angle_k;
    //             }
    //         }
    //     }
    // }
    // println!("{}", ans);
    
}

fn get_angle(i: usize, j: usize, k: usize, x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    // i->jベクトル
    let ij_x = x[j] - x[i];
    let ij_y = y[j] - y[i];
    // i->kベクトル
    let ik_x = x[k] - x[i];
    let ik_y = y[k] - y[i];
    // 内積
    let inner_product = ij_x * ik_x + ij_y * ik_y;
    // 大きさ
    let size = (ij_x * ij_x + ij_y * ij_y).sqrt() * (ik_x * ik_x + ik_y * ik_y).sqrt();
    let angle = (inner_product / size).acos() * 180.0 / PI;
    return angle;
}


// f64型に対応させた特別なlower_bound()!
// lower_bound=Key★以★上★のインデックス、
// upper_bound=Key★よ★り★大きいインデックス
// sorted_list.lower_bound(&x)は、x以上となる最小のインデックスを返すが、x超えがリスト内に無いときは、sorted_list.len()を返すので注意
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: PartialOrd> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].partial_cmp(x) {
                Some(Ordering::Less) => {
                    low = mid + 1;
                },
                Some(Ordering::Equal) | Some(Ordering::Greater) => {
                    high = mid;
                },
                _ => ()
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].partial_cmp(x) {
                Some(Ordering::Less) | Some(Ordering::Equal) => {
                    low = mid + 1;
                },
                Some(Ordering::Greater) => {
                    high = mid;
                },
                _ => ()
            }
        }
        low
    }
}