use proconio::input;
use std::f32::consts::PI;

fn main() {

    input! { 
        a: f32,
        b: f32,
        c: f32,
    }
  
    let radian = c / 180.0 * PI;

    let s = 0.5 * a * b * f32::sin(radian);
    let h =  b * f32::sin(radian);
    let d =  (a.powf(2.0) + b.powf(2.0) - 2.0 * a * b * f32::cos(radian)).powf(0.5);
    let l = a + b + d;

    println!("{} {} {}", s, l, h);

}

