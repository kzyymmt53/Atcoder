use proconio::input;

fn main() {
    
    input! {
        n: usize,
        m: usize,
    }
   
    let mut a: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut b: Vec<i32> = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            input! {
                x: i32,
            }
            a[i][j] = x;
        } 
    }
  
    for i in 0..m {
        input! { 
            x: i32,
        }
        b[i] = x;
    }
    
    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            sum += a[i][j] * b[j];
        }
        println!("{}", sum);
    }
}

