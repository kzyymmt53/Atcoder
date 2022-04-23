use proconio::input;

fn main() {

    input! { 
        n: usize,
        a: [i64; n],
    }

    let mut ans: i64 = 0;
    let mut fact: Vec<i64> = vec![0; n];
    fact[0] = 1;
    for i in 1..n {
        fact[i] = fact[i - 1] * i as i64 % 1000000007;
    }
   

    for i in 0..n {
        ans += (combi(fact[n - 1], fact[n - 1 - i] * fact[i], 1000000007) * a[i]) % 1000000007;
    }
    
    println!("{}", ans % 1000000007);

}

fn combi(bunshi: i64, bunbo: i64, m: i64) -> i64 {

    return bunshi * modpow(bunbo % m, m - 2, m) % m;

}

fn modpow(mut bunbo: i64, p: i64, m: i64) -> i64 {
    let mut ans = 1;

    for i in 0..60 {
        if p & (1 << i) != 0 {
            ans *= bunbo;
            ans %= m;
        }
        bunbo *= bunbo;
        bunbo %= m;
    }

    return ans;
}
