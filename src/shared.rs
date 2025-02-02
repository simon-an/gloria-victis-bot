#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BotMode {
    Disabled,
    Water,
    Wood,
    Iron,
    ManyIron,
    CraftLoop,
    Horse,
    SalvageItems,
}

impl BotMode {
    pub fn values() -> Vec<BotMode> {
        vec![
            BotMode::Disabled,
            BotMode::Water,
            BotMode::Wood,
            BotMode::Iron,
            BotMode::ManyIron,
            BotMode::CraftLoop,
            BotMode::Horse,
            BotMode::SalvageItems,
        ]
    }
    pub fn to_string(self) -> String {
        match self {
            Self::Disabled => "Disabled".to_string(),
            Self::Water => "Water".to_string(),
            Self::Wood => "Wood".to_string(),
            Self::Iron => "Iron".to_string(),
            Self::ManyIron => "ManyIron".to_string(),
            Self::CraftLoop => "CraftLoop".to_string(),
            Self::Horse => "RideHorse".to_string(),
            Self::SalvageItems => "SalvageItems".to_string(),
        }
    }
}

impl Default for BotMode {
    fn default() -> Self {
        BotMode::Disabled
    }
}
