use proconio::input;

fn main() {

    input! { 
        n: usize,
        q: usize,
    }

    let mut v: Vec<i64> = Vec::new();
    for i in 0..n {
        input! {
            a: i64,
        }
        if i == 0 {
            v.push(a);
        }else{
            v.push(v[i - 1] + a);
        }
    }

    input! {
        b: [(i64, i64); q],
    }

    for d in b {
        if d.0 - 2 < 0 {
            println!("{}", v[d.1 as usize - 1]);
        }else{
            println!("{}", v[d.1 as usize - 1] - v[d.0 as usize - 2]);
        }
    }


}

