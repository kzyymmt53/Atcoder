use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    if a % 2 == 0 {
        println!("Yes");
    }else{
        println!("No");
    }

}

