use proconio::input;

fn main() {

    input! { 
        n: usize,
        d: [(i64, i64); n],
    }

    let mut v: Vec<i64> = vec![0; n + 1];
    let mut v2: Vec<i64> = vec![0; n + 1];

    let mut a = 0;
    let mut b = 0;
    for i in 1..n+1 {
        if d[i - 1].0 == 1 {
            a += d[i - 1].1;
        }else {
            b += d[i - 1].1;
        }

        v[i] = a;
        v2[i] = b;
    }

    input! {
        q: usize,
        d2: [(i64, i64); q],
    }

    for i in d2 {
        println!("{} {}", v[i.1 as usize] - v[i.0 as usize -1], v2[i.1 as usize] - v2[i.0 as usize -1]);
    }

}

