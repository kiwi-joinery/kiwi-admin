use crate::api::error::APIError;
use crate::api::session::LoginResponse;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::auth::PersistedAuth;
use crate::components::footer::Footer;
use crate::components::header::HeaderComponent;
use crate::components::loading::{LoadingComponent, LoadingProps};
use crate::components::sidebar::{SidebarActive, SidebarComponent};
use crate::routes::dashboard::DashboardRoute;
use crate::routes::login::LoginRoute;
use crate::routes::not_found::NotFoundRoute;
use crate::routes::{on_route_change, AppRoute, Route, RouteAgentBridge, RouteService, Router};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::agent::RouteRequest;

#[cfg(debug_assertions)]
const API_URL: &str = "http://localhost:8001/api/";
#[cfg(not(debug_assertions))]
const API_URL: &str = "https://api.kiwijoinerydevon.co.uk";

pub struct App {
    link: ComponentLink<Self>,
    api_client: APIClient,
    loading: LoadingProps,
    current_user: Option<UserResponseItem>,
    current_user_task: Option<FetchTask>,
    router_agent: RouteAgentBridge,
    route_service: RouteService,
}

pub enum Msg {
    LoggedIn(LoginResponse),
    Logout,
    GlobalLoader(LoadingProps),
    UserResponse(Result<UserResponseItem, APIError>),
    RouteUpdated(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut client = APIClient::new(API_URL);
        let auth = PersistedAuth::load();
        let route_service = RouteService::new();
        on_route_change(route_service.get_route(), auth.is_some());
        match auth {
            None => {}
            Some(a) => client.set_auth_header(a.into()),
        }
        let router_agent = RouteAgentBridge::new(link.callback(Msg::RouteUpdated));
        Self {
            link,
            api_client: client,
            loading: LoadingProps::default(),
            current_user: None,
            current_user_task: None,
            router_agent,
            route_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoggedIn(r) => {
                let auth = PersistedAuth::persist(r.user.id, r.token);
                self.api_client.set_auth_header(auth.into());
                self.current_user_task = Some(self.load_user_task(r.user.id));
                let dest = match self.route_service.get_route().state.redirect_on_login {
                    None => Route::from(AppRoute::Dashboard),
                    Some(s) => Route::new_default_state(s),
                };
                self.router_agent.send(RouteRequest::ChangeRoute(dest));
            }
            Msg::Logout => {
                self.current_user_task = None;
                self.current_user = None;
                PersistedAuth::remove();
                self.api_client.remove_auth_header();
                self.router_agent
                    .send(RouteRequest::ChangeRoute(Route::from(AppRoute::Login)));
            }
            Msg::GlobalLoader(p) => {
                self.loading = p;
            }
            Msg::UserResponse(res) => match res {
                Ok(u) => self.current_user = Some(u),
                Err(e) => log::error!("Couldn't load the current user: {}", e),
            },
            Msg::RouteUpdated(r) => {
                on_route_change(r, self.api_client.auth_header().is_some());
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let loading_props = self.loading.clone();
        let api_client = self.api_client.clone();
        let on_loading = self.link.callback(|x| Msg::GlobalLoader(x));
        let on_login = self.link.callback(|x| Msg::LoggedIn(x));
        html! {
            <>
                <HeaderComponent
                    on_loading=on_loading.clone()
                    is_signed_in=api_client.auth_header().is_some()
                    current_user=self.current_user.clone()
                    on_logout=self.link.callback(|_| Msg::Logout)
                    api_client=api_client.clone()
                />
                <LoadingComponent with loading_props/>
                <Router
                    render = Router::render(move |switch: AppRoute| {
                        match switch {
                            AppRoute::Login => html! {<LoginRoute
                                on_loading=on_loading.clone()
                                on_login=on_login.clone()
                                api_client=api_client.clone()
                            />},
                            AppRoute::Dashboard => html! {
                                <SidebarComponent active=SidebarActive::Dashboard>
                                    <DashboardRoute/>
                                </SidebarComponent>
                            },
                            AppRoute::Users => html! {
                                <SidebarComponent active=SidebarActive::Users>
                                    <p>{"Users"}</p>
                                </SidebarComponent>
                            },
                            AppRoute::Gallery => html! {
                                <SidebarComponent active=SidebarActive::Gallery>
                                    <p>{"Gallery"}</p>
                                </SidebarComponent>
                            },
                            AppRoute::GalleryCreate => html! {
                                <SidebarComponent>
                                    <p>{"Gallery Create"}</p>
                                </SidebarComponent>
                            },
                            AppRoute::NotFound(_) => html! { <NotFoundRoute/> },
                            _ => html! {},
                        }
                    })
                />
                <Footer />
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            match self.api_client.auth_header() {
                None => {}
                Some(a) => {
                    self.current_user_task =
                        Some(self.load_user_task(a.0.username().parse().unwrap()))
                }
            }
        }
    }
}

impl App {
    fn load_user_task(&self, id: i32) -> FetchTask {
        self.api_client
            .users_get(id, None, self.link.callback(Msg::UserResponse))
    }
}
