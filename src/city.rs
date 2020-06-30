mod mean;

use mean::AccumulatingMean;
use num_format::{Locale::en, ToFormattedString};
use rand::{Rng, rngs::ThreadRng};
use super::interact;
use std::{cmp::Ordering, error::Error};

pub struct City {
    acres: u32,
    babies: u32,
    crop_yield: u32,
    dead_total: u32,
    died: u32,
    eaten_by_rats: u32,
    harvest: u32,
    pub land_price: u32,
    population: u32,
    sown: u32,
    starved_avg: AccumulatingMean,
    store: u32,
}

impl City {
    /// Create a new City with standard default values.
    pub fn new() -> City {
        City {
            acres: 1000,
            babies: 5,
            crop_yield: 3,
            dead_total: 0,
            died: 0,
            eaten_by_rats: 200,
            harvest: 3_000,
            land_price: 20,
            population: 95,
            sown: 0,
            starved_avg: AccumulatingMean::new(),
            store: 2_800,
        }
    }

    /// Did more than 45% of the population starve?
    pub fn famine(&self) -> bool {
        if self.died > (0.45 * self.population as f32) as u32 {
            interact::print_width(&format!(
                "\nYou starved {} people in one year!",
                self.died.to_formatted_string(&en)
            ));
            return true;
        }

        false
    }

    /// Decide the player's fate.
    pub fn decide_fate(&self, impeached: bool, rng: &mut ThreadRng) {

        let per_capita = self.acres as f32 / self.population as f32;
        let mean = self.starved_avg.mean();

        // Summary if not impeached early
        if !impeached {
            interact::print_width(&format!(
                "\nIn your 10-year term of office, {} percent of the \
                population starved per year on average, i.e. a total of {} \
                people died!\
                \nYou started with 10 acres per person and ended with {} acres \
                per person.",
                mean, self.dead_total.to_formatted_string(&en), per_capita
            ));
        }

        if impeached || mean > 33.0 || per_capita < 7.0 {
            interact::print_width("\nImpeachment! Due to exteme \
                mismanagement you have been impeached and thrown out of \
                office.");
        } else if mean > 10.0 || per_capita < 9.0 {
            interact::print_width("\nInfamy! Your heavy-handed \
                performance smacks of Nero and Ivan IV. The people (remaining) \
                find you an unpleasant ruler, and frankly, hate your guts!");
        } else if mean > 3.0 || per_capita < 10.0 {
            interact::print_width(&format!(
                "\nMediocrity! Your performance could have been somewhat \
                better, but really wasn't too bad at all. {} people would \
                dearly like to see you assassinated but we all have our \
                trivial problems.",
                (self.population as f32 * 0.8 * rng.gen::<f32>()).floor() as u32
            ));
        } else {
            interact::print_width("\nSuccess! A fantastic performance! \
                Charlemagne, Disraeli and Jefferson combined could not have \
                done better!");
        }
    }

    /// Feed the people with bushels of grain from the store.
    pub fn feed(&mut self) -> Result<(), Box<dyn Error>> {
        interact::print_width(&format!(
            "\nYour people ask for {} bushels of grain to feed themselves. How \
            many bushels do you wish to feed your people?",
            (self.population * 20).to_formatted_string(&en)
        ));
        loop {
            match interact::read_number()? {
                x if x > self.store => self.insufficient_bushels(),
                x => {
                    self.store -= x;
                    // How many people were fully fed? It takes 20 bushels to
                    // stop a person starving.
                    self.died = self.population - (x / 20);
                    return Ok(());
                },
            }
        }
    }

    /// Harvest grain from the fields and put it in the store.
    pub fn harvest(&mut self, rng: &mut ThreadRng) {
        self.crop_yield = rng.gen_range(1, 7);
        self.harvest = self.sown * self.crop_yield;
        self.store += self.harvest;
    }

    /// Complain if the player asked for more acres than are available.
    fn insufficient_acres(&self) {
        interact::print_width(&format!(
            "\nHamurusti: Think again. You own only {} acres. Now then...",
            self.acres.to_formatted_string(&en)
        ));
    }

    /// Complain if the player asked for more bushels than are available.
    fn insufficient_bushels(&self) {
        interact::print_width(&format!(
            "\nHamurusti: Think again. You have only {} bushels of grain. Now \
            then...", self.store.to_formatted_string(&en)
        ));
    }

