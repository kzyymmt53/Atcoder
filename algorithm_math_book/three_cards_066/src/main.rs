use proconio::input;
use std::cmp;

fn main() {

    input! { 
        n: i64,
        k: i64,
    }

    let mut yojisho = 0;

    for a in 1..=n {
        for b in cmp::max(1, a - (k - 1))..=cmp::min(n, a + (k - 1)) {
            for c in cmp::max(1, a - (k - 1))..=cmp::min(n, a + (k - 1)) {
                if (b - c).abs() < k {
                    yojisho += 1;
                }
            }
        }
    }

    println!("{}", n * n * n - yojisho);

}

