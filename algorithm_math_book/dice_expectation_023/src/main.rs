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
        ans += a[i] / n as f64 + b[i] / n as f64;
    }

    println!("{:.12}", ans);

}

