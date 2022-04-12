use proconio::input;

fn main() {

    input! { 
        x: i64,
        y: i64,
    }

    let mut a = 1;
    for i in 1..=(x + y) {
        a *= i;
        a %= 1000000007;
    }

    let mut b1 = 1;
    for i in 1..=x {
        b1 *= i;
        b1 %= 1000000007;

    }

    let mut b2 = 1;
    for i in 1..=y {
        b2 *= i; 
        b2 %= 1000000007;
    }

    let mut b = b1 * b2 % 1000000007;

    let mut ans = 1;
    for i in 0..30 {
        if  1000000005 & (1 << i) != 0 {
            ans *= b; 
            ans %= 1000000007;
        }
        b *= b;
        b %= 1000000007;
    }

    println!("{}", a * ans % 1000000007);

}

