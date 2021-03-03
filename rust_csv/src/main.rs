extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::io;
use std::process;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]

struct Record{
    city: String,
    state: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn run() -> Result<(), Box<Error>>{
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize(){
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
fn main(){
    if let Err(err) = run(){
        println!("{}", err);
        process::exit(1);
    }
}