use crate::constants;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::cmp;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tamaowlchi {
    #[serde(default)]
    pub age: i32,
    #[serde(default)]
    pub is_dead: bool,
    #[serde(default)]
    pub is_awake: bool,
    #[serde(default)]
    pub is_ill: bool,
    #[serde(default)]
    pub energy: i32,
    #[serde(default)]
    pub health: i32,
    #[serde(default)]
    pub feed: i32,
    #[serde(default)]
    pub poops: i32,
    #[serde(default)]
    last_update: u64, // timestamp
}

impl Default for Tamaowlchi {
    fn default() -> Self {
        Tamaowlchi {
            age: 0,
            is_dead: false,
            is_awake: true,
            energy: constants::MAX_ENERGY,
            is_ill: false,
            health: constants::MAX_HEALTH,
            feed: constants::MAX_FOOD,
            poops: 0,
            last_update: now(),
        }
    }
}

impl Tamaowlchi {
    pub fn clean(&mut self) {
        self.poops = 0;
    }

    pub fn heal(&mut self) {
        self.is_ill = false;
        self.health = constants::MAX_HEALTH;
    }

    pub fn feed(&mut self) {
        self.feed = constants::MAX_FOOD;
    }

    pub fn wake_up(&mut self) {
        self.is_awake = true;
    }

    pub fn put_to_bed(&mut self) {
        self.is_awake = false;
    }

    pub fn sleep_status(&self) -> &str {
        if self.is_awake {
            "awake"
        } else {
            "asleep"
        }
    }

    pub fn new() -> Self {
        Tamaowlchi{..Default::default()}
    }

    pub fn update_stats (&mut self, hours_passed: i32) {
        if self.is_dead || hours_passed == 0 {
            return;
        }

        self.update_age(hours_passed);
        self.update_health(hours_passed);
        self.update_energy(hours_passed);
        self.update_feed(hours_passed);
        self.update_poops(hours_passed);
        self.last_update = now();
    }

    /// Tamaowlshi age icnrease by 1 every 24 hours.
    /// If it reach the maximum, it dies.
    pub fn update_age (&mut self, hours_passed: i32) {
        self.age = cmp::min(self.age + (hours_passed /24), constants::MAX_AGE);

        if self.age == constants::MAX_AGE {
            self.is_dead = true;
            println!("Oh no! Your tamaowlshi has died from age.");
        }
    }

    /// Tamaowlshi health reduce by 1 per hour when he is ill.
    /// If it reach 0, he dies.
    pub fn update_health(&mut self, hours_passed: i32) {
        if !self.is_ill {
            return;
        }

        self.health = cmp::max(self.health - hours_passed, 0);

        if self.health == 0 {
            self.is_dead = true;
            println!("Oh no! Your tamaowlshi has died from disease.");
        }
    }

    /// Tamaowlshi energy decrease by 1 per hour when he is awake
    /// and increase by 1 per hour when he is asleep.
    /// If it reach 0, he autmatically fell asleep and if it each the max,
    /// it automatically wake up.
    pub fn update_energy (&mut self, hours_passed: i32) {
        if self.is_awake {
            self.energy = cmp::max(self.energy - hours_passed, 0);
        } else {
            self.energy = cmp::min(self.energy + hours_passed, constants::MAX_ENERGY);
        }

        if self.is_awake && self.energy == 0 {
            self.is_awake = false;
            println!("Your tamaowlshi is too tired and fall asleep.")
        } else if !self.is_awake && self.energy == constants::MAX_ENERGY {
            self.is_awake = true;
            println!("Your tamaowlshi is waking up.")
        }
    }

    /// Tamaowlshi feed decrease by 1 per hours when he is awake
    /// and by 1 per 2 hours when he is asleep.
    /// if it reachs 0, he dies.
    pub fn update_feed (&mut self, hours_passed: i32) {
        if self.is_awake {
            self.feed = cmp::max(self.feed - hours_passed, 0);
        } else {
            self.feed = cmp::max(self.feed - (hours_passed / 2), 0);
        }

        if self.feed == 0 {
            self.is_dead = true;
            println!("Oh no! Your tamaowlshi has died from starvation!");
        }
    }

    /// Tamaowlshi poop increase by 1 each 12 hours.
    /// If tamaowlshi poop is at its max, tamaowlshi become ill.
    pub fn update_poops (&mut self, hours_passed: i32) {
        if !self.is_awake {
            return;
        }

        self.poops = cmp::min(self.poops + hours_passed / 12, constants::MAX_POOP);

        if self.poops == constants::MAX_POOP && !self.is_ill {
            self.is_ill = true;
            println!("Your tamaowlshi became ill due to a lack of hygiene.")
        }
    }
}

pub fn update_tamaowlchi_state(mut tamaowlchi: Tamaowlchi) -> Tamaowlchi {
    let current_time = now();
    let diff_time = current_time - tamaowlchi.last_update;
    let diff_hours = (diff_time / 3600) as i32;

    tamaowlchi.update_stats(diff_hours);

    tamaowlchi
}

fn now() -> u64 {
    let now = SystemTime::now();
    let duration = now.duration_since(SystemTime::UNIX_EPOCH).expect("Something weird happened with your system's time");
    duration.as_secs()
}
