mod user_route;
use axum::{ Router };

use crate::{ routes::{ v1::user_route::UserRoute }, state::AppState };

pub fn routes_v1() -> Router<AppState> {
    Router::new().merge(UserRoute::create_user_routes())
}
