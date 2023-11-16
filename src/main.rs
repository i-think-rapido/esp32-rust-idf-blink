use std::thread;
use std::time::Duration;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() 
{
  esp_idf_sys::link_patches();

  let mut led = builtin_led();
  let n = 1;

  while n == 1 {
    led.set_high().unwrap();
    thread::sleep(Duration::from_millis(100));

    led.set_low().unwrap();
    thread::sleep(Duration::from_millis(800));

    println!("blink");
  }
}

fn builtin_led() -> PinDriver<'static, Gpio2, Output>
{
  let peripherals = Peripherals::take().unwrap();
  PinDriver::output(peripherals.pins.gpio2).unwrap()
}