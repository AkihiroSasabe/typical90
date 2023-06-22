use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::collections::BinaryHeap;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }
    // 考察
    // 例えばk=kのとき、斜めが赤になるケースを考える
    // S = RGBGR
    // T = GRGRB

    // S[i] + T[i+k] = R
    // R + R = R
    // B + G = R
    // G + B = R

    // ここでTについて、G->B, B->Gと置き換えれば
    // R + R = R
    // B + B = R
    // G + G = R
    // となり、i=1,2,...,N-kの全てのiで
    // S[i] == T[i+k] が成立すれば良い.
    // つまり、SとTの部分文字列が一致していれば、k=kのときの斜めが一色であることが判定可能(S[1,N-k] == T[1+k, N])
    // ただし、愚直にやると1本の判定にO(N)掛かってしまう。そこでローリングハッシュで判定する。

    // ローリングハッシュ
    // 部分文字列S[left_s, right_s]と部分文字列T[left_t, right_t]が一致しているかを調べたい。
    // 文字列SとTが一致しているかは、min(S.len(), T.len())の計算量を必要としてしまう。
    // 文字列をハッシュ化(数値化)してしまえば、O(1)で一致判定できる。

    // brute-force (O(N^2)なのでTLEする)
    // let mut a = vec![vec!['.'; n]; n];
    // for i in 0..n {
    //     for j in 0..n {
    //         if s[i] == t[j] {
    //             a[i][j] = s[i];
    //         }
    //         else {
    //             a[i][j] = get_unselected_color(s[i], t[j]);
    //         }
    //     }
    //     println!("{:?}", a[i]);
    // }

    
    // Tを変形する
    let t_r = transform_t(&t, 'R'); // G<=>B を交換
    let t_g = transform_t(&t, 'G'); // B<=>R を交換
    let t_b = transform_t(&t, 'B'); // R<=>G を交換
    // println!("t  ={:?}", t);
    // println!("t_r={:?}", t_r);
    // println!("t_g={:?}", t_g);
    // println!("t_b={:?}", t_b);
    
    // 以下ローリングハッシュによる解法 (ハッシュの衝突に注意。1 / modulusの確率で起きる。テストケースが甘いことを祈るのみ)
    // 適当な素数
    let modulus = 2_147_483_647; // 2 * 10^9

    // base(=100)の累乗を事前計算
    let base = 100; // baseの値は文字の種類数以上なら何でも良い
    let exponent = s.len(); // 指数は文字数-1だけ必要
    let power_of_base = get_power_of_base(base, exponent, modulus);

    // 先頭部分文字列のhash値を事前計算しておく (先頭からi文字目まで。i=0, 1, 2, ..., n-1)
    let first_hash_list_s = get_first_substring_hash(&s, base, modulus);
    let first_hash_list_tr = get_first_substring_hash(&t_r, base, modulus);
    let first_hash_list_tg = get_first_substring_hash(&t_g, base, modulus);
    let first_hash_list_tb = get_first_substring_hash(&t_b, base, modulus);

    let mut ans = 0;
    // kが正のとき
    for k in 0..n {
        // Sの0, n-1-kまでのハッシュ値
        let s_hash = get_substring_hash(&first_hash_list_s, 0, n-1-k, &power_of_base, modulus);
        // Tのk, n-1までのハッシュ値
        let tr_hash = get_substring_hash(&first_hash_list_tr, k, n-1, &power_of_base, modulus); // 赤
        let tg_hash = get_substring_hash(&first_hash_list_tg, k, n-1, &power_of_base, modulus); // 緑
        let tb_hash = get_substring_hash(&first_hash_list_tb, k, n-1, &power_of_base, modulus); // 青

        // 赤で一致するか?
        if s_hash == tr_hash {
            ans += 1;
        }
        // 緑で一致するか?
        if s_hash == tg_hash {
            ans += 1;
        }
        // 青で一致するか?
        if s_hash == tb_hash {
            ans += 1;
        }
    }

    // kが負のとき
    for k in 1..n {
        // Sのk, n-1までのハッシュ値
        let s_hash = get_substring_hash(&first_hash_list_s, k, n-1, &power_of_base, modulus);
        // Tの0, n-1-kまでのハッシュ値
        let tr_hash = get_substring_hash(&first_hash_list_tr, 0, n-1-k, &power_of_base, modulus); // 赤
        let tg_hash = get_substring_hash(&first_hash_list_tg, 0, n-1-k, &power_of_base, modulus); // 緑
        let tb_hash = get_substring_hash(&first_hash_list_tb, 0, n-1-k, &power_of_base, modulus); // 青

        // 赤で一致するか?
        if s_hash == tr_hash {
            ans += 1;
        }
        // 緑で一致するか?
        if s_hash == tg_hash {
            ans += 1;
        }
        // 青で一致するか?
        if s_hash == tb_hash {
            ans += 1;
        }
    }

    println!("{}", ans);

}


