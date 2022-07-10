fn main() {
    let address = String::from("1.1.1.1");
    let res = defang_i_paddr(address);
    println!("changed value: {}", res);
}

pub fn defang_i_paddr(address: String) -> String {
    let mut s = String::new();
    for ch in address.chars() {
        if ch == '.' {
            s.push_str("[.]");
        }else{
            s.push(ch);
        }
    }
    s
}