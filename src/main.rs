use rppal::gpio::Gpio;
use std::{thread, time::Duration};

fn main() {
    // GPIO pin number (BCM numbering, not physical pin number)
    let pin_number = 24; // GPIO17 (physical pin 11)
    
    // Initialize the GPIO
    let gpio = Gpio::new().expect("Failed to access GPIO");
    
    // Set the pin as input with internal pull-up
    let pin = gpio.get(pin_number).expect("Failed to get pin").into_input_pulldown();

    println!("Checking limit switch (NO)... Press to trigger.");

    loop {
        // Read pin state
        let is_pressed = pin.is_low(); // Because we use pull-up, LOW = pressed

        if is_pressed {
            println!("Limit switch is PRESSED!");
        } else {
            println!("Limit switch is NOT pressed.");
        }

        thread::sleep(Duration::from_millis(500));
    }
}
