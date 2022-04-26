use proconio::input;

fn main() {

    input! { 
        n: usize,
    mut a: [i64; n],
    mut b: [i64; n],
    }

    a.sort();
    b.sort();

    let mut ans: i64 = 0;
    for i in 0..a.len() {
        ans += (a[i] - b[i]).abs();
    }

    println!("{}", ans);
}

