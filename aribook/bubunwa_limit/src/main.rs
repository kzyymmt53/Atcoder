use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut avec: Vec<i32> = Vec::new();
    let mut mvec: Vec<i32> = Vec::new();

    for _ in 0..n {
        input! {
            a: i32,
            m: i32,
        }
        avec.push(a);
        mvec.push(m);
    }

    input! {
        total: i32,
    }


    let index = total + 1; 
    let mut dp: Vec<i32> = vec![-1; index as usize];

    dp[0] = 0;
    for i in 0..n {
        for j in 0..=total {
          if dp[j as usize] >= 0{
              dp[j as usize] = mvec[i]; 
          }else if j < avec[i] || dp[(j - avec[i]) as usize] <= 0 {
              dp[j as usize] = -1;
          }else{
              dp[j as usize] = dp[(j - avec[i]) as usize] - 1;
          }
        }
    }

    println!("{}", dp[total as usize]);

}

