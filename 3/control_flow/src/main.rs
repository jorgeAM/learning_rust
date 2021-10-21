fn main() {
    // conditions
    let age: u8 = 2;

    if age >= 18 {
        println!("Eres un adulto");
    } else {
        println!("Eres un niño");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // return value from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // returned value
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let arr = [13, 4, 24, 56, 65, 100, 99];

    for element in arr {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
