use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let mut v: Vec<Vec<i64>> = vec![vec![0; 2]; 2];
    v[0][0] = 1;
    v[0][1] = 1;
    v[1][0] = 1;

    let ans = power_matrix(n, &mut v);
    println!("{}", ans[1][0]);

}

fn mulmatrix(a: &mut Vec<Vec<i64>>, b: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut c: Vec<Vec<i64>> = vec![vec![0; 2]; 2];

    for i in 0..2 {
        for k in 0..2 {
            for j in 0..2 {
                c[i][j] += a[i][k] * b[k][j];
                c[i][j] %= 1000000000;
            }
        }
    }

    return c;
}

fn power_matrix(n: i64, v: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut ans: Vec<Vec<i64>> = vec![vec![0; 2]; 2];
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


