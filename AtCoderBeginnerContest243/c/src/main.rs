use proconio::input;
use std::collections::HashMap;


fn main() {

    input! { 
        n: usize,
    }

    let mut r = HashMap::new();
    let mut l = HashMap::new();

    input! {
        v: [(i32, i32); n],
    }

    input! {
        s: String,
    }


    
    for (i, c) in s.chars().enumerate() {
        if c == 'R' {
            if r.contains_key(&v[i].1) {
                if (*r.get(&v[i].1).unwrap() as i32) > v[i].0 {
                    *r.entry(&v[i].1).or_insert(0) = v[i].0;
                }
            }else{
                r.insert(&v[i].1, v[i].0);
            }
        }else{
            if l.contains_key(&v[i].1) {
                if (*l.get(&v[i].1).unwrap() as i32) < v[i].0 {
                    *l.entry(&v[i].1).or_insert(0) = v[i].0;
                }
            }else{
                l.insert(&v[i].1, v[i].0);
            }
        }

    }

    let mut ans = false;

    for d in r {
        if l.contains_key(&d.0) && d.1 <= (*l.get(&d.0).unwrap() as i32) {
            ans = true;
            break;
        }
    }


    if ans {
        println!("Yes");
    }else{
        println!("No");
    }

}

