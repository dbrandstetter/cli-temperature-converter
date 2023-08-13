extern crate core;

mod conversion;

use std::io;
use std::string::ToString;
use crate::conversion::*;

const CELSIUS_MIN: f64 = -273.15;
const FAHRENHEIT_MIN: f64 = -459.67;
const KELVIN_MIN: f64 = 0.0;

const CELSIUS_SYMBOL: &str = "C";
const FAHRENHEIT_SYMBOL: &str = "F";
const KELVIN_SYMBOL: &str = "K";


fn main() {
	greet_user();

	let initial_unit: String = get_unit("initial".to_string());
	let target_unit: String = get_unit("target".to_string());
	let mut temp_initial: f64;
	let mut temp_converted: f64;

	loop {
		temp_initial = get_initial_temp(initial_unit.to_string());

		temp_converted = convert_temperature(initial_unit.to_string(), target_unit.to_string(), temp_initial);

		println!("{temp_initial}{initial_unit} equates to {temp_converted}{target_unit}\n");
	}
}

fn convert_temperature(initial_unit: String, target_unit: String, temp_initial: f64) -> f64 {
	return match initial_unit.as_str() {
		CELSIUS_SYMBOL => {
			match target_unit.as_str() {
				FAHRENHEIT_SYMBOL => celsius_to_fahrenheit(temp_initial),
				KELVIN_SYMBOL => celsius_to_kelvin(temp_initial),

				// Initial and target unit have to be alike
				_ => temp_initial,
			}
		}
		FAHRENHEIT_SYMBOL => {
			match target_unit.as_str() {
				CELSIUS_SYMBOL => fahrenheit_to_celsius(temp_initial),
				KELVIN_SYMBOL => fahrenheit_to_kelvin(temp_initial),

				// Initial and target unit have to be alike
				_ => temp_initial,
			}
		},
		KELVIN_SYMBOL => {
			match target_unit.as_str() {
				CELSIUS_SYMBOL => kelvin_to_celsius(temp_initial),
				FAHRENHEIT_SYMBOL => kelvin_to_fahrenheit(temp_initial),

				// Initial and target unit have to be alike
				_ => temp_initial,
			}
		},
		// Initial and target unit have to be alike
		_ => temp_initial,
	};
}


// Welcomes the user
fn greet_user() {
	// Politeness is key :-)
	println!("-----------------------------------------------------------");
	println!("Hi Rustacean, welcome to this simple temperature converter!");
	println!("-----------------------------------------------------------");
}


// Prompts user for temperature unit and validates input
fn get_unit(input: String) -> String {
	let mut unit: String = String::new();

	while invalid_unit(unit.to_string().trim().to_lowercase()) {
		println!("Please select the {} unit:\n[C] Celsius, [F] Fahrenheit, [K] Kelvin", input);

		unit.clear();

		io::stdin()
			.read_line(&mut unit)
			.expect("Failed to read line")
			.to_string();
	}

	return get_unit_symbol(unit.to_string());
}


fn get_unit_symbol(input: String) -> String {
	if input.contains("celsius") || input.starts_with("c") {
		return CELSIUS_SYMBOL.to_string();
	} else if input.contains("kelvin") || input.starts_with("k") {
		return KELVIN_SYMBOL.to_string();
	} else if input.contains("fahrenheit") || input.starts_with("f") {
		return FAHRENHEIT_SYMBOL.to_string();
	}

	return "".to_string();
}


fn invalid_unit(input: String) -> bool {
	if input.contains("celsius") || input.starts_with("c") {
		println!("-> Unit [Celsius] selected.\n");
		return false;
	} else if input.contains("kelvin") || input.starts_with("k") {
		println!("-> Unit [Kelvin] selected.\n");
		return false;
	} else if input.contains("fahrenheit") || input.starts_with("f") {
		println!("-> Unit [Fahrenheit] selected.\n");
		return false;
	}

	return true;
}

fn get_initial_temp(unit: String) -> f64 {
	let mut input: String = String::new();

	'getting_degrees: loop {
		println!("Please input degrees {}:", if unit == "F" { "Fahrenheit".to_string() } else if unit == "K" { "Kelvin".to_string() } else if unit == "C" { "Celsius".to_string()
		} else { "".to_string() });

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line")
			.to_string();

		let value: f64 = input.trim().parse().unwrap();

		if valid_degree_value(unit.to_string(), value) {
			break 'getting_degrees;
		}
	}

	return input.trim().parse().unwrap();
}

fn valid_degree_value(unit: String, value: f64) -> bool {
	if value < FAHRENHEIT_MIN && unit == "F" {
		eprintln!("Fahrenheit temperatures cannot be lower than {FAHRENHEIT_MIN}!\nYour input: {value}");
		return false;
	} else if value < CELSIUS_MIN && unit == "C" {
		eprintln!("Celsius temperatures cannot be lower than {CELSIUS_MIN}!\nYour input: {value}");
		return false;
	} else if value < KELVIN_MIN && unit == "K" {
		eprintln!("Kelvin temperatures cannot be lower than {KELVIN_MIN}!\nYour input: {value}");
		return false;
	}

	return true;
}

