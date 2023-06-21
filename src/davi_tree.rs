use nalgebra::{Point3, Rotation3, Vector6};
use std::collections::VecDeque;
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

  #[allow(dead_code)]
  pub fn search_by_id(&self, id: i32) -> Option<&DaviTreeNode> {
      let mut queue = VecDeque::new();
      queue.push_back(self);

      while let Some(current) = queue.pop_front() {
        if current.id == id {
          return Some(current);
        }

        if let Some(children) = &current.children {
          for child in children {
            queue.push_back(child);
          }
        }
      }

      None
  }

  #[allow(dead_code)]
  pub fn search_by_name(&self, name: &str) -> Option<&DaviTreeNode> {
    let mut queue = VecDeque::new();
    queue.push_back(self);

    while let Some(current) = queue.pop_front() {
      if current.name == name {
        return Some(current);
      }

      if let Some(children) = &current.children {
        for child in children {
          queue.push_back(child);
        }
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use nalgebra::{Point3, Rotation3, Vector6};

  #[test]
  fn test_new() {
    let d = DaviTreeNode::new("test", 0);

    assert_eq!(d.name, "test");
    assert_eq!(d.id, 0);
    assert!(d.children.unwrap().is_empty());
    assert_eq!(d.position, Point3::<f64>::origin());
    assert_eq!(d.rotation, Rotation3::<f64>::identity());
    assert_eq!(d.vel_twist, Vector6::<f64>::zeros());
    assert_eq!(d.acc_twist, Vector6::<f64>::zeros());
  }

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

  #[test]
  fn test_search_by_id() {
    let mut d = DaviTreeNode::new("test", 0);
    let c = DaviTreeNode::new("child", 1);
    d.add_child(c);

    match d.search_by_id(1) {
      Some(node) => {
        assert_eq!(node.name, "child");
        assert_eq!(node.id, 1);
      }
      None => {
      }
    }
  }

  #[test]
  fn test_search_by_name() {
    let mut d = DaviTreeNode::new("test", 0);
    let c = DaviTreeNode::new("child", 1);
    d.add_child(c);

    match d.search_by_name("child") {
      Some(node) => {
        assert_eq!(node.name, "child");
        assert_eq!(node.id, 1);
      }
      None => {
      }
    }
  }
}