use proconio::input;
use std::f64::consts::PI;

fn main() {

    input! { 
        a: f64,
        b: f64,
        h: f64,
        f: f64,
    }

    let x1: f64 = a * (2.0 * PI * h / 12.0 + PI / 6.0 / 60.0 * f).cos();
    let y1: f64 = a * (2.0 * PI * h / 12.0 + PI / 6.0 / 60.0 * f).sin();


    let x2: f64 = b * (2.0 * PI * f / 60.0).cos();
    let y2: f64 = b * (2.0 * PI * f / 60.0).sin();


    let ans: f64 = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).powf(0.5);

    println!("{:.20}", ans);


}

