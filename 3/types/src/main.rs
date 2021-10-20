fn main() {
    let num: u8 = 128;

    //integer
    let num = num + 2;
    println!("num value is {}", num);

    //boolean
    let flag: bool = false;
    println!("flag value is {}", flag);

    //float
    let pi: f32 = 3.14156;
    println!("pi value is {}", pi);

    //char
    let emoji = '💩';
    println!("emoji value is {}", emoji);

    //tuple
    let tup: (i8, char, bool) = (28, 'J', true);

    let twenty_eight = tup.0;

    let letter = tup.1;

    println!("{}", twenty_eight);
    println!("letter value is {}", letter);

    //array
    let my_arr: [u8; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5] === [3, 3, 3, 3 ,3]

    println!("my_arr length is {}", my_arr.len());
    println!("first value of my_arr is {}", my_arr[0]);
}
