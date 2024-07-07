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
    #[serde(other)]
    Unknown,
}

impl TaskQueueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskQueueType::Workflow => "WORKFLOW",
            TaskQueueType::Activity => "ACTIVITY",
            TaskQueueType::System => "SYSTEM",
            TaskQueueType::Background => "BACKGROUND",
            TaskQueueType::Unknown => "UNKNOWN",
        }
    }
}

impl From<String> for TaskQueueType {
    fn from(source: String) -> Self {
        TaskQueueType::from(source.as_str())
    }
}

impl From<&str> for TaskQueueType {
    fn from(source: &str) -> Self {
        match source {
            "WORKFLOW" => TaskQueueType::Workflow,
            "ACTIVITY" => TaskQueueType::Activity,
            "SYSTEM" => TaskQueueType::System,
            "BACKGROUND" => TaskQueueType::Background,
            _ => TaskQueueType::Unknown,
        }
    }
}
