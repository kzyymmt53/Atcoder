use proconio::input;

fn main() {

    input! { 
        n: usize,
        w: usize,
    }

    let mut a = vec![];
    a.push((0, 0));
    for _ in 0..n {
        input! {
            b: (i64, i64),
        }
        a.push(b);
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n{
        for j in 1..=w {
            if a[i].0 > j as i64 {
                dp[i][j] = dp[i - 1][j];
            }else{
                if j >= a[i].0 as usize && dp[i - 1][j] < dp[i - 1][j - a[i].0 as usize] + a[i].1 {
                    dp[i][j] = dp[i - 1][j - a[i].0 as usize] + a[i].1;
                }else{
                    dp[i][j] = dp[i - 1][j]; 
                }
            }

        }
    }

    let mut ans = 0;
    for i in dp {
        for j in i {
            if ans < j {
                ans = j;
            }
        }
    }
    println!("{}", ans);


}

