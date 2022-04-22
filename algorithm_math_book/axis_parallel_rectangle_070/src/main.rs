use proconio::input;

fn main() {

    input! { 
        n: usize,
        k2: usize,
        v: [(i64, i64); n],
    }



    let mut ans: i64 = 9223372036854775807;

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    let lx: i64 = v[i].0;
                    let rx: i64 = v[j].0;
                    let ty: i64 = v[k].1;
                    let dy: i64 = v[l].1;
                    if check(k2, &v, lx, rx, ty, dy) {
                        if ans > (rx - lx) * (ty - dy) {
                            ans = (rx - lx) * (ty - dy);
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);



}

fn check(k: usize, v: &Vec<(i64, i64)>, lx: i64, rx: i64, ty: i64, dy: i64) -> bool {
    let mut cnt = 0; 
    for d in v {
        if d.0 <= rx && d.0 >= lx && d.1 <= ty && d.1 >= dy {
            cnt += 1;
        }
    }

    return cnt >= k;
}
