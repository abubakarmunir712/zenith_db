pub enum ForeignKeyAction {
    SetNull,
    Restrict,
    Cascade,
}

impl ForeignKeyAction {
    pub fn to_oid(&self) -> u8 {
        match &self {
            ForeignKeyAction::Restrict => 0,
            ForeignKeyAction::SetNull => 1,
            ForeignKeyAction::Cascade => 2,
        }
    }

    pub fn from_oid(oid: u8) -> Self {
        match oid {
            0 => ForeignKeyAction::Restrict,
            1 => ForeignKeyAction::SetNull,
            2 => ForeignKeyAction::Cascade,
            _ => unreachable!(),
        }
    }
}
