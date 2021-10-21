fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    another_function();
    another_function_with_args(8);

    let x = another_function_with_returns();
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function!");
}

fn another_function_with_args(x: u8) {
    println!("The value of x is {}", x)
}

// If the last line of expression does not have semicolons return that value (in this case 6)
fn another_function_with_returns() -> u8 {
    6
    // return 6;
}
