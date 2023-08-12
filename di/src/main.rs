use di::domain::{
    api::{ApiConnector, ApiConnectorParameters},
    storage::UserRepository,
};
use di::usecase::user::{IUserUpdater, UserUpdater};
use shaku::{module, HasComponent};

module! {
    InfrastructureModule {
        components = [UserRepository, ApiConnector, UserUpdater],
        providers=[]
    }
}

fn main() {
    let module = InfrastructureModule::builder()
        .with_component_parameters::<ApiConnector>(ApiConnectorParameters {
            target_user: "ojisan".to_owned(),
        })
        .build();

    let usecase: &dyn IUserUpdater = module.resolve_ref();
    usecase.update("hogeo".to_owned());
    usecase.update("john".to_owned());
}
