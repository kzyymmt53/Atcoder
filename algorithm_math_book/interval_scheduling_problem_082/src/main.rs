use proconio::input;

fn main() {

    input! { 
        n: i64,
    mut v: [(i64, i64); n], 
    }

    v.sort_by(|a, b| a.1.partial_cmp(&(b.1)).unwrap());


    let mut start = 0;
    let mut ans = 0;
    for i in v {
        if start <= i.0 {
            ans+= 1;
            start = i.1;
        }
    }

    println!("{}", ans);
}

