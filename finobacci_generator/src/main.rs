use std::io;

fn main() {
    println!("Enter which fibonacci number you want to generate:");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        },
    };

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    //generate fibonacci number
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    //print the generated fibonacci number
    println!("The fibonacci number is: {}", c);
}
