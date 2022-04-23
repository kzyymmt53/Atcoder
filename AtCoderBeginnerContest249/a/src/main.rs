use proconio::input;

fn main() {

    input! { 
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        x: f64,
    }

    let mut temp = x;
    let mut aans :f64 = 0.0;  
    let mut bans :f64 = 0.0;  

    loop {
        if temp <= 0.0{
            break;
        }

        if temp < a {
            aans += temp * b; 
            break;
        }else{
            aans += a * b;
            temp -= a;
        }

        if temp < c {
            break;
        }else{
            temp -= c;
        }
    }

    temp = x;
    loop {
        if temp <= 0.0{
            break;
        }

        if temp < d {
            bans += temp * e;
            break;
        }else{
            bans += d * e;
            temp -= d;
        }

        if temp < f {
            break;
        }else{
            temp -= f; 
        }
    }


    
    if aans > bans {
        println!("Takahashi");
    }else if bans > aans {
        println!("Aoki");
    }else{
        println!("Draw");
    }



}

