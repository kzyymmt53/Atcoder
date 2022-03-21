use std::io;

fn main() {

    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("入力えらー");

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("入力えらー");
     
    let v: Vec<&str> = a.trim().split(" ").collect();
    
    let mut result = -1;

    for i in v {
        let num: i32 = i.parse().unwrap();
        let s: String = format!("{:b}", num);
        let mut temp = 0;
        for j in s.as_str().chars().rev() {
            if j == '1' {
                break;
            }
            temp += 1; 
        }
        if result == -1 {
            result = temp;
        }else{
            if result > temp {
                result = temp;
            }
        } 
    } 
    println!("{}", result);
}
