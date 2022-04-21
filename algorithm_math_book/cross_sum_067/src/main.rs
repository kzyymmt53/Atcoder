use proconio::input;

fn main() {

    input! { 
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    }

    
    let mut rows: Vec<i64> = vec![0; h];
    let mut cols: Vec<i64> = vec![0; w];

    for i in 0..w {
        for j in 0..h {
            rows[j] += a[j][i];
            cols[i] += a[j][i];
        }
    }


    for j in 0..h {
        for i in 0..w {
            if i == 0 {
                print!("{}", rows[j] + cols[i] - a[j][i]); 
            }else{
                print!(" {}", rows[j] + cols[i] - a[j][i]); 
            }
        }
        println!("");
    }
}

