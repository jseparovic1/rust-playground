pub fn start() {
    let sum_plus_one = {
        let res = sum(10, 5);
        res + 1
    };

    println!("Sum is {}", sum_plus_one);
    println!("10 * 5 = {}", multiplication(10, 5));
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn multiplication(x: i32, y: i32) -> i32 {
    return x * y;
}
