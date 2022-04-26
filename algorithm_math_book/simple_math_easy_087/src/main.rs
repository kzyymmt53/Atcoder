use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let mut ans = n * (n + 1) / 2;
    ans %= 1000000007;

    println!("{}", ans * ans % 1000000007);

}

