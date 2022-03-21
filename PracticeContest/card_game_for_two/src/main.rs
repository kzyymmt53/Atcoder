use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
         n: i32,
    }
     
    let mut v: Vec<i32> = Vec::new();

    for _ in 0..n {
        input! {
          a: i32,
        }
        v.push(a);
    }
    v.sort_by_key(|&x| Reverse(x));
    
    let mut alice = 0;
    let mut bob = 0;

    for (i, data) in v.iter().enumerate() {
        if i % 2 == 0 {
            alice += data;
        }else{
            bob += data; 
        }
    }
    
    println!("{}", alice - bob);
}
