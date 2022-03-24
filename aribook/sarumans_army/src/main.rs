use proconio::input;

fn main() {
    input! {
        n: usize,
        r: i32,
    }
    
    let mut x: Vec<i32> = Vec::new();
    for _ in 0..n {
        input! {
            s: i32,
        }
        x.push(s);
    }
    
    let mut result = 0;
    let mut i = 0;    
   
    while i < n {
        let mut start = x[i];
        i += 1;
        while i < n && start + r >= x[i] {
            i += 1;
        }
        start = x[i - 1];
        
        while i < n && start + r >= x[i] {
            i += 1;
        }
        result += 1;
    }
    println!("{}", result);
}

