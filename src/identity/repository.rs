use uuid::Uuid;

use crate::common::Repository;

use super::User;

pub trait UserRepository: Repository<User, Uuid> {

}