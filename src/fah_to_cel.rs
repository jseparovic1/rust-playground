pub fn start(fahrenheit: f64) {
    let celsius = (fahrenheit - 32.0) / 1.8;

    println!("{}°F ~ {}°C", fahrenheit.ceil(), celsius.ceil())
}
