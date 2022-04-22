use proconio::input;

fn main() {

    input! { 
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let mut ans;
    ans = a * c;

    if a * d > ans {
        ans = a * d;
    }

    if b * c > ans {
        ans = b * c;
    }

    if b * d > ans {
        ans = b * d;
    }


    println!("{}", ans);

}

