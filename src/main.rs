

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{Array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use std::error::Error;
use std::fs::File;

use std::io;
use std::process;

//use kmeans::*;
use serde::Deserialize;



#[derive(Debug, Deserialize)]
struct Stations {
    //X,Y,STATION,LINE,TERMINUS,ROUTE
    X: f32,
    Y: f32,
    STATION: String,
    LINE: String,
    TERMINUS: String,
    ROUTE: String,
}
fn read_from_file(path: &str) -> Result<Vec<(f32, f32)>, Box<dyn Error>>{
    let mut locations: Vec<(f32, f32)> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let record: Stations = result?;
        locations.push((record.X,record.Y));

    }

    Ok(locations)
}

// fn read_csv() -> Result<(Array2<f32>), Box<dyn Error>> {
//     // Our 2x3 test array
//
//     // Read an array back from the file
//     let file = File::open("MBTA_NODE.csv")?;
//     let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
//     let array_read: Array2<f32> = reader.deserialize_array2((164, 5))?;
//
//     Ok((array_read))
// }

fn vec_to_arr(vector: Vec<(f32,f32)>) -> Array2<f32>{
    let mut result = Array2::<f32>::zeros((vector.len(), 2));
    for i in 0..vector.len(){
        for u in 0..2{
            result[[i,u]]= vector[i].

        }
    }
    result
}




fn main() {
    if let Err(e) = read_from_file("./MBTA_NODE.csv"){
        eprintln!("{}", e);
    }else {
        let mut result = read_from_file("./MBTA_NODE.csv");

    }
    let mut result = read_from_file("./MBTA_NODE.csv");
    let mut data  = result.unwrap();
    let mut data_arr = vec_to_arr(data);
    println!("{:?}",data_arr);

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

