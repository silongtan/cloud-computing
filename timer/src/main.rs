use tokio::time;
use std::io;
use indicatif::ProgressBar;

async fn counter(time: i32) {
    println!("{}", time);
}

#[tokio::main]
async fn main() {

    println!("Progress Bar Timer? Y/N:");
    let mut p = String::new();
    io::stdin()
        .read_line(&mut p)
        .expect("failed to read from stdin");
    let mut progress: bool = false;
    if p.trim() == "Y" {
        progress = true;
    }

    println!("Enter the number of seconds for the timer: ");
    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("failed to read from stdin");

    let s: i32 = second.trim().parse().expect("Input not an integer");

    println!("Timer set ===> {}s", s);

    let mut interval = time::interval(time::Duration::from_secs(1));
    if progress {
        let bar = ProgressBar::new(s.try_into().unwrap());
        for _ in 0..s {
            interval.tick().await;
            bar.inc(1);
            // ...
        }
        bar.finish();
    } else {
        let mut interval = time::interval(time::Duration::from_secs(1));
        for _i in 0..s {
            interval.tick().await;
            counter(s-_i).await;
        }
    }
    
    println!("Timer finished");
}