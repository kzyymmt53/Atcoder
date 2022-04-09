use proconio::input;

fn main() {

    input! { 
        t: usize,
        n: usize,
    }

    let mut v: Vec<i64> = vec![0; t];

    for _ in 0..n {
        input! {
            l: usize,
            r: usize,
        }

        v[l] += 1;
        if r < t {
            v[r] -= 1;
        }
    
    }


    let mut ans: i64 = 0;
    for i in v {
        ans += i;
        println!("{}", ans);
        
    }

}

