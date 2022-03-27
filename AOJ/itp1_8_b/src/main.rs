fn main() {
  
    let mut v: Vec<i32> = Vec::new();
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        if s == "0\n" {
            break;
        }
        let mut sum = 0;
        for i in s.chars() {
            if i == '\n' {
                break;
            }
            sum += i.to_digit(10).unwrap() as i32; 
        }
        v.push(sum);
    }
   
    for d in v {
        println!("{}", d);
    }

}

