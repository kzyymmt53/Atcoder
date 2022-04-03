use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    let mut check = true;
    for i in 2..=a {
        if i * i > a {
            break;
        }

        if a % i == 0 {
            check = false;
            break;
        }
    }

    if check {
        println!("Yes");
    }else{
        println!("No");
    }

}

