pub fn start() {
    let x: u8 = 5;
    println!("Immutable X is {}", x);
    // cannot assign twice to a immutable variable
    //x = 10;

    let mut y: u8 = 1;
    println!("Mutable Y is {}", y);
    y = 2;
    println!("Mutable Y is {}", y);

    // Constant must have type defined...
    const MAX_POINTS: u32 = 100_000;
    println!("Max points {}", MAX_POINTS);

    // Shadowing
    let x = 10; //Each declaration shadows a previous one
    let x = x + 1;
    let x = x * 2;
    let x = "23"; // Type can also be changed...
    println!("The value of x is {}", x);

    //Types
    // Scalar types
    // Integers, float, booleans, characters (represents a single value)

    let num: u16 = 1_000;
    println!("Visual separator is okay {}", num);

    let fl: f32 = 3.0;
    let are_you_normal: bool = false;
    let c: char = 'c'; // 4 bytes -> 32 bit?
    let tup: (i32, f64, bool) = (10, 10.4, false);
    let (x, y, z) = tup; //destructing

    let nums = [1, 2, 3]; //arrays
    println!("All vars: {:?}", (fl, are_you_normal, c, tup, nums));

    let mut a: [i32; 2] = [1, 2];

    let first = a[0];
    let second = a[1];

    println!("Arr: {:?}", (a, first, second));
}
