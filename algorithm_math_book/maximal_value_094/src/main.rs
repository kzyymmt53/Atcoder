use proconio::input;

fn main() {

    input! { 
        n: usize,
        b: [i64; n - 1],
    }


    let mut ans: i64 = 0;
    let mut pre: i64 = 100000;
    for i in (0..n-1).rev() {
        if pre > b[i] {
            ans += b[i];
            pre = b[i];
        }else{
            ans += pre;
            pre = b[i];
        }
    }
    
    ans += pre;

    println!("{}", ans);

}

