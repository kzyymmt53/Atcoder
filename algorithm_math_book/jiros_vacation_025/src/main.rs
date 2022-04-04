use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! {
        a: [f64; n],
    }

    input! {
        b: [f64; n],
    }

    let mut ans: f64 = 0.0;

    for i in 0..n {
        ans += a[i] / 3.0 + b[i] * 2.0 / 3.0;
    }

    println!("{:.12}", ans);

}

