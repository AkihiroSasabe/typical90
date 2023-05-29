use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;

fn main() {
    // E8氏の023_04b.cppをrustに翻訳
    // usedのyとxがぐちゃぐちゃでひどかったので修正

    // ありえる状態の数が、フィボナッチ数の第N項=fib(N)であることの証明: 
    // 1が連続して来ない長さNの数列の個数をdp[N]とすると、
    // dp[N] = 下1桁が1 + 下1桁が0
    //       = 下2桁が01 + 下1桁が0(下2桁が00でも10でもok)
    //       = dp[N-2] + dp[N-1]
    // となる。故にdp[N]はフィボナッチ数列の第N項=fib(N)である。

    // 今、W個のマスの状態についてかんがえればいいので、あり得る状態数は、
    // fib(W) = 1.62^W <= 1.62^25 = 172,931.9...
    // 1 << 18 = 262,144
    // ゆえにfib(W) < (1 << 18)

    // Step1: 入力
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let MODULUS = 1_000_000_007;

    // Map
    let mut cnt = vec![0; w + 1];
    let mut used = vec![vec![false; 25]; 25];
    let mut next_is_0 = vec![vec![0; 1 << 18]; w + 1];
    let mut next_is_1 = vec![vec![0; 1 << 18]; w + 1];
    let mut state = vec![vec![0; 1 << 18]; w + 1];
    let mut hash: Vec<HashMap<usize, (isize, bool)>> = vec![HashMap::new(); w + 1];

    // dp[y][x][置いた状態]
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 1 << 18]; 25]; 25];

    // step2: 初期化
    // 一番上の行について初期化
    for i in 0..w {
        // println!("i = {}", i);
        dfs(i, 0, 0, w, h, &mut cnt, &mut used, &mut state, &mut hash);
    }
    for i in 0..w {
        for j in 0..cnt[i] {
            // 次見るマスがiのときのj番目の状態
            // 例: w=5で[0,0,1,0,1] = [置かない, 置かない, 置く, 置かない, 置く]
            let t = state[i][j];

            // 置かないときの状態
            // 例: w=5で[0,1,0,1,0] // 上のtの後ろ4桁0,1,0,1を左に詰めて、右から0を追加
            let t0 = (t >> 1);

            // 置くときの状態
            // 例: w=5で[0,1,0,1,1] // 上のtの後ろ4桁0,1,0,1を左に詰めて、右から1を追加
            let t1 = (t >> 1) + (1 << w);

            // 次見るマスに、キングを置かないときの状態番号
            next_is_0[i][j] = hash[(i+1) % w][&t0].0;
            // 次見るマスに、キングを置くときの状態番号
            if hash[i][&t].1 {
                // 次見るマスに、キングを置けるとき
                next_is_1[i][j] = hash[(i+1) % w][&t1].0;
            }
            else {
                // 次見るマスに、キングを置けないとき
                next_is_1[i][j] = -1;
            }
        }
    }

    // step3: DP
    dp[0][0][0] = 1;
    for y in 0..h {
        for x in 0..w {
            let mut next_y = y;
            let mut next_x = x + 1;
            if next_x == w {
                next_y += 1;
                next_x = 0;
            }

            for k in 0..cnt[x] {
                if dp[y][x][k] == 0 {continue}
                dp[next_y][next_x][next_is_0[x][k] as usize] += dp[y][x][k];
                dp[next_y][next_x][next_is_0[x][k] as usize] %= MODULUS;
                if next_is_1[x][k] != - 1 && c[y][x] == '.' {
                    dp[next_y][next_x][next_is_1[x][k] as usize] += dp[y][x][k];
                    dp[next_y][next_x][next_is_1[x][k] as usize] %= MODULUS;
                }
            }
        }
    }

    // step4: 正解出力
    let mut ans = 0;
    for i in 0..cnt[0] {
        ans += dp[h][0][i];
        ans %= MODULUS;
    }
    println!("{}", ans);
}


// [sy, sx]にキングを置いても、キング同士が互いに攻撃しないか判定 (true: 攻撃しない、false: 攻撃する)
// <=> 隣 [sy, sx]の周囲8方向にキングが居ないかを判定
fn hantei(sy: usize, sx: usize, w: usize, h: usize, used: &Vec<Vec<bool>>) -> bool {
    // 周囲8方向: 1左下、2下、3右下、4右、5右上、6上、7左上、8左
    let dy = vec![1, 1, 1, 0, -1, -1, -1, 0];
    let dx = vec![-1, 0, 1, 1, 1, 0, -1, -1];
    for i in 0..8 {
        let ty = sy as isize + dy[i];
        let tx = sx as isize + dx[i];
        if tx < 0 || ty < 0 || tx >= w as isize {continue}
        // if tx < 0 || ty < 0 || tx >= w as isize || ty >= h as isize {continue} // 何故かtyの条件を加えるとREする
        if used[ty as usize][tx as usize] {
            return false;
        }
    }
    return true
}

// 次に置く1個のマスと、その前にいるw+1個のマスの合計w+2個のマスについて考える。
fn dfs(position: usize, depth: usize, str: usize, w: usize, h: usize, cnt: &mut Vec<usize>, used: &mut Vec<Vec<bool>>, state: &mut Vec<Vec<usize>>, hash: &mut Vec<HashMap<usize, (isize, bool)>>) {

    let sy = position / w;
    let sx = position % w;
    // println!("sx: {}, sy: {}, position: {}, depth: {}, str: {}", sx, sy, position, depth, str);

    // 次見るマスが最後のw+2個目に到着
    if depth == w + 1 {
        let idx = cnt[sx];
        let flag = hantei(sy, sx, w, h, used);
        // println!("_^_depth = w + 1 = {}, sx = {}, idx = {}, flag = {}", w + 1, sx, idx, flag);
        // 次見るマスがsxのとき、idx番目の状態はstr(=w+2個のマスについてのキングの有無)
        state[sx][idx] = str;
        // 次見るマスがsxのとき、状態strはidx番目の状態であり、そこにキングを置けるか?=>flag
        hash[sx].insert(str, (idx as isize, flag));
        // 次見るマスがsxのとき(y方向は任意)のケースを1個加算
        cnt[sx] += 1;
        return
    }

    // position + 1個目にキングを置かないケース
    dfs(position + 1, depth + 1, str, w, h, cnt, used, state, hash);

    // position + 1個目にキングを置くケース
    if hantei(sy, sx, w, h, used) {
        used[sy][sx] = true; // (sy, sx)にキングを置く
        // strは現在の状態を表している. depthの位置にキングを置いたら1 << depthのビットが加算される
        dfs(position + 1, depth + 1, str + (1 << depth), w, h, cnt, used, state, hash);
        used[sy][sx] = false; // (sy, sx)にキングを置かない
    }
}

