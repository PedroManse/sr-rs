
#[derive(serde::Deserialize, Debug)]
pub struct FormAccount {
    pub name: String,
    pub password: String,
}

pub struct AccountRef(pub uuid::Uuid);

#[derive(serde::Serialize, serde::Deserialize, Hash, Clone, Debug)]
pub struct Account {
    pub name: String,
    pub id: uuid::Uuid,
}

