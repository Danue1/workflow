#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskQueueType {
    #[serde(rename = "WORKFLOW")]
    Workflow,
    #[serde(rename = "ACTIVITY")]
    Activity,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "BACKGROUND")]
    Background,
}

impl TaskQueueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskQueueType::Workflow => "WORKFLOW",
            TaskQueueType::Activity => "ACTIVITY",
            TaskQueueType::System => "SYSTEM",
            TaskQueueType::Background => "BACKGROUND",
        }
    }
}
