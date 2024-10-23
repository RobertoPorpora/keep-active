use enigo::{self, Coordinate, Mouse, Settings};
use std::thread;
use std::time;

use clap::Parser;
mod cli;

fn main() {
    let args = cli::Arguments::parse();

    let mut seconds = args.minutes * 60.0 - 30.0;

    if seconds < 1.0 {
        seconds = 1.0;
    }

    println!("Keeping system active every {} minutes.", seconds / 60.0);

    let mut enigo = match enigo::Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error while intializing library: {}", e);
            return;
        }
    };

    let mut old_location = enigo.location().unwrap();
    let mut timer = std::time::SystemTime::now();

    loop {
        let new_location = enigo.location().unwrap();
        let mouse_moved = old_location != new_location;
        old_location = new_location.clone();
        let keys_pressed = false; // TO DO: how to check keyboard activity?
        let inactive = !mouse_moved && !keys_pressed;

        if !inactive {
            timer = std::time::SystemTime::now();
        }

        let mut poke = inactive;
        let time_elapsed = match timer.elapsed() {
            Ok(t) => t.as_secs_f64(),
            Err(e) => {
                eprintln!("Error measuring time: {}", e);
                0.0
            }
        };
        if args.r#override > 0.0 {
            if time_elapsed > args.r#override {
                poke = false;
            }
        }

        if poke {
            match enigo.move_mouse(1, 1, Coordinate::Rel) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error while moving mouse: {}", e);
                }
            };

            match enigo.move_mouse(-1, -1, Coordinate::Rel) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error while moving mouse: {}", e);
                }
            };
        }

        thread::sleep(time::Duration::from_secs_f64(seconds));
    }
}
