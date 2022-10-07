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
    let mut g = vec![vec![]; n];
    for i in 0..(n-1) {
        input! {
            a_i: usize,
            b_i: usize,
        }
        g[a_i - 1].push(b_i - 1);
        g[b_i - 1].push(a_i - 1);
    }
    let (max_dist_from_0, max_v_from_0) = get_max_dist(&g, 0, n);
    // dbg!(max_dist_from_0, max_v_from_0);
    let (max_dist, max_v) = get_max_dist(&g, max_v_from_0, n);
    // dbg!(max_dist, max_v);
    println!("{}", max_dist + 1);

}


// 幅優先探索
fn get_max_dist(g: &Vec<Vec<usize>>, start_v: usize, graph_size: usize) -> (usize, usize) {

    let mut todo: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![0; graph_size];
    let mut max_dist = 0;
    let mut max_v = start_v;
    todo.push_back(start_v);
    while !todo.is_empty() {
        let v = todo.pop_front().unwrap();
        for next_v in g[v].iter() {
            if dist[*next_v] != 0 || *next_v == start_v {continue}
            todo.push_back(*next_v);
            dist[*next_v] = dist[v] + 1;
            if dist[*next_v] > max_dist {
                max_dist = dist[*next_v];
                max_v = *next_v;
            }
        }
    }
    return (max_dist, max_v)
}







// 駄目な解放
// fn main() {
//     input! {
//         n: usize,
//     }

//     let mut g = vec![vec![]; n];
//     for i in 0..(n-1) {
//         input! {
//             a_i: usize,
//             b_i: usize,
//         }
//         g[a_i - 1].push(b_i - 1);
//     }
//     // println!("{:?}", g);
//     let mut answer = 0;
//     for v in 0..n {
//         let mut seen = vec![false; n];
//         let (max_depth, second_depth) = dfs(&mut g, v, &mut seen, 0);
//         dbg!(max_depth, second_depth);
//         answer = max(answer, max_depth + second_depth + 1);
//     }
//     println!("{}", answer);

// }

// // 深さ優先探索
// fn dfs(g: & Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, current_depth: usize) -> (usize, usize) {
//     seen[v] = true;

//     let mut depth_list = vec![current_depth];
//     for next_v in g[v].iter() {
//         if seen[*next_v] {continue}
//         let (max_depth, _) = dfs(g, *next_v, seen, current_depth+1);
//         depth_list.push(max_depth);
//     }
//     depth_list.sort();

//     let max_depth = depth_list[depth_list.len() - 1];
//     let second_max_depth;
//     if depth_list.len() >= 2 {
//         second_max_depth = depth_list[depth_list.len() - 2];
//     }
//     else {
//         second_max_depth = 0;
//     }

//     return (max_depth, second_max_depth)
// }