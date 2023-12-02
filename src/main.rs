use anyhow::anyhow;
use clap::Parser;

mod day1;
mod day2;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args: Arguments = Arguments::parse();

    let res = match args.day {
        1 => day1::calculate(),
        2 => day2::calculate(),
        _ => Err(anyhow!("illegal day")),
    };

    match res {
        Ok((part1, part2)) => println!("day {}\npart 1: {}\npart 2: {}", args.day, part1, part2),
        Err(err) => println!("{:?}", err),
    }
}
