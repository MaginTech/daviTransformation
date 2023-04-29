use serde:: Deserialize;
use std::fs::File;
use std::io::Read;

use nalgebra::Translation3;
use nalgebra::Rotation3;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DaviTreeNode {
  name: String,
  id: i32,
  position: Translation3<f64>,
  rotation: Rotation3<f64>,
  children: Option<Vec<DaviTreeNode>>
}


fn read_json_file(path: &str) -> Result<DaviTreeNode, serde_json::Error> {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let tree: DaviTreeNode = serde_json::from_str(&contents)?;
    Ok(tree)
}

// メイン処理
fn main() {
    let path = "./ext/sample.json";
    match read_json_file(path) {
        Ok(tree) => println!("Parsed JSON tree: {:?}", tree),
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
}