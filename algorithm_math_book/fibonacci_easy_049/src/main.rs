use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    let mut v: Vec<i64> = vec![1; n + 1];

    for i in 3..n + 1 {
        v[i] = (v[i - 1] + v[i - 2]) % 1000000007;
    }

    println!("{}", v[n] % 1000000007);

}


