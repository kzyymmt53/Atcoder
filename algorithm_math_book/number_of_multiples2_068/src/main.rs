use proconio::input;

fn main() {

    input! { 
        n: i64,
        k: i64,
        v: [i64; k],
    }
    
    let mut ans: i64 = 0;
    for i in 1..(1 << k) {
        let mut cnt: i64 = 0;
        let mut lcm2: i64 = 1;
        for j in 0..k {
            if i & (1 << j) != 0 {
                cnt += 1;
                lcm2 = lcm(lcm2, v[j as usize]); 
            }
        }
        let num: i64 = n / lcm2;
        if cnt % 2 == 1 {
            ans += num;
        }else{
            ans -= num;
        }
    }

    println!("{}", ans);

}

fn lcm(a: i64, b: i64) -> i64 {
    return a / gcd(a, b) * b;
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

