use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let m = 1000000007;
    let bunshi: i64 = ruijou(n + 1, 4, m) - 1;
    let bunbo: i64 = 4 - 1;

    let b: i64 = ruijou(m - 2, bunbo , m) % m;
    let ans = (b * bunshi) % m;

    println!("{}", ans);



}

fn ruijou(n: i64, mut a: i64, m: i64) -> i64 {
    let mut ans: i64 = 1;
    for i in 0..60 {
        if n & (1 << i) != 0 {
            ans *= a; 
            ans %= m;
        }
        a *= a;
        a %= m;
    }

    return ans % m;
}

