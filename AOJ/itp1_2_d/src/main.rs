use proconio::input;

fn main() {
    input! {
        w: i32,
        h: i32,
        x: i32,
        y: i32,
        r: i32,
    }
    
    if 0 <= x - r && 0 <= y - r && w >= x + r && h >= y + r {
        println!("Yes");
    }else {
        println!("No");
    }
}

