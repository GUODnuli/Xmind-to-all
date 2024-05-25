use std::{io, fs};
use std::path::PathBuf;

use serde::{ Serialize, Deserialize };

#[derive(Clone, Deserialize, Debug)]
pub struct Sheet {
    pub id: String,
    pub class: Option<String>,
    #[serde(rename = "rootTopic")]
    pub root_topic: Topic,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Topic {
    id: String,
    pub markers: Option<Vec<Markers>>,
    class: Option<String>,
    pub title: String,
    #[serde(rename = "attributedTitle")]
    attributed_title: Option<Vec<AttributedTitle>>,
    #[serde(rename = "structreClass")]
    structre_class: Option<String>,
    pub children: Option<Children>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct AttributedTitle {
    text: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Children {
    pub attached: Vec<Topic>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Markers {
    #[serde(rename = "markerId")]
    pub marker_id: String,
}

pub fn get_sheet_json(file_path: &PathBuf) -> Result<Sheet, io::Error> {
    let contents_str = fs::read_to_string(file_path)?;

    let contents_data: Vec<Sheet> = serde_json::from_str(&contents_str)?;

    let result = match contents_data.get(0) {
        Some(x) => x.clone(),
        None => panic!("解析时发生不可解决的错误。"),
    };
    Ok(result)
}