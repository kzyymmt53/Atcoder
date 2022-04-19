use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    if a % 4 == 0 {
        println!("Second");
    }else{
        println!("First");
    }

}

