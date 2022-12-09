use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Stations {
    //X,Y,STATION,LINE,TERMINUS,ROUTE
    X: f64,
    Y: f64,
    STATION: String,
    LINE: String,
    TERMINUS: String,
    ROUTE: String,
}
fn read_from_file(path: &str) -> Result<Vec<(f64, f64)>, Box<dyn Error>>{
    let mut locations: Vec<(f64, f64)> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let record: Stations = result?;
        locations.push((record.X,record.Y));

    }

    Ok(locations)
}


fn main() {
    // if let Err(e) = read_from_file("./MBTA_NODE.csv"){
    //     eprintln!("{}", e);
    // }else {
    //     let mut result = read_from_file("./MBTA_NODE.csv");
    //
    // }
    let mut result = read_from_file("./MBTA_NODE.csv");
    let mut data  = result.unwrap();
    println!("{:?}",data)

}


// use polars::prelude::*;
// use std::fs::File;
//
// fn example() -> Result<DataFrame, E> {
//     let file = File::open("MBTA_NODE.csv").expect("could not open file");
//
//     CsvReader::new(file)
//             .infer_schema(None)
//             .has_header(true)
//             .finish()
// }

