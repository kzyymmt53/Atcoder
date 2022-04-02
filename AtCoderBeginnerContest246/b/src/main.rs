use proconio::input;

fn main() {

    input! { 
        a: f64,
        b: f64,
    }

    let l = (a*a + b*b).powf(0.5);

    let x = 1.0/l * a;
    let y = 1.0/l * b;

    println!("{:.12} {:.12}", x, y);

}

