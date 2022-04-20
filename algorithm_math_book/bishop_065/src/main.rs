use proconio::input;

fn main() {

    input! { 
        h: i64,
        w: i64,
    }

    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    if h * w % 2 == 0 {
        println!("{}", h * w / 2);
    }else{
        println!("{}", h * w / 2 + 1);
    }


}

