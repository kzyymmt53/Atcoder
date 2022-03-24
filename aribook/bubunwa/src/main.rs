use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    
    let mut v: Vec<i32> = Vec::new();
  
    for _ in 0..n {
        input! {
            a: i32,
        }
        v.push(a);
    }

    input! {
        k: i32,
    }
  
    if dfs(n, 0, 0, k, &v) {
        println!("Yes");
    }else{
        println!("No");
    }
}

fn dfs(n: usize, i: usize, sum: i32, k: i32, v: &Vec<i32>) -> bool {
    if i == n {
        return sum == k;
    }
  
    if dfs(n, i + 1, sum, k, &v) {
        return true;
    }

    if dfs(n, i + 1, sum + v[i], k, &v) {
        return true;
    }
  
    return false;
}
   
    

