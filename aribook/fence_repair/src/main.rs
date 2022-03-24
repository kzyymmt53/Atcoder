use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut v: Vec<i32> = Vec::new();
    let mut n = n;
    for _ in 0..n {
        input! {
            s: i32,
        }
        v.push(s);
    }

    let mut ans = 0;
    while n > 1 {
        let mut m1 = 0; 
        let mut m2 = 1;
      
        for i in 2..n {
            if v[i] < v[m1] {
                m2 = m1;
                m1 = i;
            }else if v[i] < v[m2] {
                m2 = i; 
            }
        }

        
        let t = v[m1] + v[m2];
        ans += t;
        v[m1] = t;
        v[m2] = v[n - 1];
        n -= 1;
       
    }
    println!("{}", ans);
}
