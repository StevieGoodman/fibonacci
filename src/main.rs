use std::io::stdin;

fn main() {
    loop {
        let last_index = get_i32_from_input("Please select the index of you wish to fibonacci number you wish to calculate");
        let mut result = 0;
        let mut last_number = 1;
        let mut last_last_number = 0;
        for _ in 1..=last_index {
            result = last_number + last_last_number;
            last_last_number = last_number;
            last_number = result;
        }
        println!("The {}th fibonacci number is {}", last_index, result)
    }
}

fn get_i32_from_input<>(input_prompt: &str) -> i32 {
    loop {
        // Gets user input
        println!("{}: ", input_prompt);
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        };
        // Converts user input to i32
        let input: i32 = match input.trim().parse() {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        };
        break input
    }
}
