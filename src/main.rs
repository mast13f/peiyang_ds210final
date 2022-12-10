//https://rust-ml.github.io/book/3_kmeans.html
//https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
//

mod some_func;
// use crate::some_func::kmean_cluster;
// use crate::some_func::draw;
// use crate::some_func::vec_to_arr;
use crate::some_func::*;

use csv;
use ndarray::{Array2, OwnedRepr};
use std::error::Error;
use serde::Deserialize;


// // We'll build our dataset on our own using ndarray and rand
use ndarray::prelude::*;
// Import the plotters crate to create the scatter plot
use plotters::prelude::*;


 #[derive(Debug, Deserialize)]


struct Stations {
    //X,Y,STATION,LINE,TERMINUS,ROUTE
    X: f32,
    Y: f32,
    NAME: String,

}


fn read_csv_file(path: &str) -> Result<Vec<(f32, f32)>, Box<dyn Error>>{
    // get the x and y coordination of stations
    // from cvs file to a vector
    let mut locations: Vec<(f32, f32)> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let record: Stations = result?;
        locations.push((record.X,record.Y));

    }
    Ok(locations)
}


fn main() {

    // data of mbta stations:
    if let Err(e) = read_csv_file("./MBTA_NODE.csv"){
        eprintln!("{}", e);
    }else {
        let _result = read_csv_file("./MBTA_NODE.csv");
    }
    let result = read_csv_file("./MBTA_NODE.csv");
    let data  = result.unwrap();
    let data_arr = vec_to_arr(data);

    let dataset = kmean_cluster(data_arr,100);
    draw("MBTA.png","MBTA K-Mean Cluster",dataset);




    // data of colleges:
    if let Err(e) = read_csv_file("./COLLEGES_PT.csv"){
        eprintln!("{}", e);
    }else {
        let _result_c = read_csv_file("./COLLEGES_PT.csv");
    }
    let result_c = read_csv_file("./COLLEGES_PT.csv");
    let data_c  = result_c.unwrap();
    let data_c_arr = vec_to_arr(data_c);

    let dataset_c = kmean_cluster(data_c_arr,100);
    draw("Colleges.png","Colleges K-Mean Cluster",dataset_c)





}



