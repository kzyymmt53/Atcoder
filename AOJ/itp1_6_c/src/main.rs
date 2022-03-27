use proconio::input;

fn main() {
    
    input! {
        n: usize,
    }
   
    let mut v: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 10]; 3]; 4];

    for _ in 0..n {
        input! {
            b: usize,
            f: usize,
            r: usize,
            v2: i32,
        }
        v[b - 1][f - 1][r - 1] += v2;
    }
    
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            for k in 0..v[i][j].len() {
               print!(" ");
               print!("{}", v[i][j][k]); 
            }
            println!("");
        }
        println!("####################");
    }
}

