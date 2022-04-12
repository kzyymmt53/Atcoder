use proconio::input;

fn main() {

    input! { 
        x: i64,
        y: i64,
    }

    if 2 * x - y < 0 || 2 * y - x < 0 {
        println!("0");
        return;
    }

    if (2 * x - y) % 3 != 0 || (2 * y - x) % 3 != 0 {
        println!("0");
        return;
    }

    let a = (2 * x - y) / 3;
    let b = (2 * y - x) / 3;

    let mut bunshi = 1;
    for i in 1..=(a + b) {
        bunshi *= i;
        bunshi %= 1000000007;
    }

    let mut bunbo = 1;
    for i in 1..=a {
        bunbo *= i;
        bunbo %= 1000000007;
    }

    for i in 1..=b {
        bunbo *= i;
        bunbo %= 1000000007;
    }

    let mut ans = 1;
    for i in 0..30 {
        if 1000000005 & (1 << i) != 0 {
            ans *= bunbo;
            ans %= 1000000007;
        }
        bunbo *= bunbo;
        bunbo %= 1000000007;
    }

    println!("{}", bunshi * ans % 1000000007);


}



