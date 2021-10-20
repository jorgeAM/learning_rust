fn main() {
    // mutable variables
    let mut x = 5;
    println!("the value of x is: {}", x);

    x = 6;
    println!("the value of x is: {}", x);

    //immutable variables
    let y = 10;
    println!("the value of y is: {}", y);

    //constant

    // const MAX_POINTS: u32 = 100_000;
    const MAX_POINTS: u32 = 100000;

    println!("constant MAX_POINTS is {}", MAX_POINTS);

    // shadowing

    let num = 2;

    let num = num + 1;

    let num = num * 10;

    println!("the value of num is: {}", num);

    let space = "    ";

    println!("space: {}", space);

    let space = space.len();
    println!("space: {}", space);
}
