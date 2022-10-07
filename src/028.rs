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
    let mut lx = vec![];
    let mut ly = vec![];
    let mut rx = vec![];
    let mut ry = vec![];
    for i in 0..n {
        input! {
            lx_i: usize,
            ly_i: usize,
            rx_i: usize,
            ry_i: usize,
        }
        lx.push(lx_i);
        ly.push(ly_i);
        rx.push(rx_i);
        ry.push(ry_i);
    }

    let height = 1001;
    let width = 1001;
    // let height = 8; // debug
    // let width = 8; // debug
    let mut area_map: Vec<Vec<isize>> = vec![vec![0; width]; height];

    // いもす法
    for i in 0..n {
        area_map[ly[i]][lx[i]] += 1;
        area_map[ly[i]][rx[i]] -= 1;
        area_map[ry[i]][lx[i]] -= 1;
        area_map[ry[i]][rx[i]] += 1;
    }

    // println!("area map:0");
    // for i in 0..height {
    //     println!("{:?}", area_map[i]);
    // }

    // 横方向に累積和を取る
    // let mut acc_map: Vec<Vec<isize>> = vec![vec![0; width]; height];
    for i in 0..height {
        for j in 0..(width-1) {
            area_map[i][j+1] += area_map[i][j];
        }
    }

    // println!("area map:1");
    // for i in 0..height {
    //     println!("{:?}", area_map[i]);
    // }

    // 縦方向に累積和を取る
    for i in 0..(height-1) {
        for j in 0..width {
            area_map[i+1][j] += area_map[i][j];
        }
    }

    // println!("area map:2");
    // for i in 0..height {
    //     println!("{:?}", area_map[i]);
    // }

    let mut size_list = vec![0; n+1];
    // let mut k_planes_area = 0; 
    for i in 0..height {
        for j in 0..width {
            size_list[area_map[i][j] as usize] += 1;
            // if area_map[i][j] == k {
            //     k_planes_area += 1;
            // }
        }
    }
    for i in 1..n+1 {
        println!("{}", size_list[i]);
    }
    
}


