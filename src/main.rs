use std::io;

fn main() {
    print!("{}[2J", 27 as char);
    println!("Temperature Converter!");

    loop {
        println!("Convert (1)[°C => °F] or (2)[°F => °C]?");

        let mut choice = String::new();
        let mut value = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line!");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        if choice == 1 {
            println!("Coverting °C => °F, please input!");

            io::stdin().read_line(&mut value)
                .expect("Failed to read line!");

            let value: f64 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                },
            };

            let adder: f64 = 32.0;
            let value_converted = value * 1.8 + adder;

            print!("{}[2J", 27 as char);
            println!("{}°C is {}°F", value, value_converted);
            break;

        } else if choice == 2 {
            println!("Coverting °F => °C, please input!");

            io::stdin().read_line(&mut value)
                .expect("Failed to read line!");

            let value: f64 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                },
            };

            let subber: f64 = 32.0;
            let value_converted = (value - subber) / 1.8;

            print!("{}[2J", 27 as char);
            println!("{}°F is {:.1}°C", value, value_converted);
            break;

        } else {
            print!("{}[2J", 27 as char);
            println!("Please type 1 or 2 to make a choice!");
        }
    }
}