use mysql::{Pool, PooledConn};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool>,
}

impl AppState {
    pub fn new(database_url: &str) -> Self {
        let pool = Pool::new(database_url).expect("Impossible de créer le pool MySQL");
        Self { pool: Arc::new(pool) }
    }

    /// Récupérer une connexion du pool
    pub fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().expect("Impossible de récupérer une connexion")
    }
}
