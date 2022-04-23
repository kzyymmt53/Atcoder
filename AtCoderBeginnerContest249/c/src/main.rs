use std::collections::HashMap;
use proconio::input;

fn main() {

    input! { 
        n: usize,
        k: i64,
        s: [String; n], 
    }

    let mut map = HashMap::new();
    let mut ans = 0;

    for i in 0..(1 << n){
        map.clear();
        for j in 0..n {
            if i & (1 << j) != 0 {
                for c in s[j].chars() {
                    *map.entry(c).or_insert(0)  += 1;
                }
            }
        }
            let mut temp = 0;
        for d in &map {
            if *d.1 as i64 == k {
              temp +=1; 
            }
        }
        if temp > ans {
                ans = temp;
            }
    }
    println!("{}" , ans);

}

