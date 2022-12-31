use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let w : i32 = line.trim().parse().unwrap();
    println!("{}", if w % 2 == 0 && w > 2 {
        "YES"
    } else {
        "NO"
    });
}