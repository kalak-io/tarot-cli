#[derive(Debug, Default, Clone, PartialEq)]
pub enum ChelemState {
    Announced,
    #[default]
    NotAnnounced,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChelemResult {
    AnnouncedAndLost,
    AnnouncedAndSucceed,
    NotAnnouncedAndSucceed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chelem {
    pub state: ChelemState,
    pub result: Option<ChelemResult>,
}
