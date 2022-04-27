use proconio::input;

fn main() {

    input! { 
        a: u64,
        b: u64,
    }

    let ans: u64;
    if a >= b {
        ans = lcm(a, b);
    }else{
        ans = lcm(b, a);
    }

    if ans > 1000000000000000000 {
        println!("Large");
    }else{
        println!("{}", ans);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if a % b == 0 {
        return b;
    }

    return gcd(b, a % b);
}

fn lcm(a: u64 , b: u64) -> u64 {
    if b / gcd(a, b) > 1000000000000000000 / a {
        return 1000000000000000001;
    }
    return b / gcd(a, b) * a;
}
