use proconio::input;

fn main() {
    input! {
         n: i32,
    }

    input! {
         total: i32,
    }
    
    let mut flag = 0;

    for i in 0..=n {
        for j in 0..=(n-i) {
            let k = n - i - j;
            if total - i * 10000 - j * 5000 - k * 1000 == 0 {
                println!("{} {} {}", i, j, k);
                flag = 1;
                break; 
            }
       }
       if flag == 1 {
            break;
       }
    }
    if flag == 0 {
        println!("-1 -1 -1");
    }
}

