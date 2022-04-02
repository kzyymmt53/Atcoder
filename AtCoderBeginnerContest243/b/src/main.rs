use proconio::input;
use std::collections::HashMap;


fn main() {

    let mut map = HashMap::new();
    let mut map2 = HashMap::new();
    input! { 
        n: usize,
    }

    for i in 0..n {
        input! {
            a: String,
        }
        map.insert(a, i);
    }

    for i in 0..n {
        input! {
            b: String,
        }
        map2.insert(b, i);
    }

    let mut result = 0;
    let mut result2 = 0;

    for d in map {
        if map2.contains_key(&d.0) {
            if map2.get(&d.0).unwrap() == &d.1 {
                result += 1;
            }else{
                result2 += 1
            }
        }
    }

    println!("{}", result);
    println!("{}", result2);

}

