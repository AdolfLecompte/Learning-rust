fn main() {

    let c1 = 100.0;
    let f1 = celsius_a_fahrenheit(c1);
    println!("{c1:.2}°C = {f1:.2}°F");

    let f2 = 212.0;
    let c2 = fahrenheit_a_celsius(f2);
    println!("{f2:.2}°F = {c2:.2}°C");
}

fn celsius_a_fahrenheit(c: f64) -> f64{
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_a_celsius(f: f64) -> f64{
    (f - 32.0) * 5.0 / 9.0
}
