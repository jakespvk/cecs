use std::time::Duration;
use std::thread::sleep;

const ONE_SECOND: Duration = Duration::new(1, 0);

fn main() {
    println!("beginning...");
    let mut i = 5;
    while i >= 1 {
        sleep(ONE_SECOND);
        println!("{}", i);
        i = i - 1;
    }
    sleep(ONE_SECOND);

}
