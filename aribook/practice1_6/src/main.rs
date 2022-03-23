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
   
    let mut result = 0;
    v.sort_by_key(|&x| Reverse(x));

    for i in 0..v.len() {
       for j in i+1..v.len() {
           for k in j+1..v.len() {
               if v[i] < v[j] + v[k] && result < v[i] + v[j] + v[k] {
                   result = v[i] + v[j] + v[k];
               }
           }
       } 
    } 
    println!("{}", result);

}

