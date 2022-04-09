use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    let mut v: Vec<i64> = vec![0; n - 1];

    for i in 0..n-1 {
        input! {
            a: i64,
        }
        if i == 0 {
            v[i] = a;
        }else{
            v[i] = v[i - 1] + a;
        }

    }


    input! {
        m: usize,
    }

    let mut ans:i64 = 0;
    let mut start: i64 = 0;

    for i in 0..m {
        input! {
            b: i64,
        }
        if i != 0 {
            if start < b {
               if start - 2 < 0 {
                   if b - 2 >= 0 {
                       ans += v[b as usize - 2]; 
                   }
               }else{
                   ans += v[b as usize - 2] - v[start as usize - 2];
               }
            }else if start > b {
               if b - 2 < 0 {
                  if start - 2 >= 0 {
                      ans += v[start as usize - 2];
                  }
               }else{
                   ans += v[start as usize - 2] - v[b as usize - 2];
               }
            }
        }
        start = b as i64;

    }

    println!("{}", ans);

}

