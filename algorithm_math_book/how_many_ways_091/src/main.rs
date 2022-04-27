use proconio::input;

fn main() {

    input! { 
        n: i64,
        x: i64,
    }

    let mut ans: i64 = 0;
    for i in 1..=n {
        for j in i+1..=n {
            for k in j+1..=n {
                if i + j + k == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);

}

