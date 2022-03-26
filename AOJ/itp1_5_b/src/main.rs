use proconio::input;

fn main() {
    
    let mut w: Vec<i32> = Vec::new();
    let mut h: Vec<i32> = Vec::new();
    loop {
        input! {
            a: i32,
            b: i32,
        }
        if a == 0 && b == 0{
            break;
        }
        h.push(a);
        w.push(b);
    }

    for i in 0..h.len() {
      for j in 0..h[i] {
          for k in 0..w[i] {
             if j == 0 || j == h[i] - 1 || k == 0 || k == w[i] - 1 {
                 print!("#");
             }else{
                 print!(".");
             }
          }
          println!("");
      }
      println!("");
    } 
    
}

