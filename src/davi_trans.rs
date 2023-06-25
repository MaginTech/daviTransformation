// use nalgebra::{Point3, Rotation3};
use std::cell::RefCell;
use std::rc::Rc;

use crate::davi_tree::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DaviTrans {
  tree: Option<Rc<RefCell<DaviTreeNode>>>
}

#[allow(dead_code)]
impl DaviTrans {
  pub fn new(tree: Rc<RefCell<DaviTreeNode>>) -> Self {
    DaviTrans { tree : Some(tree), }
  }

  // pub fn regist_node(&self, node: DaviTreeNode, parent_id: i32) {

  // }

  // pub fn request_position(&self, id: i32) -> Position3<f64>{

  // }

  // pub fn request_rotation(&self, id: i32) -> Rotation3<f64>{

  // }

  // pub fn request_relative_position(&self, id: i32, rel_id: i32) -> Position3<f64>{

  // }

  // pub fn request_relative_rotation(&self, id: i32, rel_id: i32) -> Rotation3<f64>{

  // }

  // pub fn update_position(&self, id: i32, pos: Position3<f64>){

  // }

  // pub fn update_rotation(&self, id: i32, rot: Rotation3<f64>){

  // }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_new() {
    let _davi = DaviTrans::new(DaviTreeNode::new("test", 0));
  }

  // #[test]
  // fn test_regist_node() {
  // }

  // #[test]
  // fn test_request_translation() {
  // }

  // #[test]
  // fn test_request_rotation() {
  // }

  // #[test]
  // fn test_update_rotation() {
  // }

  // #[test]
  // fn test_update_rotation() {
  // }
}