    /// Roll the die on the chance of plague.
    pub fn plague(&mut self, rng: &mut ThreadRng) {
        if rng.gen_bool(0.15) {
            interact::print_width("\nBut then a horrible plague struck! \
                Half the people died.");
            self.population /= 2;
        }
    }

    /// Calculate babies made, avergae people starved and remove the dead from
    /// the population.
    pub fn populate(&mut self, rng: &mut ThreadRng) {
        // How many births and deaths?
        self.babies = rng.gen_range(1, 7) * (
            (20 * self.acres + self.store) / (100 * self.population)
        ) + 1;
        self.starved_avg.push(self.died as f32 / self.population as f32);
        self.dead_total += self.died;

        // Update population total
        self.population += self.babies;
        self.population -= self.died;
    }

    /// How much grain was lost to rats?
    pub fn rats(&mut self, rng: &mut ThreadRng) {
        self.eaten_by_rats = if rng.gen_bool(0.15) {
            (self.store as f32 * rng.gen_range(0.1, 0.3)) as u32
        } else {
            0
        };
        self.store -= self.eaten_by_rats;
    }

    /// Report to the player in detail about the state of the city.
    pub fn report(&self) {
        // Only report rats if they ate something.
        let rats_ate = match self.eaten_by_rats.cmp(&0) {
            Ordering::Greater => format!("\nRats ate {} bushels of grain.",
                self.eaten_by_rats.to_formatted_string(&en)),
            _ => String::from(""),
        };

        interact::print_width(&format!(
            "\nOur population is now {} people.\
            \nThe city now owns {} acres.\
            \nThe harvest was {} bushels of grain per acre.\
            {}\
            \nYou now have {} bushels of grain in store.\
            \nLand is trading at {} bushels of grain per acre.",
            self.population.to_formatted_string(&en), self.acres.to_formatted_string(&en),
            self.crop_yield, rats_ate,
            self.store.to_formatted_string(&en), self.land_price
        ));
    }

    /// Sell land. The player is paid in bushels of grain which go into the
    /// store.
    fn sell(&mut self) -> Result<(), Box<dyn Error>> {
        if self.acres == 0 {
            return Ok(());
        }
        loop {
            interact::print_width("\nHow many acres of land do you wish to sell?");

            match interact::read_number()? {
                x if x > self.acres => self.insufficient_acres(),
                x => {
                    self.acres -= x;
                    self.store += self.land_price * x;
                    return Ok(());
                },
            }
        }
    }

    /// Sow seeds onto the fields.
    pub fn sow(&mut self) -> Result<(), Box<dyn Error>> {
        interact::print_width(&"\nIt takes two bushels to plant an acre.");
        loop {
            interact::print_width(&"How many acres do you wish to sow \
                with seed?");

            match interact::read_number()? {
                0 => {
                    self.sown = 0;
                    return Ok(());
                },
                x if x > self.acres => self.insufficient_acres(),
                x if x / 2 >= self.store => self.insufficient_bushels(),
                x if x > 10 * self.population => {
                    interact::print_width(&format!(
                        "\nBut you have only {} people to tend the fields. \
                        Each person can sow ten acres. Now then...",
                        self.population.to_formatted_string(&en)
                    ));
                },
                x => {
                    self.store -= x / 2;
                    self.sown = x;
                    return Ok(());
                },
            }
        }
    }

    /// A yearly, high-level report to the player.
    pub fn summary(&self, year: u32) {
        interact::print_width(&format!(
            "\n\n    YEAR {}
            \nHamurusti: I beg to report to you, in year {}, {} people starved \
            and the population grew by {} people.",
            year, year, self.died.to_formatted_string(&en),
            self.babies.to_formatted_string(&en)
        ));
    }

    /// Purchase land. The player pays with bushels of grain from the store.
    pub fn trade(&mut self) -> Result<(), Box<dyn Error>> {
        if self.store == 0 {
            return self.sell();
        }
        loop {
            interact::print_width("\nHow many acres of land do you wish to buy?");

            match interact::read_number()? {
                0 => return self.sell(),
                x if self.land_price * x > self.store => self.insufficient_bushels(),
                x => {
                    self.store -= self.land_price * x;
                    self.acres += x;
                    return Ok(());
                },
            }
        }
    }
}
