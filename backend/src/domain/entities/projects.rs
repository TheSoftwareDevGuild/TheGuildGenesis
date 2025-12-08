use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::WalletAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(pub Uuid);

impl ProjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ProjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    #[serde(rename = "proposal")]
    Proposal,
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "rejected")]
    Rejected,
}

impl ProjectStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectStatus::Proposal => "proposal",
            ProjectStatus::Ongoing => "ongoing",
            ProjectStatus::Rejected => "rejected",
        }
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ProjectStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "proposal" => Ok(ProjectStatus::Proposal),
            "ongoing" => Ok(ProjectStatus::Ongoing),
            "rejected" => Ok(ProjectStatus::Rejected),
            _ => Err(format!("Invalid project status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub creator: WalletAddress,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: String,
        description: String,
        status: ProjectStatus,
        creator: WalletAddress,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: ProjectId::new(),
            name,
            description,
            status,
            creator,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }

        if self.name.len() > 255 {
            return Err("Project name cannot exceed 255 characters".to_string());
        }

        if self.description.trim().is_empty() {
            return Err("Project description cannot be empty".to_string());
        }

        Ok(())
    }

    pub fn update_info(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        status: Option<ProjectStatus>,
    ) {
        if let Some(n) = name {
            self.name = n;
        }
        if let Some(d) = description {
            self.description = d;
        }
        if let Some(s) = status {
            self.status = s;
        }
        self.updated_at = Utc::now();
    }

    pub fn change_status(&mut self, new_status: ProjectStatus) {
        self.status = new_status;
        self.updated_at = Utc::now();
    }
}