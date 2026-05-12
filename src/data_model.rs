use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub applications: Vec<Application>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub command: String,
    pub args: Vec<String>,
    // Future-proofing: Optional fields for exact window placement
    pub workspace_id: Option<u32>,
    //pub geometry: Option<WindowGeometry>,
}

// Future-proofing: Coordinates and dimensions for window managers
// #[derive(Debug, Serialize, Deserialize)]
// pub struct WindowGeometry {
//     pub x: i32,
//     pub y: i32,
//     pub width: u32,
//     pub height: u32,
// }
