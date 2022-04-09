use proconio::input;

fn main() {

    input! { 
        n: usize,
        q: usize,
    }

    let mut v: Vec<i64> = vec![0; n ];

    for _ in 0..q {
        input! {
            l: usize,
            r: usize, 
            x: i64,
        }
            v[l - 1] += x;
            if r != n {
                v[r] -= x;
            }
       
    }

    
    for i in 1..n {
        if v[i] > 0 {
            print!("<");
        }else if v[i] < 0 {
            print!(">");
        }else{
            print!("=");
        }
    }
    println!("");

}

