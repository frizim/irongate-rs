use uuid::Uuid;

pub struct User {
    id: Uuid,
    name: String,
    creation_date: u64,
    last_login: u64
}