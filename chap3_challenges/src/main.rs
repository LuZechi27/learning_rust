use std::io;

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius.");

    let mut temperature = String::new();

    loop {
        println!("Please input the temperature.");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Feiled to read  the line.");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid!"); continue},
        };

        let converted_temp = convert_temperature_f_to_c(temperature, 'C');
        println!("{temperature}°C in °F is: {converted_temp}");
        let converted_temp = convert_temperature_f_to_c(temperature, 'F');
        print!("{temperature}°F in °C is: {converted_temp}");
        break;
    }

    println!("\n");
    println!("Generate the nth Fibonacci number.");

    let mut number = String::new();

    loop {
        println!("Please input the number.");
        io::stdin()
            .read_line(&mut number)
            .expect("Feiled to read  the line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid!"); continue},
        };

        let n_fibonacci = generate_nth_fibonacci(number);

        println!("{number}º Fibonacci is: {n_fibonacci}");
        break;
    }

}


fn convert_temperature_f_to_c(temp: f32, scale: char) -> f32 {   
    match scale {
        'C' => return (temp * 9.0/5.0) + 32.0,
        'F' => return (temp - 32.0) * 5.0/9.0,
        _ => return temp
    } 
}

fn generate_nth_fibonacci(n: u32) -> u32 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => {
            let mut result: u32 = 1;
            let mut prev: u32 = 0;
            for _i in 1..n {
                result = result + prev;
                prev = result - prev;
            }
            return result
        }
    }
}
