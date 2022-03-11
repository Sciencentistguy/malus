use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut smc = macsmc::Smc::connect()?;

    loop {
        thread::sleep(Duration::from_secs(1));
        let w = smc.power_system_total()?;
        println!("Current power draw: {:.2}W", *w)
    }
}
