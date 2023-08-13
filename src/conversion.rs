pub fn celsius_to_fahrenheit(degrees: f64) -> f64 {
	degrees * (9.0 / 5.0) + 32.0
}

pub fn celsius_to_kelvin(degrees: f64) -> f64 {
	degrees - 273.15
}

pub fn fahrenheit_to_celsius(degrees: f64) -> f64 {
	(degrees - 32.0) * (5.0 / 9.0)
}

pub fn fahrenheit_to_kelvin(degrees: f64) -> f64 {
	(degrees - 32.0) * (5.0 / 9.0) + 273.15
}

pub fn kelvin_to_celsius(degrees: f64) -> f64 {
	degrees + 273.15
}

pub fn kelvin_to_fahrenheit(degrees: f64) -> f64 {
	(degrees - 273.15) * (9.0 / 5.0) + 32.0
}
