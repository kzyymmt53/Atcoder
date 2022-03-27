use proconio::input;

fn main() {
    
    let mut b: Vec<char> = Vec::new();
    loop {
        input! {
            m: i32,
            f: i32,
            r: i32,
        }
        if m == -1 && f == -1 && r == -1 {
            break;
        }
        if m == -1 || f == -1 {
            b.push('F'); 
        }else if m + f >= 80 {
            b.push('A'); 
        }else if m + f >= 65 {
            b.push('B'); 
        }else if m + f >= 50 {
            b.push('C'); 
        }else if m + f >= 30 {
            if r >= 50 {
                b.push('C');
            }else{
                b.push('D');
            }
        }else{
            b.push('F');
        }
    }
   
    for d in b {
        println!("{}", d);
    }
}

