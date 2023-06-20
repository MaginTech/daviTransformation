use nalgebra::{Translation3, Rotation3};
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
  #[serde(default = "default_translation3")]
  pub position: Translation3<f64>,
  #[serde(default = "default_rotation3")]
  pub rotation: Rotation3<f64>,
  pub children: Option<Vec<DaviTreeNode>>
  // #[serde(skip_deserializing)]
}

pub fn default_translation3() -> Translation3<f64> {
  Translation3::identity()
}

pub fn default_rotation3() -> Rotation3<f64> {
  Rotation3::identity()
}
