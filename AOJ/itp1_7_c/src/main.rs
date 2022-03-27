use proconio::input;

fn main() {
    
    let mut cols: Vec<i32> = Vec::new();
    input! {
        r: usize,
        c: usize,
    }
    let mut rows: Vec<i32> = vec![0; c];
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; c]; r]; 
    let mut sum = 0;
    
    for i in 0..r {
        let mut col = 0;
        for j in 0..c {
           input! {
               a: i32,
           }
           matrix[i][j] = a; 
           col += a;
           rows[j] += a; 
        } 
        cols.push(col);
        sum += col;
    }
   
    for i in 0..r {
        for j in 0..c {
            print!("{} ", matrix[i][j]);
        }
        println!("{}", cols[i]); 
    }
    for i in 0..c {
        print!("{}", rows[i]);
        if i != c - 1 {
            print!(" ");
        }else{
            println!(" {}", sum);
        }
    }
}

