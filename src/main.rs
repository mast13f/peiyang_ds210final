
use csv;
use ndarray::Array2;
use std::error::Error;
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


fn read_from_file(path: &str) -> Result<Vec<(f32, f32,String)>, Box<dyn Error>>{
    // get the x and y coordination of stations
    // from cvs file to a vector
    let mut locations: Vec<(f32, f32, String)> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let record: Stations = result?;
        locations.push((record.X,record.Y,record.LINE));

    }
    Ok(locations)
}


fn vec_to_arr(vector: Vec<(f32,f32,String)>) -> Array2<f32>{
    // transfer vectors to 2d array in order to conduct k mean
    let mut result = Array2::<f32>::zeros((vector.len(), 3));
    for i in 0..vector.len(){
        for u in 0..3{
            if u == 0{
                result[[i,u]]= vector[i].0
            }else{
                result[[i,u]]= vector[i].1
            }
        }
    }
    result
}


fn main() {
    if let Err(e) = read_from_file("./MBTA_NODE.csv"){
        eprintln!("{}", e);
    }else {
        let _result = read_from_file("./MBTA_NODE.csv");
    }
    let result = read_from_file("./MBTA_NODE.csv");
    let data  = result.unwrap();
    let data_arr = vec_to_arr(data);
    println!("{:?}",data_arr);

}



