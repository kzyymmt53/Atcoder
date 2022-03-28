use proconio::input;

fn main() {

    input! { 
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    }
    
    let result = ((x2 - x1).abs().powf(2.0) + (y2 - y1).abs().powf(2.0)).powf(0.5);
    
    println!("{:.8}", result);

}

