use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
    }

    let mut v: Vec<i32> = Vec::new();
    for _ in 0..n {
        input! {
            a: i32,
        }
        v.push(a);
    } 
  
    let mut v2: Vec<i32> = Vec::new();
    for _ in 0..n {
        input! {
            b: i32,
        }
        v2.push(b);
    }

    
    let mut memo: Vec<Vec<i32>> = vec![vec![-1; n]; 2];

    if rec(0, k, v[0], &v, &v2, n, &mut memo, 0) || rec(0, k, v2[0], &v, &v2, n, &mut memo, 1) {
        println!("Yes");
    }else{
        println!("No");
    }
}

fn rec(i: usize, k: i32, target: i32, v: &Vec<i32>, v2: &Vec<i32>, n: usize, memo: &mut Vec<Vec<i32>>, flag: usize) -> bool {
    if i == n - 1 {
        return true;
    }
    if memo[flag][i] != -1 {
        if memo[flag][i] == 0 { 
            return false;
        }else{
            return true;
        }
    }
    if (target - v[i + 1]).abs() <= k {
        if rec(i + 1, k, v[i + 1], &v, &v2, n , memo, 0) {
            memo[flag][i] = 1; 
            return true;
        }
    }
  
    if (target - v2[i + 1]).abs() <= k {
        if rec(i + 1, k, v2[i + 1], &v, &v2, n, memo, 1) {
            memo[flag][i] = 1; 
            return true;
        }
    } 
    memo[flag][i] = 0; 
    return false;
}
