use proconio::input;

fn main() {

    input! { 
        n: usize,
        a: [i64; n],
    }

    let mut a1: Vec<i64> = Vec::new();
    let mut a2: Vec<i64> = Vec::new();


    for i in 1..n {
        a1.push(a[i - 1] * (n - i) as i64);
        a2.push(a[i] * i as i64);

    }


    let mut ans = 0;
    for i in a2 {
        ans += i; 
    }

    for i in a1 {
        ans -= i;
    }

    println!("{}", ans);

}

