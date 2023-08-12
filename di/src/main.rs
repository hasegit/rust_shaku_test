// mod domain;

// use shaku::{module, Component, Interface, HasComponent};
// use std::sync::Arc;

// trait Logger: Interface {
//     fn log(&self, content: &str);
// }

// trait DateLogger: Interface {
//     fn log_date(&self);
// }

// #[derive(Component)]
// #[shaku(interface = Logger)]
// struct LoggerImpl;

// impl Logger for LoggerImpl {
//     fn log(&self, content: &str) {
//         println!("{}", content);
//     }
// }

// #[derive(Component)]
// #[shaku(interface = DateLogger)]
// struct DateLoggerImpl {
//     #[shaku(inject)]
//     logger: Arc<dyn Logger>,
//     #[shaku(default)]
//     today: String,
//     #[shaku(default)]
//     year: usize,
// }

// impl DateLogger for DateLoggerImpl {
//     fn log_date(&self) {
//         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//     }
// }

// module! {
//     MyModule {
//         components = [LoggerImpl, DateLoggerImpl],
//         providers = []
//     }
// }

use shaku::{module, HasComponent};
use di::infrastructure::api::UserRepositoryImpl;
use di::domain::api::{ApiConnector, ApiConnectorParameters};

module! {
    ApiModule {
        components = [UserRepositoryImpl],
        providers=[]
    }
}

module! {
    ApModule {
        components = [ApiConnector],
        providers=[]
    }
}


fn main() {
    let module = ApiModule::builder().build();

    let app = module.resolve_ref();
    app.find_user("POI".to_owned());

    let module = ApModule::builder().with_component_parameters::<ApiConnector>(ApiConnectorParameters {
        target_date: "2020-20-20".to_owned()
    })
    .build();

    let app = module.resolve_ref();
    app.get();

    // let date_logger: &dyn DateLogger = module.resolve_ref();
    // date_logger.log_date();
}
