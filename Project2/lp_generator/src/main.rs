use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Project 2 - Solving Hard Compiler Problems",
    about = "Simplified implementation of Dr. Uli Kremer's RSDG algorithm with Gurobi"
)]
struct Opt {
    // Execution resource budget provided by user
    #[structopt(long)]
    budget: f32,

    // Path to input XML file
    #[structopt(long)]
    #[structopt(parse(from_os_str))]
    xml: PathBuf,

    // Path to output LP file
    #[structopt(long)]
    #[structopt(parse(from_os_str))]
    app: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
