use proconio::input;
use std::cmp;

fn main() {

    input! { 
        n: i64,
        k: i64,
    }

    let mut ans: i64 = 0;
    for x in 2..=2*n {
        if 2 <= x - k && x - k <= 2 * n {
            let cd = x - k;
            ans += cmp::min(x - 1, 2 * n + 1 - x) * cmp::min(cd - 1, 2 * n + 1 - cd); 
        }
    }
    println!("{}", ans);

}

