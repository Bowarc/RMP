#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub enum Phase {
    Waiting,
    PreProcessing,
    Downloading { current_percentage: f32 },
    Postrocessing,
    Done,
    Error,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct Report {
    pub uuid: uuid::Uuid,
    pub url: String,
    pub phase: Phase,
}
