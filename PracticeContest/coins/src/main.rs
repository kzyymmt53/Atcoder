use proconio::input;

fn main() {
    input! {
        a500: i32,
    }
    
    input! {
        a100: i32,
    }

    input! {
        a50: i32,
    }
  
    input! {
        total: i32,
    }
  
    let mut result = 0;

    for i in 0..=a500 {
        for j in 0..=a100 {
            for k in 0..=a50 {
               if total - 500 * i - 100 * j - 50 * k == 0 {
                  result += 1; 
               }
            }
        }
    }
    println!("{}", result);
}

