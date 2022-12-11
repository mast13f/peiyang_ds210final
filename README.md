## K-Mean Redistricting Greater Boston Area MassGIS Data
DS210 Final Project   
Peiyang Liu

### Introduction
This project tries to redistrict the Greater Boston area using K-Mean clustering and compares the results conducted by different datasets. As a college student who takes MBTA to school every day, I found the data about public transportation and educational institutions in the Boston area valuable enough to do more research. 


### Data
- ```MBTA_NODE.csv```: X-coordinates, Y-coordinates, and names of MBTA rapid transit stops in the Greater Boston area
- ```MBTABUSSTOPS_PT.csv```: X-coordinates, Y-coordinates, and names of MBTA bus stops in the Greater Boston area
- ```COLLEGES_PT.csv```: X-coordinates, Y-coordinates, and names of all colleges in the Greater Boston area
- ```SCHOOLS_PT.csv```: X-coordinates, Y-coordinates, and names of all K12 educational facilities in the Greater Boston area

  *Notes:*   
  *- All data is retrieved from [MassGIS](https://www.mass.gov/info-details/massgis-data-layers), a Massachusetts state government geodata site*    
  *- The source file format is shapefile(.shp) and layer file(.lyr), using [MyGeodata](https://mygeodata.cloud/converter/shp-to-csv) to transfer to the (.csv) file we want.*

### Methodology
Based on [Rust Machine Learning Book](https://rust-ml.github.io/book/3_kmeans.html), this project first convert csv files to ```Vec<(f32,f32)>``` variables consisting of tuples, and then covert to ```Array2<f32>``` variables in order to perform K-Mean cluster using ```linfa-clustering```, and visualize the result using ```plotters```. 

https://rust-ml.github.io/book/3_kmeans.html
### Analysis
### Result
### Conclusion
### References
