use proconio::input;

fn main() {

    let mut v: Vec<i32> = Vec::new();
    input! {
        n: usize,
    }

    for _ in 0..n {
        input! {
            a: i32,
        }
        v.push(a);
    }
    println!("{} {} {}", v.iter().min().unwrap(), v.iter().max().unwrap(), v.iter().fold(0, |sum, a| sum + a)); 



}

