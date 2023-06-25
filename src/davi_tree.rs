// use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;
use nalgebra::{Point3, Rotation3, Vector6};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DaviTreeNode {
  pub name: String,
  pub id: i32,
  pub children: Option<Vec<Rc<RefCell<DaviTreeNode>>>>,

  pub position: Point3<f64>,
  pub rotation: Rotation3<f64>,

  pub vel_twist: Vector6<f64>,
  pub acc_twist: Vector6<f64>,
}

impl DaviTreeNode {
  #[allow(dead_code)]
  pub fn new(name: &str, id: i32) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(DaviTreeNode {
      name: name.to_string(),
      id,
      children: None,
      position: Point3::<f64>::origin(),
      rotation: Rotation3::<f64>::identity(),
      vel_twist: Vector6::<f64>::zeros(),
      acc_twist: Vector6::<f64>::zeros()
    }))
  }

  #[allow(dead_code)]
  fn add_child(&mut self, name: &str, id: i32) {
    let new_child = DaviTreeNode::new(name, id);
    match &mut self.children {
      Some(children) => {
        children.push(new_child);
      }
      None => {
        self.children = Some(vec![new_child]);
      }
    }
  }

  // #[allow(dead_code)]
  // pub fn search_by_id(&self, id: i32) -> Option<&DaviTreeNode> {
  //     let mut queue = VecDeque::new();
  //     queue.push_back(self);

  //     while let Some(current) = queue.pop_front() {
  //       if current.id == id {
  //         return Some(current);
  //       }

  //       if let Some(children) = &current.children {
  //         for child in children {
  //           queue.push_back(child);
  //         }
  //       }
  //     }

  //     None
  // }

  // #[allow(dead_code)]
  // pub fn search_by_name(&self, name: &str) -> Option<&DaviTreeNode> {
  //   let mut queue = VecDeque::new();
  //   queue.push_back(self);

  //   while let Some(current) = queue.pop_front() {
  //     if current.name == name {
  //       return Some(current);
  //     }

  //     if let Some(children) = &current.children {
  //       for child in children {
  //         queue.push_back(child);
  //       }
  //     }
  //   }

  //   None
  // }
}

#[cfg(test)]
mod tests {
  use super::*;
  use nalgebra::{Point3, Rotation3, Vector6};

  #[test]
  fn test_new() {
    let d = DaviTreeNode::new("test", 0);

    assert_eq!(d.borrow().name, "test");
    assert_eq!(d.borrow().id, 0);
    assert!(d.borrow().children.is_none());
    assert_eq!(d.borrow().position, Point3::<f64>::origin());
    assert_eq!(d.borrow().rotation, Rotation3::<f64>::identity());
    assert_eq!(d.borrow().vel_twist, Vector6::<f64>::zeros());
    assert_eq!(d.borrow().acc_twist, Vector6::<f64>::zeros());
  }

  #[test]
  fn test_add_child() {
    let d = DaviTreeNode::new("test", 0);
    let mut d_mut = d.borrow_mut();
    d_mut.add_child("child", 1);

    match &d_mut.children {
      Some(children) => {
        if let Some(child) = children.get(0) {
          assert_eq!(child.borrow().name, "child");
          assert_eq!(child.borrow().id, 1);
        }else{
        }
      }
      None => {
      }
    }
  }

  // #[test]
  // fn test_search_by_id() {
  //   let mut d = DaviTreeNode::new("test", 0);
  //   let c = DaviTreeNode::new("child", 1);
  //   d.add_child(c);

  //   match d.search_by_id(1) {
  //     Some(node) => {
  //       assert_eq!(node.name, "child");
  //       assert_eq!(node.id, 1);
  //     }
  //     None => {
  //     }
  //   }
  // }

  // #[test]
  // fn test_search_by_name() {
  //   let mut d = DaviTreeNode::new("test", 0);
  //   let c = DaviTreeNode::new("child", 1);
  //   d.add_child(c);

  //   match d.search_by_name("child") {
  //     Some(node) => {
  //       assert_eq!(node.name, "child");
  //       assert_eq!(node.id, 1);
  //     }
  //     None => {
  //     }
  //   }
  // }
}