use proconio::input;

fn main() {

    input! { 
        a: i64,
        b: i64,
    }

    let mut ans: i64 = 0;
    for i in 1..b {
        if check(a, b, i) {
            if ans < i {
                ans = i;
            }
        }

    }

    println!("{}", ans);

}

fn check(a: i64, b: i64, t: i64) -> bool {
    if b / t  - (a + t - 1) / t >= 1 {
        return true;
    }

    return false;
}

