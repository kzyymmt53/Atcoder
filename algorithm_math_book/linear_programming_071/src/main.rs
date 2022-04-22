use proconio::input;

fn main() {

    input! { 
        n: usize,
        d: [(f64, f64, f64); n],
    }

    let mut ans: f64 = 0.0;
    for i in 0..n {
        for j in i+1..n {
            if d[i].0 * d[j].1 == d[j].0 * d[i].1 {
                continue;
            }
            let dx = (d[j].1 * d[i].2 - d[i].1 * d[j].2) / (d[i].0 * d[j].1 - d[j].0 * d[i].1 );
            let dy = (d[j].0 * d[i].2 - d[i].0 * d[j].2) / (d[j].0 * d[i].1 - d[i].0 * d[j].1 );
            if check(dx - 0.0000001, dy - 0.0000001, &d, n) {
                if ans < dx - 0.0000001 + dy - 0.0000001 {
                    ans = dx - 0.0000001 + dy - 0.0000001;
                }
            }

        }
    }

    println!("{:.6}", ans);

}

fn check(dx: f64, dy: f64, d: &Vec<(f64, f64, f64)>, n: usize) -> bool {
    let mut flag = true;
    for i in 0..n {
        if d[i].0 * dx + d[i].1 * dy > d[i].2 {
            flag = false;
            break;
        }
    }

    return flag;
}

