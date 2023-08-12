use shaku::Component;
use shaku::Interface;

#[derive(Component)]
#[shaku(interface = IUserRepository)]
pub struct UserRepository {}

pub trait IUserRepository: Interface {
    fn update(&self, user_id: String);
}
