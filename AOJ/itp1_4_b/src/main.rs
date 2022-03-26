use proconio::input;

fn main() {
    let pi = std::f32::consts::PI;
    input! {
        r: f32,
    }
     
    println!("{:.6} {:.6}", r * r * pi, 2.0 * pi * r);
    
}

