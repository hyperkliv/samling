use axum::Router;

use crate::state::AppState;

pub(crate) type AppRouter = Router<AppState>;
