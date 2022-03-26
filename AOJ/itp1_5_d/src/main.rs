use proconio::input;

fn main() {
    
    input! {
            a: i32,
    }
    for i in 1..=a {
        if i % 3 == 0 {
            print!(" {}", i);
        }
    } 
    println!("");
}

