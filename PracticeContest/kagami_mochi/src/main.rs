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

    let mut result = 0;
    let mut pre = -1;

    for data in v {
        if pre == -1 {
            pre = data;
            result += 1;
        }else{
            if pre > data {
                pre = data;
                result += 1;
            }
        }
    }

    println!("{}", result);
}

