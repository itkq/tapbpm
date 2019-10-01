extern crate chrono;
extern crate timer;

use std::io;
use std::time::Instant;
use std::process;

fn main() {
    println!("Tap any key to start calculating BPM.");

    let mut count: u32 = 0;
    let mut total_nanos: f64 = 0.0;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    loop {
        let timer = timer::Timer::new();
        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
            println!("\n\nbpm = {:.1} (sample count: {})", calc_bpm(total_nanos, count as f64), count);
            process::exit(0);
        });

        let start = Instant::now();
        io::stdin().read_line(&mut buffer).unwrap();
        let duration = start.elapsed();

        count += 1;
        total_nanos += duration.as_nanos() as f64 / (1000.0 * 1000.0 * 1000.0);
        println!("bpm = {:.1}", calc_bpm(total_nanos, count as f64));
    }
}

fn calc_bpm(total_nanos: f64, count: f64) -> f64 {
    let average_duration = total_nanos / count;
    1.0 / average_duration * 60.0
}
