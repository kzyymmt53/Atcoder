use proconio::input;

fn main() {

    let mut x:Vec<f32> = Vec::new();
    let mut y:Vec<f32> = Vec::new();

    input! { 
        n: usize,
    }

    for _ in 0..n {
        input! {
            a: f32,
        }
        x.push(a);
    }

    for _ in 0..n {
        input! {
            b: f32,
        }
        y.push(b);
    }

    let mut result1 = 0.0;
    let mut result2 = 0.0;
    let mut result3 = 0.0;
    let mut result4 = 0.0;

    for i in 0..n {
        let temp = (x[i] - y[i]).abs();
        result1 += temp;
        result2 += temp.powf(2.0);
        result3 += temp.powf(3.0);
        if result4 < temp {
            result4 = temp;
        }
    }

    println!("{}", result1);
    println!("{}", result2.powf(1.0/2.0));
    println!("{}", result3.powf(1.0/3.0));
    println!("{}", result4);


}

