use std::fs::File;
use std::io::Read;

use serde::{Deserialize};
use serde_json;

use nalgebra::{Translation3, Rotation3, Quaternion};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DaviTreeNode {
  name: String,
  id: i32,
  #[serde(default = "default_translation3")]
  position: Translation3<f64>,
  #[serde(default = "default_rotation3")]
  rotation: Rotation3<f64>,
  quaternion: Option<Quaternion<f64>>,
  children: Option<Vec<DaviTreeNode>>
}

fn default_translation3() -> Translation3<f64> {
  Translation3::identity()
}

fn default_rotation3() -> Rotation3<f64> {
  Rotation3::identity()
}

pub fn read_json_file(path: &str) -> Result<DaviTreeNode, serde_json::Error> {
  let mut file = File::open(path).expect("Unable to open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");

  let tree: DaviTreeNode = serde_json::from_str(&contents)?;
  Ok(tree)
}
