use proconio::input;
use std::collections::HashMap;

fn main() {
    
    input! {
        n: usize,
    }
    let mut hash = HashMap::new(); 
    for _ in 0..n {
        input! {
            s: String,
            a: String,
        }
        hash.insert(s + "_" + &a, 1);
    }
    check("S", &hash);
    check("H", &hash);
    check("C", &hash);
    check("D", &hash);

}

fn check(s: &str, hash: &HashMap<String, usize>) {
    for i in 1..=13 {
       let key: String = String::from(s) + "_"  + &i.to_string();
       if !hash.contains_key(&key) {
           println!("{} {}",s , i);
       }
    }
}

