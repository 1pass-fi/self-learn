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
    println!("Tuples: {}, {}, {}", x, y, z)

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