use std::collections::HashMap;
use proconio::input;

fn main() {

    input! { 
        n: i64,
        s: [(String, i64); n],
    }

    let mut map = HashMap::new();


    let mut max: i64 = 0;
    let mut ans: i64 = 0;
    let mut count = 0;
    for d in s {
        count += 1;
        if map.contains_key(&d.0) {
            continue;
        }
        if max < d.1 {
            max = d.1;
            ans = count; 
        }
        map.insert(d.0, d.1);
        
    }

    println!("{}", ans);

}

