use proconio::input;

fn main() {

    input! { 
        n: usize,
        mut a: [i64; n],
    }

    a.sort();
    let mut ans: i64 = 0;
    for i in 0..n {
        ans += a[i] * ((-1 * n as i64 + 1) + 2 * i as i64);
    }

    println!("{}", ans);

}

