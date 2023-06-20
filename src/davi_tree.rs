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

pub fn default_rotation3() -> Rotation3<f64> {
  Rotation3::identity()
}
