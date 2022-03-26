use proconio::input;

fn main() {
    input! {
        m: i32,
        l: i32,
    }
    
    let length = m * 2 + l * 2; 
    let area = m * l;
    println!("{} {}",  area, length);
}
