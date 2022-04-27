use proconio::input;
use std::collections::HashSet;

fn main() {

    input! { 
        n: i64,
        b: i64,
    }

    let mut targets: HashSet<i64> = HashSet::new();

    make_targets(&mut targets, 0, 0);

    let mut ans: i64 = 0;
    for target in targets {
        let m: i64 = b + target;
        let f_m: i64 = product(m);
        if m == f_m + b && m <= n && 1 <= m {
            ans += 1;
        }
    }

    println!("{}", ans);


}

fn make_targets(targets: &mut HashSet<i64>, i: i64, j: i64) {
    if i == 11 {
        targets.insert(product(j));
        return;
    }

    let min_value: i64 = j % 10;
    for k in min_value..10 {
        make_targets(targets, i + 1, 10 * j + k);
    }
}

fn product(m: i64) -> i64 {
    if m == 0 {
        return 0;
    }

    let mut temp: i64 = m;
    let mut ans: i64 = 1;
    loop {
        ans *= temp % 10;
        temp /= 10;

        if temp == 0 {
            return ans;
        }
    }
}
