use proconio::input;

fn main() {
    input! {
        a: i64, 
    }
    
    let s1 = a / 100;
    let s2 = (a - s1 * 100) / 10;
    let s3 = a % 10;
    println!("{}", s1 + s2 + s3);
}
