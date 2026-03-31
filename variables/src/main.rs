fn shadowing() {
    let mut x = 5;

    println!("Showing");
    println!("ini var x: {:p}", &x);

    x += 1;
    println!("mut var x: {:p}", &x); // Just mutable value -> same address

    let x = x + 1; // new variable shadowing the previous variable -> different address
    println!("new var x: {:p}", &x);
}

fn constant_var() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("three hours in secondes = {THREE_HOURS_IN_SECONDS}")
}

fn scalar() {
    println!("===== Scalar types =====");
    let f: bool = true;
    println!("boolean: {}", f);

    let number: i8 = 127;
    println!("Signed integer: {}", number);

    let number: u8 = 255;
    println!("Unsigned integer: {}", number);

    let float_num: f32 = 42.7;
    println!("Float: {}", float_num);

    let z: char = 'z';
    println!("Character: {}", z);
}

fn compound() {
    println!("===== Compound types =====");
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("Extract y from tuple: {y}");

    // array
    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    for (i, val) in arr.iter().enumerate() {
        println!("addr: {:p}, value: {}", &arr[i], val)
    }
}

fn main() {
    shadowing();

    constant_var();

    scalar();

    compound();
}
