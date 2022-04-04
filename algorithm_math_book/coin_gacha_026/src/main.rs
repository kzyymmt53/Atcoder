use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    let mut ans: f64 = 0.0;

    for i in 1..=a {
        ans += a as f64 /i as f64;
    }

    println!("{:.12}", ans);

}

