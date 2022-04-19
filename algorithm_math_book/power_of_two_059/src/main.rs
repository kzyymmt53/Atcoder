use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    if a % 4 == 0 {
        println!("6");
    }else if a % 4 == 1 {
        println!("2");
    }else if a % 4 == 2 {
        println!("4");
    }else {
        println!("8");
    }

}

