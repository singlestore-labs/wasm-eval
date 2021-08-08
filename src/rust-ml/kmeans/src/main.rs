use std::str::FromStr;
use std::str::from_utf8;
use ndarray::Array2;

fn main() {
    fit(b"data/iris.data.csv", 2, 2);
}

pub fn fit (csv_content: &[u8], dim: i32, num_clusters: usize) {
    let data = read_data(csv_content, dim as usize);
    let (_means, _clusters) = rkm::kmeans_lloyd(&data.view(), num_clusters as usize);

    // let mut serialized_vec = Vec::new();
    // for row in means.genrows() {
    //   serialized_vec.push(row[0]);
    //   serialized_vec.push(row[1]);
    // }
    // return serialized_vec;
}

fn read_data(csv_content: &[u8], dim: usize) ->  Array2<f32> {
    let mut data_reader = csv::Reader::from_path(from_utf8(&csv_content).unwrap()).unwrap();
    let mut data: Vec<f32> = Vec::new();
    for record in data_reader.records() {
        for field in record.unwrap().iter() {
            let value = f32::from_str(field);
            data.push(value.unwrap());
        }
    }
    Array2::from_shape_vec((data.len() / dim, dim), data).unwrap()
}
