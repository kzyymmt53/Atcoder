use proconio::input;

fn main() {

    input! { 
        n: i64,
        k: i64,
    }

    let mut ans: i64 = 1;
    for i in 0..n {
        if i == 0 {
            ans *= k;
        }else{
            ans *=k - 1;
        }
    }

    println!("{}", ans);

}

