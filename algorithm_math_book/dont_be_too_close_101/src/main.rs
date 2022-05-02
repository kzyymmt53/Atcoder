use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let mut v: Vec<i64> = vec![1; n as usize + 1];
    let m: i64 = 1000000007;
    

    for i in 1..n + 1 {
        v[i as usize] = i * v[i as usize - 1] % m;
    }


    for i in 1..=n {
        let mut ans: i64 = 0;
        for j in 1..=(n + i - 1) / i {
            ans += ncr(n - (i - 1) * (j - 1), j, &v, m);
        }
        println!("{}", ans % m);
    }
}

fn ncr(n: i64, r: i64, fact: &Vec<i64>, m: i64) -> i64 {
    return division(fact[n as usize], fact[(n - r) as usize] * fact[r as usize] % m, m); 
}

fn division(bunshi: i64, bunbo: i64, m: i64) -> i64 {
    return bunshi * modpow(bunbo, m - 2, m) % m;
}

fn modpow(bunbo: i64, p: i64, m: i64) -> i64 {
    let mut a = bunbo;

    let mut ans = 1;
    for i in 0..60 {
        if p & (1 << i) != 0 {
            ans *= a;
            ans %= m;
        }
        a *= a;
        a %= m;
    }

    return ans;
}

