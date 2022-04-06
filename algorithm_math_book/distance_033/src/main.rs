use proconio::input;

fn main() {

    input! { 
        a: [(f64, f64); 3],
    }

    let check: f64 = (a[0].0 - a[1].0) * (a[2].0 - a[1].0) + (a[0].1 - a[1].1) * (a[2].1 - a[1].1);
    let check2: f64 = (a[0].0 - a[2].0) * (a[1].0 - a[2].0) + (a[0].1 - a[2].1) * (a[1].1 - a[2].1);
    
    let ans: f64;
    if check < 0.0 {
       ans = ((a[0].0 - a[1].0).powf(2.0) + (a[0].1 - a[1].1).powf(2.0)).powf(0.5); 
    }else if check2 < 0.0 {
       ans = ((a[0].0 - a[2].0).powf(2.0) + (a[0].1 - a[2].1).powf(2.0)).powf(0.5); 
    }else{
       let bc :f64 = ((a[2].0 - a[1].0).powf(2.0) + (a[2].1 - a[1].1).powf(2.0)).powf(0.5);
       let area :f64 = ((a[0].0 - a[2].0) * (a[1].1 - a[2].1) - (a[0].1 - a[2].1) * (a[1].0 - a[2].0)).abs();
       ans = area / bc;
    }
    println!("{:.12}", ans);

}

