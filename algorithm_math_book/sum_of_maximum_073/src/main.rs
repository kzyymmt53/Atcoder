use proconio::input;

fn main() {

    input! { 
        n: usize,
        a: [i64; n],
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        let temp = a[i] * original_pow(i as i64) % 1000000007;
        ans += temp % 1000000007;
    }

    println!("{}", ans % 1000000007);

}


fn original_pow(i: i64) -> i64 {
    let mut ans: i64 = 1;
    let mut rate: i64 = 2;
    for j in 0..60 {
        if i & (1 << j) != 0 {
            ans *= rate;
            ans %= 1000000007;
        }

        rate *= rate;
        rate %= 1000000007;
    }

    return ans;
}
