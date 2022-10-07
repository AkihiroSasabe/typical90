use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        h: usize,
        w: usize,
        rs: usize,
        cs: usize,
        rt: usize,
        ct: usize,
        s: [Chars; h]
    }

    // 上下左右の動き (正確には左上右下)
    let cross: Vec<[isize; 2]> = vec![[0,-1], [-1,0], [0,1], [1,0]];
    let serialized_cross = vec![-1, - (w as isize), 1, w as isize];

    // BFS
    let INF: usize = 1 << 60;
    // todoとdistには、(1)頂点番号に加えて、(2)向き(現在向いている方向)も持たせる
    let mut todo: VecDeque<Vec<usize>> = VecDeque::new();
    // distの定義は、dist[頂点][向き]で、頂点がある向きを向くのに必要なコストが格納。
    let mut dist = vec![vec![INF; 4]; h*w];
    let start_v = serialize(rs - 1, cs - 1, w);    
    let terminal_v = serialize(rt - 1, ct - 1, w);
    dist[start_v] = vec![0, 0, 0, 0];
    // 方向の指定
    for orient in 0..4 {
        todo.push_back(vec![start_v, orient]);
    }
    while todo.len() != 0 {
        let state = todo.pop_front().unwrap();
        let v = state[0];
        let orient = state[1];
        // 次の頂点は、現在の頂点位置から、現在の頂点が向いている方向に行く
        let next_v = (v as isize + serialized_cross[orient]) as usize;
        let (mut y, mut x) = decode(v, w);
        let next_y = y as isize + cross[orient][0];
        let next_x = x as isize + cross[orient][1];
        
        // 次の探索頂点位置が、画像からはみ出てたらスキップ
        if !check_inside_map(next_y, next_x, h as isize, w as isize, &s) {continue}
        // 次の探索頂点位置が、壁だったらスキップ
        if s[next_y as usize][next_x as usize] == '#' {continue}
        for next_orient in 0..4 {
            let before = dist[next_v][next_orient];
            if orient == next_orient {
                dist[next_v][next_orient] = min(dist[v][orient], dist[next_v][next_orient]);
            }
            else {
                dist[next_v][next_orient] = min(dist[v][orient] + 1, dist[next_v][next_orient]);
            }
            // distが更新されたらtodoに追加
            // 既に探索済みの頂点でも別の方向から入ったときは更新されてtodoに追加されることがある。
            let updated = (before != dist[next_v][next_orient]);
            if updated {
                // println!("updated!: next_y: {} next_x: {}, orient: {}, value: {}", next_y, next_x, next_orient, dist[next_v][next_orient]);
                todo.push_back(vec![next_v, next_orient]);
            }
        }
    }

    let mut answer = INF;
    for d in dist[terminal_v].iter() {
        answer = min(answer, *d);
    }
    println!("{}", answer);
    // println!("{}", terminal_v);

    // for i in dist.iter() {
    //     println!("{:?}", i);
    // }


}

// 指定した座標が画像の枠に収まっているか確認
fn check_inside_map(y: isize, x: isize, h: isize, w: isize, s: & Vec<Vec<char>>) -> bool {
    if y < 0 || h <= y || x < 0 || w <= x {
        return false
    }
    else {
        return true
    }
}

// 画像上の座標を2次元で指定(y, x)したものを、1次元の値(左上から何番目か)で表現
fn serialize(y: usize, x: usize, w: usize) -> usize {
    return x + y * w
}

// 1次元化された画像上の座標を、2次元(y, x)で返す
fn decode(serialized_num: usize, w: usize) -> (usize, usize) {
    let y = serialized_num / w;
    let x = serialized_num % w;
    return (y, x)
}