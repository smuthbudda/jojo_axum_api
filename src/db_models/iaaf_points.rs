#![allow(dead_code)]

use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Points {
    pub points: i16,
    pub gender: String,
    pub category: String,
    pub event: String,
    pub mark: f32,
    pub mark_time: String,
}

impl Points{
    
    pub fn new (points: i16, gender: String, category:String,
         event: String, mark: f32, mark_time: String) -> Self{
        Self{points, gender, category,
            event, mark, mark_time}
    }
}

pub fn read_from_file() -> Vec<Points>{
    println!("Reading json file.");
    let file_path = "data/WorldAthletics.json";
    let file = std::fs::File::open(file_path).expect("Could not open file");
    let points:Vec<Points> = serde_json::from_reader(file).expect("error reading from file");

    return points
}

pub enum Category{
    Indoor,
    Outdoor
}