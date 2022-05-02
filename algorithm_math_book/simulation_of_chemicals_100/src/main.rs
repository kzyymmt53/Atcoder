use proconio::input;

fn main() {

    input! { 
        q: usize,
        v: [(f64, f64, f64, i64); q],
    }

    
    for d in v {
        let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; 3]; 3]; 
        matrix[0][0] = 1.0 - d.0;
        matrix[0][1] = d.1;
        matrix[1][1] = 1.0 - d.1;
        matrix[1][2] = d.2;
        matrix[2][0] = d.0;
        matrix[2][2] = 1.0 - d.2;

        let result = power_matrix(d.3, &mut matrix);
        let a: f64 = 1.0 * result[0][0] + 1.0 * result[0][1] + 1.0 * result[0][2];
        let b: f64 = 1.0 * result[1][0] + 1.0 * result[1][1] + 1.0 * result[1][2];
        let c: f64 = 1.0 * result[2][0] + 1.0 * result[2][1] + 1.0 * result[2][2];

        println!("{:.15} {:.15} {:.15}", a, b, c);

    }

}

fn mulmatrix(a: &mut Vec<Vec<f64>>, b: &mut Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let mut c: Vec<Vec<f64>> = vec![vec![0.0; 3]; 3];

    for i in 0..3 {
        for k in 0..3 {
            for j in 0..3 {
                c[i][j] += a[i][k] * b[k][j]; 
            }
        }
    }

    return c;
}


fn power_matrix(n: i64, v: &mut Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let mut ans: Vec<Vec<f64>> = vec![vec![0.0; 3]; 3];
    let mut flag = false;
    let mut temp;
    for i in 0..60 { 
        if n & (1 << i) != 0 {
            if !flag {
                ans = v.to_vec();
                flag = true;
            }else{
                ans = mulmatrix(&mut ans, v);
            }
        }
        temp = v.clone();
        *v = mulmatrix(&mut temp, v);
    }

    return ans;
}



