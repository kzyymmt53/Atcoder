use proconio::input;

fn main() {

    input! { 
        x1: f64,
        y1: f64,
        r1: f64,
    }

    input! {
        x2: f64,
        y2: f64,
        r2: f64,
    }

    let l = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).powf(0.5);
    if l == r1 + r2 {
        println!("4");
    }else if l > r1 + r2 {
        println!("5");
    }else if r1 == l + r2 || r2 == l + r1 {
        println!("2");
    }else if r1 < r2 && l + r1 < r2 || r1 >= r2 && l + r2 < r1 { 
        println!("1");
    }else{
        println!("3");
    }



}

