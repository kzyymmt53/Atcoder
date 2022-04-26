use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let mut rest: i64 = n;
    let mut ans: i64 = rest / 10000;
    rest = rest % 10000;
    ans += rest / 5000;
    rest = rest % 5000;

    ans += rest / 1000;


    println!("{}", ans);
    

}

