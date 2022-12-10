//https://rust-ml.github.io/book/3_kmeans.html
//https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
//


use csv;
use ndarray::Array2;
use std::error::Error;
use serde::Deserialize;


// Import the linfa prelude and KMeans algorithm
use linfa::prelude::*;
use linfa_clustering::KMeans;
// We'll build our dataset on our own using ndarray and rand
use ndarray::prelude::*;
use rand::prelude::*;
// Import the plotters crate to create the scatter plot
use plotters::prelude::*;


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


fn vec_to_arr(vector: Vec<(f32,f32)>) -> Array2<f32>{
    // transfer vectors to 2d array in order to conduct k mean
    let mut result = Array2::<f32>::zeros((vector.len(), 2));
    for i in 0..vector.len(){
        for u in 0..2{
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
    //println!("{:?}",data_arr);

    let dataset = DatasetBase::from(data_arr);
    let rng = thread_rng(); // Random number generator
    let n_clusters = 5;
    let model = KMeans::params_with_rng(n_clusters, rng)
    .max_n_iterations(300)
    .tolerance(1e-5)
    .fit(&dataset)
    .expect("Error while fitting KMeans to the dataset");

    let dataset = model.predict(dataset);
    println!("{:?}", dataset.records);
    println!("{:?}", dataset.targets);



    let root = BitMapBackend::new("../src/kmeans.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_lim = 230000.0..242000.0f32;
    let y_lim = 880000.0..910000.0f32;

    let mut ctx = ChartBuilder::on(&root)
    .set_label_area_size(LabelAreaPosition::Left, 40) // Put in some margins
    .set_label_area_size(LabelAreaPosition::Right, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("KMeans Demo", ("sans-serif", 25)) // Set a caption and font
    .build_cartesian_2d(x_lim, y_lim)
    .expect("Couldn't build our ChartBuilder");

    ctx.configure_mesh().draw().unwrap();
    let root_area = ctx.plotting_area();

    // check_array_for_plotting(dataset: &Array2<f32>) -> bool {}
    //check_array_for_plotting(&dataset.records); // Panics if that's not true


    for i in 0..dataset.records.shape()[0] {
    let coordinates = dataset.records.slice(s![i, 0..2]);

    let point = match dataset.targets[i] {
    0 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&RED).filled(),
    ),

    1 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&GREEN).filled(),
    ),

    2 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&BLUE).filled(),
    ),

    3 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&YELLOW).filled(),
    ),

    4 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&RGBColor(255, 69, 0)).filled(),
    ),

    // Making sure our pattern-matching is exhaustive
    _ => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&BLACK).filled(),
    ),
    };

    root_area
    .draw(&point)
    .expect("An error occurred while drawing the point!");
}
}



