use std::io;

fn main() {
    println!("Hello, world!");

    println!("Enter a temperature with either 'C' or 'F' at the end for conversion:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp = temp.trim();
    if temp.ends_with('C') || temp.ends_with('c') {
        println!("You entered a Celsius temperature.");
        let temp: f64 = match temp[..temp.len()-1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number.");
                return;
            },
        };

        let temp = (temp * 9.0/5.0) + 32.0;
        println!("The temperature in Fahrenheit is: {:.2}", temp);
    } else if temp.ends_with('F') || temp.ends_with('f') {
        println!("You entered a Fahrenheit temperature.");
        let temp: f64 = match temp[..temp.len()-1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number.");
                return;
            },
        };

        let temp = (temp - 32.0) * 5.0/9.0;
        println!("The temperature in Celsius is: {:.2}", temp);
    } else {
        println!("You did not enter a valid temperature.");
    }

}
