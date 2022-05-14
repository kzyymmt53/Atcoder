use std::collections::HashMap;
use proconio::input;
use itertools::Itertools;

fn main() {

    input! { 
        n: i64,
        w: i64,
        a: [i64; n],
    }

    let mut map: HashMap<i64, i64> = HashMap::new();

    let mut ans: i64 = 0;
    for j in 1..=3 {
    for comb in (0..n).combinations(j) { 
        let mut temp: i64 = 0;
        for i in comb {
            temp += a[i as usize];
        }
        if temp <= w {
            if map.contains_key(&temp) {
                continue;
            }
            ans += 1;
            map.insert(temp, 1);
        }
    }
    }
    println!("{}", ans);

}

