use proconio::input;

fn main() {

    input! { 
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    }


    loop {
        if v - a < 0 {
            println!("{}", 'F');
            break;
        }else if v - a - b < 0 {
            println!("{}", 'M');
            break;
        }else if v - a - b - c < 0 {
            println!("{}", 'T');
            break;
        }
        v = v - a - b - c;
    }

}

