use csv;
use ndarray::{Array2, OwnedRepr};
use std::error::Error;
use serde::Deserialize;
use linfa::prelude::*;
use linfa_clustering::KMeans;
use ndarray::prelude::*;
use rand::prelude::*;
use plotters::prelude::*;


pub fn vec_to_arr(vector: Vec<(f32,f32)>) -> Array2<f32>{
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

pub fn kmean_cluster(input_data: Array2<f32>, n_iter:usize ) -> DatasetBase<ArrayBase<OwnedRepr<f32>, Ix2>, Array1<usize>> {
    // kmean clustering, modified from Rust Machine Learning Book

    let dataset = DatasetBase::from(input_data);
    let rng = thread_rng(); // Random number generator
    let n_clusters = 5;
    let model = KMeans::params_with_rng(n_clusters, rng)
    .max_n_iterations(100)
    .tolerance(1e-5)
    .fit(&dataset)
    .expect("Error while fitting KMeans to the dataset");

    let dataset = model.predict(dataset);
    println!("{:?}", dataset.records);
    println!("{:?}", dataset.targets);
    dataset
}

pub fn draw(pngname: &str, title:&str, data:DatasetBase<ArrayBase<OwnedRepr<f32>, Ix2>, Array1<usize>>){
    // Visualization, modified from Rust Machine Learning Book

    let root = BitMapBackend::new(pngname, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_lim = 210000.0..260000.0f32;
    let y_lim = 870000.0..930000.0f32;

    let mut ctx = ChartBuilder::on(&root)
    .set_label_area_size(LabelAreaPosition::Left, 40) // Put in some margins
    .set_label_area_size(LabelAreaPosition::Right, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption(title, ("sans-serif", 25)) // Set a caption and font
    .build_cartesian_2d(x_lim, y_lim)
    .expect("Couldn't build our ChartBuilder");

    ctx.configure_mesh().draw().unwrap();
    let root_area = ctx.plotting_area();

    // check_array_for_plotting(dataset: &Array2<f32>) -> bool {}
    //check_array_for_plotting(&dataset.records); // Panics if that's not true


    for i in 0..data.records.shape()[0] {
    let coordinates = data.records.slice(s![i, 0..2]);

    let point = match data.targets[i] {
    0 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&RGBColor(0, 153, 153)).filled(),
    ),

    1 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&RGBColor(153, 0, 153)).filled(),
    ),

    2 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&RGBColor(102, 204, 0)).filled(),
    ),

    3 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&RGBColor(255, 51, 51)).filled(),
    ),

    4 => Circle::new(
    (coordinates[0], coordinates[1]),
    3,
    ShapeStyle::from(&RGBColor(210, 180, 0)).filled(),
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