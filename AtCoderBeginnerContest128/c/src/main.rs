use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut v = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            s: [Usize1; k],
        }
        v.push(s);
    }

    input! {
        p: [usize; m],
    }

    let mut ans = 0;
    for i in 0..(1 << n) {
        let mut result = true;
        for j in 0..m {
            let mut cnt = 0;
            for d in v[j].iter() {
                if i & (1 << d) != 0 {
                    cnt += 1;
                }
            }
            if cnt % 2 != p[j] {
                result = false;
            }
        }
        if result {
            ans += 1;
        }
    }
    println!("{}", ans);
}
