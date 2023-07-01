use std::fs::File;
use std::io::Read;
use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Point3, Rotation3, Vector6};

use serde_json;
use serde::{Deserialize};

use crate::davi_tree::*;

#[derive(Debug, Clone, Deserialize)]
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
// convert_to_davitree converts a DaviDesNode to a DaviTreeNode
fn convert_to_davitree(s: DaviDesNode) -> Option<DaviTreeNode> {
  let children = match s.children {
    Some(child_nodes) => {
      let converted_children: Vec<Rc<RefCell<DaviTreeNode>>>
        = child_nodes.into_iter()
                      .filter_map(|child| convert_to_davitree(child))
                      .map(|node| Rc::new(RefCell::new(node))) 
                      .collect();
      Some(converted_children)
    },
    None => None,
  };
  
  Some(DaviTreeNode {
    name: s.name,
    id: s.id,
    children: children,
    rel_pos: s.rel_pos,
    rel_rot: s.rel_rot,
    vel_twist: Vector6::<f64>::zeros(),
    acc_twist: Vector6::<f64>::zeros()
  })
}

#[allow(dead_code)]
// read_json_file reads a json file and returns a DaviDesNode
fn read_json_file(path: &str) -> Result<DaviDesNode, serde_json::Error> {
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
  // test_convert_to_davitree tests the convert_to_davitree function
  fn test_convert_to_davitree() {
    let des = DaviDesNode{
      name: "origin".to_string(),
      id: 0,
      children: None,
      rel_pos: Point3::<f64>::origin(),
      rel_rot: Rotation3::<f64>::identity()
    };

    match convert_to_davitree(des){
      Some(tree) => {
        assert_eq!(tree.name, "origin");
        assert_eq!(tree.id, 0);
        assert_eq!(tree.rel_pos, Point3::<f64>::origin());
        assert_eq!(tree.rel_rot, Rotation3::<f64>::identity());
        assert!(tree.children.is_none());
      },
      None => {
        assert!(false);
      },
    }
  }

  #[test]
  // test_read_json_file tests the read_json_file function
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
          None => {
            assert!(false);
          },
        }
      },
      Err(e) => panic!("Failed to parse JSON: {}", e),
    }
  }
}