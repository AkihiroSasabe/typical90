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
    let mut points = vec![];
    for i in 0..n {
        input! {
            x_i: isize,
            y_i: isize,
        }
        points.push(vec![x_i, y_i]);
    }

    // 凸包上の点 (入力する点が3個以上あるので、Noneが返されることはなく、unwrap()して良い。)
    let convex_hull = andrews_monotone_chain_convex_hull_algorithm(&mut points, true).unwrap();
    
    // 凸包内の面積(の2倍)を求める。凸包を"凸包の頂点数-2"個の三角形に分割( P0, Pi, Pi+1 からなる三角形)して、各面積を足し合わせる。
    let mut area_x2 = 0;
    // P0の座標
    let x0 = convex_hull[0][0];
    let y0 = convex_hull[0][1];
    for i in 1..convex_hull.len()-1 {
        // Piの座標
        let xi = convex_hull[i][0];
        let yi = convex_hull[i][1];
        // Pi+1の座標
        let xip1 = convex_hull[i+1][0];
        let yip1 = convex_hull[i+1][1];
        
        // ベクトルP0->Pi=(a,b), P0->Pi+1=(c,d)を求める
        let a = xi - x0;
        let b = yi - y0;
        let c = xip1 - x0;
        let d = yip1 - y0;
        
        // 外積で面積を求める(三角形の面積を求めるために2で割ると小数になってしまうので、一旦平行四辺形の面積を求める)
        let area_i = - (a*d - b*c); // 凸包上の点は時計回りに格納されているので、外積は負になり、正の面積を得る為に-1を掛けている。
        area_x2 += area_i;
    }

    // Pick's theorem (ピックの定理, 証明: https://www.koshi-h.ed.jp/wp-content/uploads/2019/02/H30_10_picks_theorem.pdf)
    // A = I + B/2 - 1 // (A: 多角形の面積, I: 多角形の内側にある格子点数, B: 多角形の外周上の頂点数(格子点数))
    // I = A - B/2 + 1
    // A: the area of the polygon
    // I: the number of integer points interior to the polygon.
    // B: the number of integer points on its boundary (including both vertices and points along the sides).

    // 凸包の辺上にある格子点 (凸包の頂点は除く) の数を求める
    let mut betweens = 0;
    for i in 0..convex_hull.len() {
        // 隣り合う凸包の頂点PiとPi+1の座標を求める
        // 頂点Piの座標
        let x1 = convex_hull[i][0];
        let y1 = convex_hull[i][1];
        // 頂点Pi+1の座標
        // 最後の頂点P_lastはP0と繋げる必要があるので剰余を取る
        let x2 = convex_hull[(i+1) % convex_hull.len()][0];
        let y2 = convex_hull[(i+1) % convex_hull.len()][1];

        // Pi->Pi+1 = (x,y) = (x2-x1, y2-y1)のベクトル上に格子点は(x,y)の最大公約数-1個ある。
        // 例えば(x, y) = (12,4)だったら最大公約数は4であり(12,4) = 4 *(3,1)となるから間にいる格子点数は4-1=3個
        let x = (x2-x1).abs() as usize;
        let y = (y2-y1).abs() as usize;
        let gcd = gcd(x, y);
        let between = gcd - 1;
        // println!("x={}, y={}, gcd={}, between={}", x, y, gcd, between);
        betweens += between;
    }

    let B = convex_hull.len() + betweens;

    // Iは必ず整数になることに注意
    // area_x2の面積が2で割り切れるとき、B/2は整数にならないので、ピックの定理をそのまま適応可能
    // area_x2の面積が2で割り切れないとき、商の小数点部分は0.5だが、Iが整数なのでB/2の小数点部分も0.5になる
    // 両変数の小数点部分は0.5なので結局相殺されるので、ピックの定理はそのまま適応可能
    let I = area_x2 as usize / 2 + 1 - B / 2 ; // 1が左に行くと負になってoverflowしてしまう。
    // println!("B: {}, I: {}, 2S: {}", B, I, area_x2);
    println!("{}", B + I - n);

}


// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(x: usize, y:usize) -> usize {
    if y == 0 {
        // 任意の整数xは0の約数と言える(∵0 % x == 0)ので、0とxの最大公約数はx
        return x;
    }
    else {
        return gcd(y, x % y);
    }
}

