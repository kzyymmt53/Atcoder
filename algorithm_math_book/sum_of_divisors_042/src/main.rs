use proconio::input;

fn main() {

    input! { 
        n: usize,
    }


    let mut ans = 0;
    for i in 1..n+1 {
        for j in (i..=n).step_by(i) {
            ans +=  j as i64 * 1;
        }
    }



    println!("{}", ans);

}

