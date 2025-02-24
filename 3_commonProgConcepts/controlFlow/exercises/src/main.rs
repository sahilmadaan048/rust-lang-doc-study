/*


Convert temperatures between Fahrenheit and Celsius.


*/
use std::io;

fn change_to_celcius(temp: i32) -> i32 {
    (temp - 32) * 5 / 9
}

fn change_to_farenheit(temp: i32) -> i32 {
    9 * (temp) / 5 + 32
}

fn main() {
    println!("Enter the temperature: ");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("temp is not in the right syntax");
    println!();
    let temperature: i32 = temperature.trim().parse().expect("Please type a number");

    println!("Change the unit you want to convert temperature to: (c,f): ");
    let mut change_to = String::new();
    io::stdin()
        .read_line(&mut change_to)
        .expect("please enter a one digit character (c/f)!");

    //convert input string into lowercase before use
    let change_to = change_to.trim().to_lowercase();

    if change_to == "c" {
        let changed_to_celcius = change_to_celcius(temperature);
        println!("{changed_to_celcius}");
    } else {
        let changed_to_farenheit = change_to_farenheit(temperature);
        println!("{changed_to_farenheit}");
    }
}
