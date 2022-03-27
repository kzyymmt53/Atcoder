use proconio::input;

fn main() {
    
    let mut b: Vec<i32> = Vec::new();
    loop {
        input! {
            n: i32,
            x: i32,
        }
        if n == 0 && x == 0 {
            break;
        }
        let mut count = 0;
        for i in 1..n+1 {
            for j in i+1..n+1 {
                for k in j+1..n+1 {
                    if i + j + k == x{
                        count += 1;
                    }
                }
                
            }
        }
        b.push(count)
    }
   
    for d in b {
        println!("{}", d);
    }
}