// 凸包をO(NlogN)で求めるandrewのアルゴリズム。ソートのせいでO(NlogN)の計算量がかかる。
fn andrews_monotone_chain_convex_hull_algorithm(points: &mut Vec<Vec<isize>>, include_points_on_edge: bool) -> Option<Vec<Vec<isize>>> {
    // pointsは[[x0, y0], [x1, y1], ... , [xn, yn]] の形式である必要がある。
    // include_points_on_edgeは、凸包の返上の点(頂点と頂点の間にある点)を含めるならtrue, 含めないならfalse.基本trueで全部含めて使いたいケースが多いと思う。

    // 3点以上必要
    if points.len() < 3 {
        println!("The number of input points should be more than 3 for Andrew's algorythm.");
        return None
    }

    // x軸について昇順にソートする
    points.sort();
    // println!("points={:?}", points);
    
    // 凸包の上部の初期化。左(xが小さいもの)から1番目と、2番目の点をスタックに格納
    let mut upper = vec![points[0].clone(), points[1].clone()];

    // 凸包の下部の初期化。右(xが大きいもの)から1番目と、2番目の点をスタックに格納
    let mut lower = vec![points[points.len()-1].clone(), points[points.len()-2].clone()];

    // 凸包の上部を構築する (左から3番目から)
    for i in 2..points.len() {
        // 次に含める点
        let next_x = points[i][0];
        let next_y = points[i][1];

        // 次に含める点を追加したら、凸じゃなくなる場合、これまでに追加してきた点を新しい順に削除していく
        remove_until_convex_hull_is_restored(&mut upper, next_x, next_y, include_points_on_edge);
        upper.push(points[i].clone());
    }
    // println!("upper = {:?}", upper);

    // 凸包の下部を構築する (右から3番目から)
    for i in 2..points.len() {
        // 次に含める点
        let next_x = points[points.len()-1-i][0];
        let next_y = points[points.len()-1-i][1];

        // 次に含める点を追加したら、凸じゃなくなる場合、これまでに追加してきた点を新しい順に削除していく
        remove_until_convex_hull_is_restored(&mut lower, next_x, next_y, include_points_on_edge);
        lower.push(points[points.len()-1-i].clone());
    }
    // println!("lower = {:?}", lower);

    // 凸包の上部と凸包の下部を合併する
    for i in 1..lower.len()-1 {
        upper.push(lower[i].clone());
    }

    return Some(upper)
}

fn remove_until_convex_hull_is_restored(half_convex_hull: &mut Vec<Vec<isize>>, next_x: isize, next_y: isize, include_points_on_edge: bool) {
    // 次に含める点を追加して凸じゃなくなる場合、これまでに追加してきた点を新しい順に削除していく
    while half_convex_hull.len() >= 2  {
        // 現在までに追加した凸包上の点について、後ろから2個前の点
        let pre_x = half_convex_hull[half_convex_hull.len()-2][0];
        let pre_y = half_convex_hull[half_convex_hull.len()-2][1];
        // 現在までに追加した凸包上の点について、後ろから1個前の点
        let current_x = half_convex_hull[half_convex_hull.len()-1][0];
        let current_y = half_convex_hull[half_convex_hull.len()-1][1];
        // 後ろから2個目の点 -> 後ろから1個目の点
        let vector1 = vec![current_x - pre_x, current_y - pre_y];
        // 後ろから1個目の点-> 次に含める点
        let vector2 = vec![next_x - current_x, next_y - current_y];
        if is_clockwise(&vector1, &vector2, include_points_on_edge) {
            // 時計回り(clockwise)なら凸が崩れない
            break
        }
        else {
            // 反時計回り (counterclockwise)のときは、凸じゃなくなるので、一番最後の点を削除
            half_convex_hull.pop();
        }
    }
}

fn is_clockwise(vector1: &Vec<isize>, vector2: &Vec<isize>, include_points_on_edge: bool) -> bool {
    // vector1 -> vector2 の回転が時計回りだったらtrueを返す 
    // <=> 外積 vector1 x vector2 が正だったら反時計回り、負だったら反時計回り
    // example
    // v1 = [1,0]; v2 = [0, 1]; cp == 1     // counterclockwise
    // v1 = [1,0]; v2 = [0,-1]; cp == -1    // clockwise
    // v1 = [1,0]; v2 = [1, 0]; cp == 0     // parallel 

    let cross_product = get_cross_product(vector1, vector2);
    if cross_product < 0 {
        // 時計回り (clockwise)のとき
        return true
    }
    else if cross_product > 0 {
        // 反時計回り (counterclockwise)のときは、一番最後の凸包を削除
        return false
    }
    else {
        // cross_product == 0 のケース (v1とv2が平行なとき)
        if include_points_on_edge {
            return true
        }
        else {
            return false
        }
    }
    
}

fn get_cross_product(vector1: &Vec<isize>, vector2: &Vec<isize>) -> isize {
    // 外積 vector1 x vector2 を求める
    // example
    // v1 = [1,0]; v2 = [0, 1]; cp == 1     // counterclockwise
    // v1 = [1,0]; v2 = [0,-1]; cp == -1    // clockwise
    // v1 = [1,0]; v2 = [1, 0]; cp == 0     // parallel 

    let x1 = vector1[0];
    let y1 = vector1[1];
    let x2 = vector2[0];
    let y2 = vector2[1];

    let cross_product = x1 * y2 - y1 * x2;

    return cross_product
}

// ・凸包(Convex Hull)の定義
// 凸包は点集合 P の全ての点を含む面積最小の凸多角形のことである。
// (つまり外周の長さを最小化するような多角形で、全頂点を輪ゴムで囲った時の最終形状と考えて良さそう)
// http://i-health.u-aizu.ac.jp/CompuGeo/2017/handouts/chapter3/Chapter3H2.pdf

// 凸包上の点を求めるアルゴリズム
// ・直接法(Direct method)
// O(N^3)
// ・包装法(Wrapping method)
// O(N^2)
// ・Graham走査法(Graham scan)
// O(nlogn)

// ・逐次添加法(Incremental method)
// O(nlogn)

// ・分割統治法(Divide and conquer)
// O(nlogn)

// ・内点消去法(Inner points elimination)
// O(nlogn)

// 螺旋本はandrewのアルゴリズムでやっている。本コードもそれを利用。
// ソート時にO(nlogn)が必要
// O(nlogn)