use proconio::input;

fn main() {

    input! { 
        a: i32,
        b: i32,
        c: i32,
        x: i32,
        y: i32,
    }

    if a + b < c * 2 {
        println!("{}", a * x + b * y);
    }else{
        if x <= y {
            let mut temp = x * c * 2;
            if (y - x) * 2 * c < (y - x) * b {
                temp += (y - x) * 2 * c;
            }else{
                temp += (y - x) * b;
            }

            println!("{}", temp);
        }else{
            let mut temp = y * c * 2;
            if (x - y) * c * 2 < (x - y) * a {
                temp += (x - y) * c * 2;
            }else{
                temp += (x - y) * a;
            }
            println!("{}", temp);
        }
    }


}

