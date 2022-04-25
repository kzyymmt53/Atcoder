use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    let ans = (2 * 1 + (a - 2)) * (a - 1) / 2;

    println!("{}", ans);

}

