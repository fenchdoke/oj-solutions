use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n : i32 = s.trim().parse().unwrap();
    for _ in 0..n {
        s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().parse().unwrap();
        println!("{}", if s.len() > 10 {
            let mut t = String::from(s.chars().nth(0).unwrap());
            t += &(s.len() - 2).to_string();
            t += &String::from(s.chars().last().unwrap());
            t
        } else {
            s
        });
    }
}