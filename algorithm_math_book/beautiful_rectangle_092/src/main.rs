use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let mut ans: i64 = 10000000000000;


    for i in 1..n+1 {
        if i * i > n {
            break;
        }

        if n % i == 0 {
            let temp: i64 = i * 2 + n / i * 2;
            if ans > temp {
                ans = temp;
            }
        }
    }

    println!("{}", ans);

}

