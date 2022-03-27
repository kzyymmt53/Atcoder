use proconio::input;

fn main() {

    input! {
        n: usize,
        m: usize,
        l: usize,
    }
    let mut a: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut b: Vec<Vec<i32>> = vec![vec![0; l]; m];
    let mut c: Vec<Vec<i32>> = vec![vec![0; l]; n];

    for i in 0..n {
        for j in 0..m {
           input! {
               d: i32,
           }
           a[i][j] = d;
        }
    }
 
    for i in 0..m {
        for j in 0..l {
           input! {
               d: i32,
           }
           b[i][j] = d;
        }
    }

    for i in 0..n {
        for j in 0..l {
            let mut sum = 0;
            for k in 0..m {
                sum += a[i][k] * b[k][j]; 
            }
            c[i][j] = sum;
        }
    }
   
    for i in 0..n {
        for j in 0..l {
            print!("{} ", c[i][j]);
        }
        println!("");
    }


}

