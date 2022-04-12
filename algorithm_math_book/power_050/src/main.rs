use proconio::input;

fn main() {

    input! { 
        mut a: i64,
        b: u32,
    }

    let mut ans = 1;

    for i in 0..30 {
        if (b & (1 << i)) != 0 {
            ans *= a;
            ans %= 1000000007;
        }
        a *= a;
        a %= 1000000007;
    }



    println!("{}", ans % 1000000007);


}

