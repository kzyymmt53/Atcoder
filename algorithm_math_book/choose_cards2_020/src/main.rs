use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! {
        a: [i64; n],
    }

    let mut ans = 0;

    for i in 0..a.len() {
        for j in i+1..a.len() {
            for k in j+1..a.len() {
                for l in k+1..a.len() {
                    for m in l+1..a.len() {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

