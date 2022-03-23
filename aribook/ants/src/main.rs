use proconio::input;

fn main() {
    input! {
        l: i32,
    }
    input! {
        n: i32,
    }
    
    let mut v: Vec<i32> = Vec::new();
  
    for _ in 0..n {
        input! {
            a: i32,
        }
        v.push(a);
    }
   
    let mut result = 0;
    let mut result2 = 0;

    for d in v {
        result = std::cmp::max(result, std::cmp::min(d, l - d));
        result2 = std::cmp::max(result2, std::cmp::max(d, l - d));
    }

    println!("最小 {}", result);
    println!("最大 {}", result2);

   

}