// ローリングハッシュ
fn get_substring_hash(first_hash_list: &Vec<usize>, left: usize, right: usize, power_of_base: &Vec<usize>, modulus: usize) -> usize {
    // 任意の部分文字列string[l,r]のhash値を取得する. rは閉区間(rを含む)ので注意。開区間ではない。
    // first_hash_list: r=0,1,2,...nのstring[0,r]について、
    //     先頭文字列のハッシュ値を事前に計算して格納したリスト
    // left: 先頭index
    // right: 末尾のindex
    // power_of_base: 底の累乗を計算したリスト。最大指数は文字数-1
    // modulus: 素数。ハッシュ化時に数が膨大過ぎてオーバーフローするのを防ぐため、この素数の剰余を取る

    // 例
    // r = 5
    // l = 2
    //    0123456 <-index
    //    1234567 <-value
    // S="abcdefg"
    // h[5] = 1*base^5 + 2*base^4 + 3*base^3 + 4*base^2 + 5*base^1 + 6*base^0 <-先頭から6文字目までのハッシュ値
    // h[1] = 1*base^1 + 2*base^0 <-先頭から2文字目までのハッシュ値
    // h[2,5] = h[5] - h[1] * base^(4)       <- 具体例
    // h[l,r] = h[r] - h[l-1] * base^(r-l+1) <- 一般化

    if left == 0 {
        return first_hash_list[right];
    }

    let substring_hash = (modulus + first_hash_list[right] - first_hash_list[left-1] * power_of_base[right-left+1] % modulus) % modulus;
    
    return substring_hash
}

// ローリングハッシュで、文字列をべき乗級数として計算する際に必要な事前計算
fn get_power_of_base(base: usize, exponent: usize, modulus: usize) -> Vec<usize> {
    // baseのべき乗を事前計算する
    // base: 底
    // exponent: 最大指数. 正確にはexponent-1乗まで求める
    // modulus: 法

    let mut power_of_base =  vec![1_usize; exponent];
    for i in 1..exponent {
        power_of_base[i] = power_of_base[i-1] * base % modulus;
    }
    return power_of_base
}

// hash: 他動詞: [1]細かく刻む, [2]〔文字列などを〕値に変換する
fn get_first_substring_hash(string: &Vec<char>, base: usize, modulus: usize) -> Vec<usize> {
    // 先頭からの部分文字列のhash値を取得する関数(ローリングハッシュの前処理)
    // <=> string[0,r] (r=0,1,2,...n)についての文字列のハッシュ値を格納したリストを返す
    
    // 入力引数
    // string: 文字列
    // base: 底. base進法. baseの値は文字の種類数以上なら何でも良い
    // modulus: 法


    // ハッシュ化の方法: 
    // 長さNで使用文字種類数Bの文字列は、
    // N桁のB進法(例えば100進法. Bは登場する文字の種類数以上なら何でも良い。アルファベットならB=26で良い。)の数字で一意に表現可能。
    // ハッシュ値 = 100^(N-1) * string[0] + 100^(N-2) * string[1] + ... + 100^(0) * string[N-1]
    // ただしB=100でN=1,000文字の文字列は、ハッシュ値が(100^1000)-1=(10^2000)-1で、64bit(≒10^19)を遥かに超えてしまう。

    // そこで適当な素数Mで割った余りをハッシュ値とする。
    // ハッシュ衝突する確率(異なる文字列が同じハッシュ値になる確率)は1/Mであり非常に小さいため、
    // ハッシュ衝突したときだけ。生の文字列のまま比較する。
    
    // left=0, right=i (i=0, 1, ..., n-1)を計算
    let mut first_hash_list = vec![0_usize; string.len()];
    first_hash_list[0] = (string[0] as usize - 'A' as usize + 1) % modulus;
    for i in 1..string.len() {
        // 文字を数字に変換('A'=1, 'B'=2, ...になるような感じで計算)
        let ci = (string[i] as usize - 'A' as usize + 1) % modulus;
        first_hash_list[i] = (first_hash_list[i-1] * base % modulus + ci) % modulus;
    }

    return first_hash_list
}


// 文字列Tの、指定した色以外の2色同士を置換する
fn transform_t(t: &Vec<char>, color: char) -> Vec<char> {
    // 例: Rを指定したらGとBを置換するので、RRGGBB を RRBBGG にして返す
    let rgb = vec!['R','G','B'];
    let mut input_index = 4; // 適当に初期化
    for i in 0..3 {
        if color == rgb[i] {
            input_index = i;
        }
    }
    
    let mut transformed_t = vec![];
    for i in 0..t.len() {
        if t[i] == rgb[(input_index + 1) % 3] {
            transformed_t.push(rgb[(input_index + 2) % 3]);
        }
        else if t[i] == rgb[(input_index + 2) % 3] {
            transformed_t.push(rgb[(input_index + 1) % 3]);
        }
        else {
            transformed_t.push(rgb[input_index]);
        }
    }

    return transformed_t
}


// デバッグ用
fn get_unselected_color(x: char, y: char) -> char {
    if x == 'R' && y == 'G' || y == 'R' && x == 'G' {
        return 'B'
    }
    else if x == 'G' && y == 'B' || y == 'G' && x == 'B' {
        return 'R'
    }
    else {
        return 'G'
    }
}