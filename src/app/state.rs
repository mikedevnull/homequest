use crate::{app::config::Config, db::Database};

pub struct AppState {
    pub config: Config,
    pub db_pool: Database,
}
