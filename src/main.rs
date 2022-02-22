use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// A Random Number Generator (RNG) written in Rust
#[structopt(name = "Random Number Generator | ArtieFuzzz#8298 |")]
struct Args {
    #[structopt(short, long)]
    negative: bool,
    number: i32,
}

fn main() {
    let args = Args::from_args();

    if args.number.eq(&0) {
        return println!("Number cannot be 0");
    }

    let num = rand::thread_rng().gen_range(0..args.number);

    if args.negative {
        return println!("-{}", num);
    } else {
        return println!("{}", num);
    }
}
