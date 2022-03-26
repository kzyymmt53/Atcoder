use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    
    println!("{}", a / b);
    println!("{}", a % b);
    println!("{:.5}", (a as f32) / (b as f32));
    
}

