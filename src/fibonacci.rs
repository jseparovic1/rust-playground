pub fn start() {
    println!("Fib {}", fib(8));
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}
