use proconio::input;

fn main() {

    input! { 
        n: usize,
        mut t: [i64; n],
    }

    let mut sum: i64 = 0;
    for td in &t {
        sum += td;
    }

    let mut dp:Vec<Vec<bool>> = vec![vec![false; sum as usize + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=sum {
            if j < t[i as usize - 1] {
                if dp[i as usize - 1][j as usize] {
                    dp[i as usize][j as usize] = true;
                }else{
                    dp[i as usize][j as usize] = false;
                }
            }
            if j as i64 >= t[i as usize - 1] {
                if dp[i as usize - 1][j as usize] || dp[i as usize - 1][j as usize - t[i as usize - 1] as usize] {
                    dp[i as usize][j as usize] = true;
                }else{
                    dp[i as usize][j as usize] = false;
                }
            }
        }
    }


    let mut ans = 9223372036854775807;
    for i in 0..=sum {
        if dp[n as usize][i as usize] {
            let time;
            if i > sum - i {
                time = i;
            }else{
                time = sum - i;
            }

            if ans > time {
                ans = time;
            }
        }
    }

    println!("{}", ans);
}

