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
        c: [Chars; h]
    }

    let mut max_depth = 0;
    for i in 0..h {
        for j in 0..w {
            // (i, j)を始点と終点とする。
            if c[i][j] == '#' {continue}
            let mut seen = vec![vec![false; w]; h];
            dfs(& c, i, j, 0, &mut seen, h, w, i, j, &mut max_depth)
        }
    }
    if max_depth == 0 {
        println!("-1");
    }
    else {
        println!("{}", max_depth);
    }

}


fn dfs(map: &Vec<Vec<char>>, y: usize, x: usize, current_depth: usize, seen: &mut Vec<Vec<bool>>, h: usize, w: usize, y_end: usize, x_end: usize, max_depth: &mut usize) {
    // println!("y: {}, x: {}, current_depth: {}, y_end: {}, x_end: {}", y, x, current_depth, y_end, x_end);
    seen[y][x] = true;
    // 左上右下
    let dir: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];
    for i in 0..4 {
        let next_y = y as isize + dir[i][0];
        let next_x = x as isize + dir[i][1];
        // 画像の外
        if next_y < 0 || h as isize <= next_y || next_x < 0 || w as isize <= next_x {continue}

        // キャスト
        let next_y = next_y as usize;
        let next_x = next_x as usize;
        
        // 始点(終点)にたどり着いたとき
        if next_y == y_end && next_x  == x_end && 3 <= current_depth + 1 {
            *max_depth = max(*max_depth, current_depth + 1);
            // println!("current max!: {}", current_depth + 1);
            continue
        }
        // 既に通過済み
        if seen[next_y][next_x] {continue}
        // 山
        if map[next_y][next_x] == '#' {continue}
        dfs(map, next_y, next_x, current_depth + 1, seen, h, w, y_end, x_end, max_depth);
    }
    // バックトラック := 頂点vより先を全て調べ終わったら(行き止まりに突き当たったら)、1手戻る
    // => 帰りがけに、seenをもとに戻す
    seen[y][x] = false;

}