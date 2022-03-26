use proconio::input;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    loop {
        input! {
            a: i32,
        }
        if a == 0 {
            break;
        }
        v.push(a);
    }
   
    for (i, d) in v.iter().enumerate() {
        println!("Case {}: {}", i + 1, d); 
    }   
    
}

