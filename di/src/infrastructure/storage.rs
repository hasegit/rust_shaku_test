use crate::domain::storage::{IUserRepository, UserRepository};

impl UserRepository {}

impl IUserRepository for UserRepository {
    fn update(&self, user_id: String) {
        println!("update {} data", user_id);
    }
}
