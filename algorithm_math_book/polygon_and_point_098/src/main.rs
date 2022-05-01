use proconio::input;

fn main() {

    input! { 
        n: usize,
        v: [(i64, i64); n],
        target: (i64, i64),
    }

    let mut cnt = 0;

    for i in 0..n {
        let mut xa: i64 = v[i].0 - target.0; 
        let mut ya: i64 = v[i].1 - target.1; 
        let mut xb: i64 = v[(i + 1) % n].0 - target.0; 
        let mut yb: i64 = v[(i + 1) % n].1 - target.1; 

        if ya > yb {
            let temp = xa;
            xa = xb;
            xb = temp; 
            
            let temp = yb;
            yb = ya;
            ya = temp;
        }

        if ya <= 0 && 0 < yb && xa * yb - xb * ya < 0 {
			cnt += 1;
		}
    }

    if cnt % 2 == 1 {
        println!("INSIDE");
    }else{
        println!("OUTSIDE");
    }

}

