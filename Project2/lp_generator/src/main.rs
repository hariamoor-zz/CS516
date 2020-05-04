use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use serde_xml_rs::from_reader;
use std::error::Error;

mod resource;
use resource::Resource;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Project 2 - Solving Hard Compiler Problems",
    about = "Simplified implementation of Dr. Uli Kremer's RSDG algorithm with Gurobi"
)]
struct Opt {
    // Execution resource budget provided by user
    #[structopt(long)]
    budget: f64,

    // Path to input XML file
    #[structopt(long)]
    #[structopt(parse(from_os_str))]
    xml: PathBuf,

    // Path to output LP file
    #[structopt(long)]
    #[structopt(parse(from_os_str))]
    app: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    // println!("Command-line options are: {:?}", opt);

    let resource: Resource = from_reader(File::open(&opt.xml)?)?;
    println!("Resource is: {:#?}", resource);

    Ok(())
}
