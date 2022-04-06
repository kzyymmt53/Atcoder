use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! {
        a: [(f64, f64); n],
    }

    let mut ans: f64 = 1000000000000.0;
    for i in 0..n {
        for j in i + 1..n{
            let temp: f64 = ((a[i].0 - a[j].0).powf(2.0) + (a[i].1 - a[j].1).powf(2.0)).powf(0.5);
            if ans > temp {
                ans = temp;
            }
        }
    }

    println!("{:.28}", ans);
}


