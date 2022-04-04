use std::collections::HashMap;
use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! {
        a: [i64; n],
    }

    let mut map: HashMap<i64, i64> = HashMap::new();

    for d in a {
        *map.entry(d).or_insert(0)  += 1;
    }

    let mut ans = 0;
    for d in &map {
        if d.0 > &(100000 / 2) {
            continue;
        }
        let b = 100000 - d.0;
        if b == *d.0 {
            ans += (d.1 - 1) * d.1 / 2;
        }else{
            if map.contains_key(&b) {
                ans += d.1 * map.get(&b).unwrap();
            }
        }
    }
    println!("{}", ans);

}

