use proconio::input;

fn main() {

    input! { 
        n: usize,
        mut v: [(i64, i64); n],
    }

    v.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());
    let mut ans: i64 = 0;
    for i in 0..n {
        ans += v[i].1 * ((-1 * n as i64 + 1) + 2 * i as i64);
    }
    v.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());
    for i in 0..n {
        ans += v[i].0 * ((-1 * n as i64 + 1) + 2 * i as i64);
    }

    println!("{}", ans);

}

