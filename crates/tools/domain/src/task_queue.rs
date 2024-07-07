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

impl std::str::FromStr for TaskQueueType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WORKFLOW" => Ok(TaskQueueType::Workflow),
            "ACTIVITY" => Ok(TaskQueueType::Activity),
            "SYSTEM" => Ok(TaskQueueType::System),
            "BACKGROUND" => Ok(TaskQueueType::Background),
            _ => Err(()),
        }
    }
}
