use std::io;

fn main() {
    let t = true;
    let t2 = false;
    let f: bool = false;
    println!("Boolean values: {}, {f}, {}", t, t2);

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("Chars: {}, {z}, {}", c, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuples: {}, {}, {}", x, y, z);
    println!("Tuples: {}", tup.1);

    let a = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "...", "Dec"];
    println!("Array: {}, {}", a[0], months[1]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {}, {}", a[0], a[3]);

    let a = [3; 5];
    println!("Array: {}, {}", a[2], a[4]);

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an index number");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is : {}", element);

    // let spaces = "  ";
    // let spaces = spaces.len();
    // println!("Length: {}", spaces);
    // let guess: i16 = "42".parse().expect("");
    // println!("Guess: {}", guess);

    // let x = 2.0;
    // let y: f32 = 3.0;
    // println!("Floating numbers: {}, {}", x, y);

    // let sum = 5 + 10;
    // let diff = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;
    // let remainder = 43 % 5;
    // println!("New values: {}, {}, {}, {}, {}, {}", sum, diff, product, quotient, truncated, remainder);
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("Three hours value in seconds: {}", THREE_HOURS_IN_SECONDS);

    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    
    // println!("The value of x is: {x}");
}