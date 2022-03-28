use proconio::input;

fn main() {

    let mut v:Vec<f32> = Vec::new();

    loop {
        let mut sum: f32 = 0.0;
        let mut temp:Vec<f32> = Vec::new();
        input! { 
            n: usize,
        }

        if n == 0 {
            break;
        }
        for _ in 0..n {
            input! {
                a: f32,
            } 
            temp.push(a);
            sum += a;
        }
        let ave = sum / n as f32;
        let mut result: f32 = 0.0;
        for d in temp {
            result += (d - ave).powf(2.0)
        }
        v.push((result / n as f32).powf(0.5));
    }

    for d in v {
        println!("{}", d); 
    }

}

