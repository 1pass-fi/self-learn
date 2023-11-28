fn main() {
    
    // let user1 = User {
    //     active: true,
    //     username: "11",
    //     email: "22",
    //     sign_in_count: 1,
    // };
    // println!("{}", user1.email);
    // let subject = AlwaysEqual;
    // println!("{}", subject);
    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
    // println!("({}, {}, {})", black.0, black.1, black.2);
    // let user1 = build_user(String::from("1"), String::from("2"));
    // let user2 = User {
    //     email: String::from("3"),
    //     ..user1
    // };
    // println!("{}", user1.email);
    // println!("{}", user2.email);
    // let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3];

    // assert_eq!(slice, &[2, 3]);
    // let s = String::from("hello world");
    // println!("{}", first_word(&s));
    // let (len1, len2) = split(s);
    // println!("{}, {}", len1, len2);
    // let mut str = String::from("hello world");
    // println!("{}", first_word(&str));
    // str.clear();
    // println!("{}", str);
    // let m = no_dangle();
    // println!("{}", m);
    // let mut s = String::from("hello");
    // let _r1 = &s;
    // let _r2 = &s;
    // println!("{}, {}", _r1, _r2);
    // let r3 = &mut s;
    // println!("{}", r3);
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);
    // let str = String::from("hello");
    // let (len, newstr) = calculate_length(str);
    // println!("{}, {}", len, newstr);
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{}", s1);
    // let temp = 54.0;
    // println!("{}", convert_to_celsius(temp));
    // println!("{}", convert_to_fahrenheit(temp));
    // println!("fibo: {}", fibo(5));
    // write_chrismas(12);
    // for number in (4..1).rev() {
    //     println!("{number}");
    // }
    // println!("liftoff");
    // let a = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("value is: {element}");
    // }
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is {}", a[index]);
    //     index = index + 2;
    // }
    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    // println!("Liftoff!");
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("result: {result}")
    // let condition = true;
    // let number = if condition {5} else {6};
    // println!("{number}");
    // if number < 5 {
    //     println!("Condition was true");
    // } else {
    //     println!("Condition was false");
    // }
    // if number % 4 == 0 {
    //     println!("divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("divisible by 3");
    // } else if number % 3 == 0 {
    //     println!("divisible by 2");
    // } else {
    //     println!("not divisible by 4, 3, or 2");
    // }
    


    // println!("hello, world");
    // another_function(2, 'z');

    // let x = {
    //     let y = 6;
    //     y + 1
    // };
    // println!("{}", x);

    // let x = five(3);
    // println!("The return value: {}", x);
    // let t = true;
    // let t2 = false;
    // let f: bool = false;
    // println!("Boolean values: {}, {f}, {}", t, t2);

    // let c = 'z';
    // let z: char = 'Z';
    // let heart_eyed_cat = '😻';
    // println!("Chars: {}, {z}, {}", c, heart_eyed_cat);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("Tuples: {}, {}, {}", x, y, z);
    // println!("Tuples: {}", tup.1);

    // let a = [1, 2, 3, 4, 5];
    // let months = ["Jan", "Feb", "Mar", "...", "Dec"];
    // println!("Array: {}, {}", a[0], months[1]);

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("Array: {}, {}", a[0], a[3]);

    // let a = [3; 5];
    // println!("Array: {}, {}", a[2], a[4]);

    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter an index number");
    // let mut index = String::new();
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    // let element = a[index];
    // println!("The value of the element at index {index} is : {}", element);

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

// fn another_function(x: i32, y: char) {
//     println!("Another function {x}{y}");
// }

// fn five(x: i32) -> i32 {
//     1 + x + 1
// }

// fn convert_to_celsius(fa: f64) -> f64 {
//     (fa * 9.0/5.0) + 32.0
// }

// fn convert_to_fahrenheit(ce: f64) -> f64 {
//     (ce - 32.0) * 5.0/9.0
// }

// fn fibo(n: u32) -> u32 {
//     if n == 0 || n == 1{
//         1
//     }
//     else {
//         fibo(n-1) + fibo(n-2)
//     }
// }

// fn write_chrismas(n: u32) {
//     for _ in 1..n {
//         println!("Sing a song");
//     }
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let le = s.len();
//     (s, le)
// }

// fn change(s: &mut String) {
//     s.push_str(", world");
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
// fn split(s: String) -> (usize, usize) {
//     let hello = &s[0..2];
//     let world = &s[..s.len()];
//     (hello.len(), world.len())
// }
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         email,
//         username,
//         sign_in_count: 1,
//     }
// }
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// struct AlwaysEqual;
// struct User<'a> {
//     active: bool,
//     username: &'a str,
//     email: &'a str,
//     sign_in_count: u64,
// }