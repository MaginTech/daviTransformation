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
