use shaku::Component;
use crate::domain::api::{UserRepository, ApiConnector, Connector};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn find_user(&self, id: String) {
        println!("find {}", id)
    }
}

impl ApiConnector {}

impl Connector for ApiConnector {
    fn get(&self) {
        println!("{}",self.target_date);
    }
}

// #[derive(Component)]
// #[shaku(interface = Logger)]
// struct LoggerImpl;

// impl Logger for LoggerImpl {
//     fn log(&self, content: &str) {
//         println!("{}", content);
//     }
// }
