use axum::extract::FromRef;

use crate::deadpool_postgres::Pool;
use crate::{db_migrations::Migrator, hashing::Hasher, CloudflareApi, Environment, JwtSigner};

#[derive(Debug, Clone, FromRef)]
pub(crate) struct AppState {
    pub(crate) environment: Environment,
    pub(crate) db_pool: Pool,
    pub(crate) migrator: Migrator,
    pub(crate) cloudflare_api: CloudflareApi,
    pub(crate) jwt_signer: JwtSigner,
    pub(crate) hasher: Hasher,
}
impl AppState {
    pub(crate) fn new(
        environment: Environment,
        db_pool: Pool,
        cloudflare_api: CloudflareApi,
        jwt_signer: JwtSigner,
        hasher: Hasher,
    ) -> Self {
        let migrator = Migrator::new(db_pool.clone());
        Self {
            environment,
            db_pool,
            migrator,
            cloudflare_api,
            jwt_signer,
            hasher,
        }
    }
}
