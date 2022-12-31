use std::io;

fn main() {
    let mut line = String::new();
    let mut res : i32 = 0;
    io::stdin().read_line(&mut line).unwrap();
    let n : i32 = line.trim().parse().unwrap();
    for _ in 0..n {
        line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let list = line.trim().split(" ");
        let mut cnt : i32 = 0;
        for s in list {
            let t : i32 = s.trim().parse().unwrap();
            cnt += t;
        }
        res += if cnt >= 2 {
            1
        } else {
            0
        };
    }
    println!("{}", res);
}