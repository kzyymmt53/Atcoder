use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    input! {
        v: [[f64; 2]; n],
    }

    let mut ans: f64 = 0.0;

    for d in v {
        ans += d[1] / d[0];
    }

    println!("{:.12}", ans);

}

