use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(Uuid);

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
