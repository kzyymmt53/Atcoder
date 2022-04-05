use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! { 
        h: [i64; n ],
    }

    let mut dp: Vec<i64> = vec![1000000; n];

    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs(); 
    for i in 2..n {
        if dp[i - 1] + (h[i] - h[i - 1]).abs() > dp[i - 2] + (h[i - 2] - h[i]).abs() {
                dp[i] = dp[i - 2] + (h[i - 2] - h[i]).abs();
        }else{
                dp[i] = dp[i - 1] + (h[i] - h[i - 1]).abs();
        }
    }

    println!("{}", dp[n - 1]);
}

