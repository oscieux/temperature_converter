use std::io;

fn main() {

    println!("Welcome to temperature converter");

    loop {
        println!("Please choose input unit (C, K or F)");
        let mut input_unit = String::new();
        io::stdin().read_line(&mut input_unit)
            .expect("Failed to read line");

        println!("Please choose ouput unit (C, K or F)");
        let mut output_unit = String::new();
        io::stdin().read_line(&mut output_unit)
            .expect("Failed to read line");

        println!("Please input your temp");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let output_temp:f32 = if input_unit.trim() == "F" {
            if output_unit.trim() == "C" {
                (temp - 32.0) / 1.8
            } else if output_unit.trim() == "K" {
                ((temp - 32.0) / 1.8) + 273.15
            } else {
                temp * 1.0
            }
        } else if input_unit.trim() == "C" {
            if output_unit.trim() == "F" {
                temp * 1.8 + 32.0
            } else if output_unit.trim() == "K" {
                temp + 273.15
            } else {
                temp * 1.0
            } 
        } else {
            if output_unit.trim() == "C" {
                temp - 273.15
            } else if output_unit.trim() == "F" {
                (temp - 273.15) * 1.8 + 32.0
            } else {
                temp * 1.0
            }
        };

        println!("{}{} = {}{}", 
            temp, input_unit.trim(), output_temp, output_unit.trim());


    }
}
