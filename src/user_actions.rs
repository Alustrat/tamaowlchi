use crate::tamaowlchi::Tamaowlchi;
use crate::constants;


pub fn create_new_tamaowlchi() -> Tamaowlchi {
    let tamaowlchi = Tamaowlchi::new();

    println!("A new tamaowlchi is born! Hurray!");

    tamaowlchi
}

pub fn clean(tamaowlchi: &mut Tamaowlchi) {
    if tamaowlchi.poops == 0 {
        return println!("There is no poops to clean.")
    }

    tamaowlchi.clean();
    println!("All poops has been cleaned!")
}

pub fn feed(tamaowlchi: &mut Tamaowlchi) {
    if !tamaowlchi.is_awake {
        return println!("You can't feed your tamaowlchi while he is sleeping.")
    }

    if tamaowlchi.feed == constants::MAX_FOOD {
        return println!("Your tamaowlchi is not hungry.");
    }

    tamaowlchi.feed();
    println!("Your tamaowlchi is no longer hungry!")
}

pub fn heal(tamaowlchi: &mut Tamaowlchi) {
    if !tamaowlchi.is_awake {
        return println!("You can't heal your tamaowlchi while he is sleeping.")
    }

    if !tamaowlchi.is_ill && tamaowlchi.health == constants::MAX_HEALTH {
        return println!("Your tamaowlchi is already in top condition.");
    }

    tamaowlchi.heal();
    println!("Your tamaowlchi is not ill anymore and is in top condition!")
}

pub fn put_to_bed(tamaowlchi: &mut Tamaowlchi) {
    if tamaowlchi.energy == constants::MAX_ENERGY {
        return println!("Your tamaowlchi is not tired and doesn't want to go to bed.");
    }

    tamaowlchi.put_to_bed();
    println!("Your tamaowlchi fell asleep.")
}

pub fn wake_up(tamaowlchi: &mut Tamaowlchi) {
    if tamaowlchi.energy == 0 {
        return println!("Your tamaowlchi is too tired to be awake.");
    }

    tamaowlchi.wake_up();
    println!("Your tamaowlchi has woken up.")
}

pub fn status(tamaowlchi: &Tamaowlchi) {
    print_owl_ascii_art(tamaowlchi.age);
    println!("Your Tamaowlshi is {} years old and is currently {}.", tamaowlchi.age, tamaowlchi.sleep_status());

    if tamaowlchi.is_ill && tamaowlchi.health <= constants::HEALTH_WARNING_THRESHOLD {
        println!("Hurry up, he is ill and his health is really low!");
    } else if tamaowlchi.is_ill {
        println!("Be carrefull, he is ill.");
    } else {
        println!("He is in good shape.");
    }

    if tamaowlchi.feed <= constants::FOOD_WARNING_THRESHOLD {
        println!("He is a little hungry.");
    }

    if tamaowlchi.health <= constants::ENERGY_WARNING_THRESHOLD {
        println!("He will soon feel asleep.");
    }

    if tamaowlchi.poops >= constants::MAX_POOP {
        println!("He really needs to be clean or he will get sick!");
    } else if tamaowlchi.poops > 0 {
        println!("He needs to be clean.");
    }

    println!();
}

fn print_owl_ascii_art(age: i32) {
    if age == 100 {
        println!("{}", include_str!("./ascii_art/dead_owl.txt"));
    } else if age > 70 {
        println!("{}", include_str!("./ascii_art/old_owl.txt"));
    } else if age > 30 {
        println!("{}", include_str!("./ascii_art/midlife_owl.txt"));
    } else {
        println!("{}", include_str!("./ascii_art/baby_owl.txt"));
    }
}