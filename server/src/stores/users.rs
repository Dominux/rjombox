use crate::errors::{AppError, AppResult};

pub type User = String;
pub type ID = u8;

const MAX_USERS: ID = 8;

pub struct UserStore {
    data: Vec<User>,
}

impl UserStore {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, user: User) -> AppResult<ID> {
        let id = self.data.len() as ID;

        if id == MAX_USERS {
            return Err(AppError::TooManyUsers);
        } else if self.data.contains(&user) {
            return Err(AppError::AlreadyExists(format!("User {}", user)));
        }

        self.data.push(user);

        Ok(id)
    }

    pub fn get_by_id(&self, id: ID) -> AppResult<&User> {
        self.data
            .get(Into::<usize>::into(id))
            .ok_or(AppError::DoesNotExist(format!("User with ID \"{}\"", id)))
    }
}
