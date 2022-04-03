use proconio::input;

fn main() {

    input! { 
        n: usize,
        s: usize,
    }

    input!{
        v: [i32; n],
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        for j in 0..s+1 {
            if dp[i][j] {
                dp[i + 1][j] = true;
            }
            if v[i] <= j as i32 && dp[i][j - (v[i] as usize)] {
                dp[i + 1][j] = true;
            }
        }
    }
    if dp[n][s] {
        println!("Yes");
    }else{
        println!("No");
    }
}

