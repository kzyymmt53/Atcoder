use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    
    let mut count: i32 = 0;
    for i in a..=b {
        if c % i == 0 {
           count += 1; 
        }
    } 
    println!("{}", count);
    
}

