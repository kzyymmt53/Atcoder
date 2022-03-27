fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let mut p = String::new();
    std::io::stdin().read_line(&mut p).ok();
   
    let s2: String = s.clone().trim().to_owned() + &s[0..p.chars().count()];
    println!("{}", s2);
   
    let mut result: bool = false;
    let pchars = p.chars();
    let mut pchars_index = 0;
    for i in s2.chars() {
        if i == pchars.clone().nth(pchars_index).unwrap() {
            pchars_index += 1; 
        }else{
            pchars_index = 0;
        }

        if pchars_index == pchars.clone().count() - 1{
            result = true;
            break;
        }

    }
   
    if result {
        println!("Yes");
    }else{
        println!("No");
    }
}
