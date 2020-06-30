mod city;
mod interact;

use city::City;
use rand::{Rng, thread_rng};
use std::{error::Error, io};
use textwrap::{fill, termwidth};

fn play() -> Result<(), Box<dyn Error>> {

    let mut rng = thread_rng();

    interact::print_width("\n    HAMURUSTI\n\
        \nTry your hand at governing Ancient Sumeria successfully for a \
        ten-year term of office.");

    let mut city = City::new();

    // Let's assume the player is going to win. Start out positive!
    let mut impeached = false;

    for year in 1 ..= 10 {

        city.summary(year);

        // Chance of plague after the first year
        if year > 1 {
            city.plague(&mut rng);
        }
        
        city.land_price = rng.gen_range(17, 27);
        city.report();
        city.feed()?;
        city.trade()?;
        city.sow()?;
        city.rats(&mut rng);
        city.harvest(&mut rng);

        if city.famine() {
            impeached = true;
            break
        }

        city.populate(&mut rng);
    }
    city.decide_fate(impeached, &mut rng);

    interact::print_width("\n\n\nSo long for now.");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        play()?;
        let text = "\nWould you like to play again? (y/n)";
        println!("{}", fill(text, termwidth()));

        let mut entry = String::new();
        io::stdin().read_line(&mut entry)?;
        match entry.trim().to_lowercase().as_str() {
            "y" => continue,
            _ => break,
        }
    }
    Ok(())
}
