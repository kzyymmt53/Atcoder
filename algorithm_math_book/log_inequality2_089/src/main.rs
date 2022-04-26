use proconio::input;

fn main() {

    input! { 
        a: u64,
        b: u64,
        c: u64,
    }

    let mut p = 1;
    if c == 1 {
        println!("No");
        return;
    }
    for _ in 1..=b {
        if a / c < p { 
            println!("Yes");
            return;
        }
        p *= c;
    }

   println!("No");
    

}

