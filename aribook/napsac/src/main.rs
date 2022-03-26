use proconio::input;

fn main() {
    input! {
        n: usize,
    }
   
    let mut wvec: Vec<i32> = Vec::new();
    let mut vvec: Vec<i32> = Vec::new();

    for _ in 0..n {
        input! {
            w: i32, 
            v: i32,
        }
        wvec.push(w);
        vvec.push(v);
    }

    input! {
        total: i32,
    }
     

    let index = n + 1;
    let index2 = total + 1;
    let mut memo: Vec<Vec<i32>> = vec![vec![-1; index2 as usize]; index];


    let res = rec(n, 0, &wvec, &vvec, total, &mut memo);
    println!("{}", res);

}

fn rec(n: usize, i: usize, wvec: &Vec<i32>, vvec: &Vec<i32>, j: i32, memo: &mut Vec<Vec<i32>>) -> i32{
    let res;
    
    if n == i {
       res = 0;
    }else if memo[i][j as usize] >= 0 {
       res = memo[i][j as usize];
    }else if j < wvec[i] {
       res = rec(n, i + 1, &wvec, &vvec, j, memo);
    }else {
       res = std::cmp::max(rec(n, i + 1, &wvec, &vvec, j, memo), rec(n, i + 1, &wvec, &vvec, j - wvec[i], memo) + vvec[i]); 
    }
    memo[i][j as usize] = res;
    return res; 
}

