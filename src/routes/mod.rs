pub mod dashboard;
pub mod forgot_password;
pub mod gallery_create;
pub mod gallery_edit;
pub mod gallery_list;
pub mod login;
pub mod not_found;
pub mod password_reset;
pub mod users_create;
pub mod users_edit;
pub mod users_list;

use serde::{Deserialize, Serialize};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;
use yew_router::switch::Permissive;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/forgot_password"]
    ForgotPassword,
    #[to = "/password_reset"]
    ResetPassword,
    #[to = "/gallery/create"]
    GalleryCreate,
    #[to = "/gallery/{id}"]
    GalleryEdit(u32),
    #[to = "/gallery"]
    Gallery,
    #[to = "/users/create"]
    UsersCreate,
    #[to = "/users/{id}"]
    UserEdit(u32),
    #[to = "/users"]
    Users,
    #[to = "/!"]
    Dashboard,
    #[to = "/{}"]
    NotFound(Permissive<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct HistoryState {
    pub redirect_on_login: Option<String>,
}

impl AppRoute {
    pub fn requires_auth(&self) -> bool {
        match &self {
            AppRoute::Login => false,
            AppRoute::ForgotPassword => false,
            AppRoute::ResetPassword => false,
            AppRoute::NotFound(_) => false,
            _ => true,
        }
    }
    pub fn redirect_if_logged_in(&self) -> bool {
        match &self {
            AppRoute::Login => true,
            AppRoute::ForgotPassword => true,
            AppRoute::ResetPassword => true,
            _ => false,
        }
    }
}

pub fn on_route_change(new_route: Route, is_authenticated: bool) {
    match AppRoute::switch(new_route.clone()) {
        None => {}
        Some(a) => {
            let mut dispatch = RouteAgentDispatcher::new();
            if a.requires_auth() && !is_authenticated {
                let mut redirect = Route::from(AppRoute::Login);
                redirect.state.redirect_on_login = Some(new_route.route);
                dispatch.send(RouteRequest::ReplaceRoute(redirect))
            } else if a.redirect_if_logged_in() && is_authenticated {
                let redirect = Route::from(AppRoute::Dashboard);
                dispatch.send(RouteRequest::ReplaceRoute(redirect))
            }
        }
    }
}

pub type State = HistoryState;
#[allow(dead_code)]
pub type Route = yew_router::route::Route<State>;
#[allow(dead_code)]
pub type RouteService = yew_router::service::RouteService<State>;
#[allow(dead_code)]
pub type RouteAgent = yew_router::agent::RouteAgent<State>;
#[allow(dead_code)]
pub type RouteAgentBridge = yew_router::agent::RouteAgentBridge<State>;
#[allow(dead_code)]
pub type RouteAgentDispatcher = yew_router::agent::RouteAgentDispatcher<State>;
#[allow(dead_code)]
pub type RouterAnchor = yew_router::components::RouterAnchor<AppRoute, State>;
#[allow(dead_code)]
pub type RouterButton = yew_router::components::RouterButton<AppRoute, State>;
#[allow(dead_code)]
pub type Router = yew_router::router::Router<AppRoute, State>;
