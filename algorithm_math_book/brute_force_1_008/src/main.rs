use proconio::input;

fn main() {

    input! { 
        n: i32,
        s: i32,
    }

    let mut ans = 0;
    for i in 1..=n {
        let m = s - i;
        if m > 0{
            if m <= n {
                ans += m;
            }else{
                ans += n;
            }
        }
    }

    println!("{}", ans);

}

