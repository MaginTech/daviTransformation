use std::fs::File;
use std::io::Read;

use nalgebra::{Point3, Rotation3};

use serde_json;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DaviDesNode {
  name: String,
  id: i32,
  children: Option<Vec<DaviDesNode>>,

  #[serde(default)]
  rel_pos: Point3<f64>,
  #[serde(default)]
  rel_rot: Rotation3<f64>,
}

#[allow(dead_code)]
pub fn read_json_file(path: &str) -> Result<DaviDesNode, serde_json::Error> {
  let mut file = File::open(path).expect("Unable to open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");

  let des: DaviDesNode = serde_json::from_str(&contents)?;
  Ok(des)
}

#[cfg(test)]
mod tests {
  use super::*;
  use nalgebra::{Point3, Rotation3};

  #[test]
  fn test_read_json_file() {
    let test_file_path = "./ext/sample.json";

    match read_json_file(test_file_path) {
      Ok(tree) => {
        assert_eq!(tree.name, "origin");
        assert_eq!(tree.id, 0);
        assert_eq!(tree.rel_pos, Point3::<f64>::origin());
        assert_eq!(tree.rel_rot, Rotation3::<f64>::identity());
        match tree.children {
          Some(children) => {
            if let Some(child) = children.get(0){
              assert_eq!(child.name, "robot1");
              assert!(child.children.is_none());
            }
            if let Some(child) = children.get(1){
              assert_eq!(child.name, "robot2");
              assert!(child.children.is_none());
            }
          },
          None => {},
        }
      },
      Err(e) => panic!("Failed to parse JSON: {}", e),
    }
  }
}