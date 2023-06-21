use nalgebra::{Point3, Rotation3, Vector6};
use serde::{Deserialize};

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

  #[allow(dead_code)]
  pub fn add_child(&mut self, child: DaviTreeNode) {
    if let Some(children) = &mut self.children {
      children.push(child);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use nalgebra::{Point3, Rotation3, Vector6};

  #[test]
  #[test]
  fn test_add_child() {
    let mut d = DaviTreeNode::new("test", 0);
    let c = DaviTreeNode::new("child", 1);
    d.add_child(c);

    match d.children {
      Some(children) => {
        if let Some(child) = children.get(0) {
          assert_eq!(child.name, "child");
          assert_eq!(child.id, 1);
        }else{
        }
      }
      None => {
      }
    }
  }
}