use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let mut ans: Vec<Vec<i64>> = vec![vec![0; 3]; 3];
    ans[0][0] = 1;
    ans[0][1] = 1;
    ans[0][2] = 1;
    ans[1][0] = 1;
    ans[2][1] = 1;

    ans = power_matrix(n - 3, &mut ans);
    println!("{}", (ans[0][0] * 2 + ans[0][1] + ans[0][2]) % 1000000007);
}


fn mulmatrix(a: &mut Vec<Vec<i64>>, b: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>> {

    let mut c: Vec<Vec<i64>> = vec![vec![0; 3]; 3];

    for i in 0..3 {
        for k in 0..3 {
            for j in 0..3 {
                c[i][j] += a[i][k] * b[k][j]; 
                c[i][j] %= 1000000007;
            }
        }
    }

    return c;
}


fn power_matrix(n: i64, v: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>> {

    let mut ans: Vec<Vec<i64>> = vec![vec![0; 3]; 3];
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



