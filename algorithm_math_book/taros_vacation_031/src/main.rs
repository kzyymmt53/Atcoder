use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! {
        a: [i64; n],
    }

    let mut dp: Vec<i64> = vec![0; n];
    dp[0] = a[0];
    dp[1] = a[1];
    for i in 2..n {
        if dp[i - 2] + a[i] > dp[i - 1] {
            dp[i] = dp[i - 2] + a[i];
        }else{
            dp[i] = dp[i - 1];
        }
    }

    println!("{:?}", dp[n - 1]);

}

