use nalgebra::{Point3, Rotation3, Vector6};
use serde::{Deserialize};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DaviTrans {
  root: DaviTreeNode
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DaviTreeNode {
  pub name: String,
  pub id: i32,
  pub children: Option<Vec<DaviTreeNode>>,

  #[serde(default)]
  pub position: Point3<f64>,
  #[serde(default)]
  pub rotation: Rotation3<f64>,

  #[serde(default, skip_deserializing)]
  pub vel_twist: Vector6<f64>,
  #[serde(default, skip_deserializing)]
  pub acc_twist: Vector6<f64>,
}

impl DaviTreeNode {
  #[allow(dead_code)]
  pub fn new(name: &str, id: i32) -> DaviTreeNode {
    DaviTreeNode {
      name: name.to_string(),
      id,
      children: Some(Vec::new()),
      position: Point3::<f64>::origin(),
      rotation: Rotation3::<f64>::identity(),
      vel_twist: Vector6::<f64>::zeros(),
      acc_twist: Vector6::<f64>::zeros()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use nalgebra::{Point3, Rotation3, Vector6};

  #[test]
  fn test_davi_tree_node_new() {
    let davi_node = DaviTreeNode::new("test", 0);

    assert_eq!(davi_node.name, "test");
    assert_eq!(davi_node.id, 0);
    assert!(davi_node.children.unwrap().is_empty());
    assert_eq!(davi_node.position, Point3::<f64>::origin());
    assert_eq!(davi_node.rotation, Rotation3::<f64>::identity());
    assert_eq!(davi_node.vel_twist, Vector6::<f64>::zeros());
    assert_eq!(davi_node.acc_twist, Vector6::<f64>::zeros());
  }
}