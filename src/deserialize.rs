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

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn test_davi_tree_node_default_values() {
    let sample_json = json!({
      "name": "node",
      "id": 1,
      "children": []
    });

    let parsed_node: DaviTreeNode = serde_json::from_value(sample_json).unwrap();

    assert_eq!(parsed_node.name, "node");
    assert_eq!(parsed_node.id, 1);
    assert_eq!(parsed_node.position, default_translation3());
    assert_eq!(parsed_node.rotation, default_rotation3());
    assert!(parsed_node.quaternion.is_none());
    assert!(parsed_node.children.unwrap().is_empty());
  }

  #[test]
  fn test_read_json_file() {
    let test_file_path = "./test_data/sample.json";

    match read_json_file(test_file_path) {
      Ok(tree) => {
        assert_eq!(tree.name, "node");
        assert_eq!(tree.id, 1);
        assert_eq!(tree.position, default_translation3());
        assert_eq!(tree.rotation, default_rotation3());
        assert!(tree.quaternion.is_none());
        assert!(tree.children.unwrap().is_empty());
      }
      Err(e) => panic!("Failed to parse JSON: {}", e),
    }
  }
}