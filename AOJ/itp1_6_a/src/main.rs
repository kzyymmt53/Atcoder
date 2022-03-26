use proconio::input;

fn main() {
    
    input! {
        n: usize,
    }
   
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..n {
        input! {
            a: i32,
        }
        v.push(a);
    }
    
    for i in (0..n).rev() {
        print!("{} ", v[i]);
    }
    println!("");
}

