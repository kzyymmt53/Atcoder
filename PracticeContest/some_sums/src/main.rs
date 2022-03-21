use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }
   
    let mut result = 0;
    for i in 1..=n {
        let temp: u32 = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum(); 
        if a <= temp && temp <= b {
            result += i;
        } 
    }
    println!("{}", result);
    
}

