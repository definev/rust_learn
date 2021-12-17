fn main() {
    println!("Fibo 20: {}", fibo(25));
}

fn fibo(n: u8) -> i128 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        let mut x1 = 1;
        let mut x2 = 2;

        for _ in 2..n {
            let x3 = x1 + x2;
            x1 = x2;
            x2 = x3;
        }

        return x2;
    }
}
