pub mod login;

use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/forgot_password"]
    ForgotPassword,
    #[to = "/reset_password"]
    ResetPassword,
    #[to = "/gallery"]
    Gallery,
    #[to = "/gallery/create"]
    GalleryCreate,
    #[to = "/users"]
    Users,
    #[to = "/users/@{id}"]
    User(i64),
    #[to = "/"]
    Dashboard,
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
