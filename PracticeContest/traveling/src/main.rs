use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    
    let mut t: Vec<i32> = Vec::new();
    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();
   
    t.push(0);
    x.push(0);
    y.push(0);

    for _ in 0..n {
        input! {
            a: i32,
            b: i32,
            c: i32,
        }
        t.push(a);
        x.push(b);
        y.push(c);
    }

    let mut result = true;
    for (i, _v) in t.iter().enumerate() {
        if i == t.len() - 1 {
            break;
        }
        let t2 = t[i+1] - t[i];
        let dist = (x[i+1] - x[i]).abs() + (y[i+1] - y[i]).abs();
      
        if t2 < dist {
            result = false;
        }
  
        if t2 % 2 != dist % 2 {
            result = false;
        }
        
    } 
  
    if result {
        println!("Yes");
    }else {
        println!("No");
    }
}

