extern crate core;

use std::io;
use std::string::ToString;

use crate::conversion::*;

mod conversion;

// Declare the temperature minima
const CELSIUS_MIN: f64 = -273.15;
const FAHRENHEIT_MIN: f64 = -459.67;
const KELVIN_MIN: f64 = 0.0;

// Declare the temperature symbols
const CELSIUS_SYMBOL: &str = "C";
const FAHRENHEIT_SYMBOL: &str = "F";
const KELVIN_SYMBOL: &str = "K";


fn main() {
	greet_user();

	// Get the units used for conversion
	let initial_unit: String = get_unit("initial".to_string());
	let target_unit: String = get_unit("target".to_string());

	// Create storage for input and converted degrees
	let mut temp_initial: f64;
	let mut temp_converted: f64;

	// Let the user convert as many temperatures as wanted
	loop {
		// Prompt user for degree value to convert from
		temp_initial = get_initial_temp(initial_unit.to_string());

		// Calculate the converted degree value
		temp_converted = convert_temperature(initial_unit.to_string(), target_unit.to_string(), temp_initial);

		println!("{temp_initial:.2}{initial_unit} equates to {temp_converted:.2}{target_unit}\n");
	}
}

// Used for choosing right method for converting two temperatures
fn convert_temperature(initial_unit: String, target_unit: String, temp_initial: f64) -> f64 {
	// Filter by initial unit
	return match initial_unit.as_str() {
		CELSIUS_SYMBOL => {
			// Filter by initial unit
			match target_unit.as_str() {
				FAHRENHEIT_SYMBOL => celsius_to_fahrenheit(temp_initial),
				KELVIN_SYMBOL => celsius_to_kelvin(temp_initial),

				// Initial and target unit have to be alike
				_ => temp_initial,
			}
		}
		FAHRENHEIT_SYMBOL => {
			// Filter by initial unit
			match target_unit.as_str() {
				CELSIUS_SYMBOL => fahrenheit_to_celsius(temp_initial),
				KELVIN_SYMBOL => fahrenheit_to_kelvin(temp_initial),

				// Initial and target unit have to be alike
				_ => temp_initial,
			}
		}
		KELVIN_SYMBOL => {
			// Filter by initial unit
			match target_unit.as_str() {
				CELSIUS_SYMBOL => kelvin_to_celsius(temp_initial),
				FAHRENHEIT_SYMBOL => kelvin_to_fahrenheit(temp_initial),

				// Initial and target unit have to be alike
				_ => temp_initial,
			}
		}
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


// This function prompts the user for temperature unit and validates input
fn get_unit(input: String) -> String {
	// Temp variable for user input
	let mut unit: String = String::new();

	// Continue prompting until a valid input is received
	while invalid_unit(unit.to_string().trim().to_lowercase()) {
		println!("Please select the {} unit:\n[C] Celsius, [F] Fahrenheit, [K] Kelvin", input);

		// As input gets appended, the string has to be cleared
		unit.clear();

		// Fill unit with user input
		io::stdin()
			.read_line(&mut unit)
			.expect("Failed to read line")
			.to_string();
	}

	// Returns normed input
	return get_unit_symbol(unit.to_string().trim().to_lowercase());
}

// Used for extracting the unit symbol out of the users input
fn get_unit_symbol(input: String) -> String {
	// Check for contained unit names and starting letters
	if input.contains("celsius") || input.starts_with("c") {
		return CELSIUS_SYMBOL.to_string();
	} else if input.contains("kelvin") || input.starts_with("k") {
		return KELVIN_SYMBOL.to_string();
	} else if input.contains("fahrenheit") || input.starts_with("f") {
		return FAHRENHEIT_SYMBOL.to_string();
	}

	return "".to_string();
}

// Determines whether a given string is a valid unit input
fn invalid_unit(input: String) -> bool {
	// Check for contained unit names and starting letters and informs user of successful selection
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

// This function lets the user type in the selected degree value to convert from
fn get_initial_temp(unit: String) -> f64 {
	// Create temp for storing user input
	let mut input: String = String::new();

	// Run until a valid input is received
	'getting_degrees: loop {
		println!("Please input degrees {}:", if unit == "F" { "Fahrenheit".to_string() } else if unit == "K" { "Kelvin".to_string() } else if unit == "C" {
			"Celsius".to_string()
		} else { "".to_string() });

		// As input gets appended, the string has to be cleared
		input.clear();

		// Fill input with user input
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line")
			.to_string();

		// Parse the input to a float
		let value: f64 = input.trim().replace(",", ".").parse().unwrap();

		// Check degrees for validity
		if valid_degree_value(unit.to_string(), value) {
			break 'getting_degrees;
		}
	}

	// Return parsed and cleaned input
	return input.trim().replace(",", ".").parse().unwrap();
}

// Used for validating degree values
fn valid_degree_value(unit: String, value: f64) -> bool {
	// No temperatures below absolute zero are possible - inform user if wrongly typed
	if value < FAHRENHEIT_MIN && unit == "F" {
		eprintln!("Fahrenheit temperatures cannot be lower than {FAHRENHEIT_MIN}!\nYour input: {value:.2}\n");
		return false;
	} else if value < CELSIUS_MIN && unit == "C" {
		eprintln!("Celsius temperatures cannot be lower than {CELSIUS_MIN}!\nYour input: {value:.2}\n");
		return false;
	} else if value < KELVIN_MIN && unit == "K" {
		eprintln!("Kelvin temperatures cannot be lower than {KELVIN_MIN}!\nYour input: {value:.2}\n");
		return false;
	}

	return true;
}

