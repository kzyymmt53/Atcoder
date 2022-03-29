use proconio::input;

fn main() {

    input! { 
        a: i32,
    }
    
    let mut flag = 0;
    for i in 1..10 {
        if a % i == 0  && a / i <= 9 && 1 <= a/i {
            println!("Yes");
            flag = 1;
            break;
        }
    }

    if flag == 0 {
        println!("No");
    }

}

