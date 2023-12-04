use crate::database::connect::DB;

#[derive(Clone)]
pub struct AppState {
    pub db: DB,
}
