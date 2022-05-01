use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    let mut v: Vec<Vec<i64>> = vec![Vec::new(); n + 1];
    let mut isvisit: Vec<bool> = vec![false; n + 1];
    for _ in 0..n - 1 {
        input! {
            a: i64,
            b: i64,
        }

        v[a as usize].push(b);
        v[b as usize].push(a);
    }

    let mut dp: Vec<i64> = vec![0; n + 1];
    dfs(&mut dp,  &v, &mut isvisit, 1);
    let mut ans: i64 = 0;

    for i in 2..=n {
        ans += dp[i] * (n as i64 - dp[i]);
    }

    println!("{}", ans);

}

fn dfs(dp: &mut Vec<i64>, v: &Vec<Vec<i64>>, isvisit: &mut Vec<bool>, target: i64) {

    isvisit[target as usize] = true;
    dp[target as usize] = 1;

    for d in &v[target as usize] {
        if !isvisit[*d as usize] {
            dfs(dp, v, isvisit, *d);
            dp[target as usize] += dp[*d as usize];
        }
    }
}

