use crate::api::error::APIError;
use crate::api::session::LoginResponse;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::auth::PersistedAuth;
use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;
use crate::components::loading::{LoadingComponent, LoadingProps};
use crate::components::sidebar::{SidebarActive, SidebarComponent};
use crate::loader_task::{LoadingFunction, LoadingTask, LoadingTaskConfig};
use crate::routes::dashboard::DashboardRoute;
use crate::routes::forgot_password::ForgotPasswordRoute;
use crate::routes::gallery_create::CreateGalleryItemRoute;
use crate::routes::gallery_edit::EditGalleryItemRoute;
use crate::routes::gallery_list::ListGalleryRoute;
use crate::routes::login::LoginRoute;
use crate::routes::not_found::NotFoundRoute;
use crate::routes::password_reset::PasswordResetRoute;
use crate::routes::users_create::CreateUserRoute;
use crate::routes::users_edit::EditUserRoute;
use crate::routes::users_list::ListUsersRoute;
use crate::routes::{on_route_change, AppRoute, Route, RouteAgentBridge, RouteService, Router};
use std::rc::Rc;
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
    UserResponse(Result<UserResponseItem, APIError>),
    RouteUpdated(Route),
    StartLoading(LoadingTaskConfig),
    StopLoading,
    UpdateLoadingText(Option<String>),
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
            Msg::UserResponse(res) => match res {
                Ok(u) => self.current_user = Some(u),
                Err(e) => log::error!("Couldn't load the current user: {}", e),
            },
            Msg::RouteUpdated(r) => {
                on_route_change(r, self.api_client.auth_header().is_some());
            }
            Msg::StartLoading(cfg) => {
                log::info!("Starting loading");
                if self.loading.active {
                    log::error!("Global loader is already active");
                }
                self.loading.active = true;
                self.loading.delay_full_appearance = cfg.get_delay_full_appearance();
                self.loading.text = None;
            }
            Msg::StopLoading => {
                log::info!("Stopping loading");
                self.loading.active = false;
            }
            Msg::UpdateLoadingText(x) => {
                self.loading.text = x;
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
        let link_clone = self.link.clone();
        let loading_function = LoadingFunction(Rc::new(move |cfg| {
            link_clone.send_message(Msg::StartLoading(cfg));
            Box::new(AppLoadingTask {
                link: link_clone.clone(),
            })
        }));
        let on_login = self.link.callback(|x| Msg::LoggedIn(x));
        html! {
            <>
                <HeaderComponent
                    on_loading=loading_function.clone()
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
                                on_loading=loading_function.clone()
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
                                    <ListUsersRoute
                                        on_loading=loading_function.clone()
                                        api_client=api_client.clone()
                                    />
                                </SidebarComponent>
                            },
                            AppRoute::UsersCreate => html! {
                                <SidebarComponent>
                                    <CreateUserRoute
                                        on_loading=loading_function.clone()
                                        api_client=api_client.clone()
                                    />
                                </SidebarComponent>
                            },
                            AppRoute::UserEdit(id) => html! {
                                <SidebarComponent>
                                    <EditUserRoute
                                        on_loading=loading_function.clone()
                                        api_client=api_client.clone()
                                        user_id=id
                                    />
                                </SidebarComponent>
                            },
                            AppRoute::Gallery => html! {
                                <SidebarComponent active=SidebarActive::Gallery>
                                    <ListGalleryRoute
                                        on_loading=loading_function.clone()
                                        api_client=api_client.clone()
                                    />
                                </SidebarComponent>
                            },
                            AppRoute::GalleryCreate => html! {
                                <SidebarComponent>
                                    <CreateGalleryItemRoute
                                        loader=loading_function.clone()
                                        api_client=api_client.clone()
                                    />
                                </SidebarComponent>
                            },
                            AppRoute::GalleryEdit(id) => html! {
                                <SidebarComponent>
                                    <EditGalleryItemRoute
                                        on_loading=loading_function.clone()
                                        api_client=api_client.clone()
                                        item_id=id
                                    />
                                </SidebarComponent>
                            },
                            AppRoute::ForgotPassword => html! {
                                <ForgotPasswordRoute
                                    on_loading=loading_function.clone()
                                    api_client=api_client.clone()
                                />
                            },
                            AppRoute::ResetPassword => html! {
                                <PasswordResetRoute
                                    on_loading=loading_function.clone()
                                    api_client=api_client.clone()
                                />
                            },
                            AppRoute::NotFound(_) => html! { <NotFoundRoute/> },
                        }
                    })
                />
                <FooterComponent/>
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
    fn load_user_task(&self, id: u32) -> FetchTask {
        self.api_client
            .users_get(id, None, self.link.callback(Msg::UserResponse))
    }
}

pub struct AppLoadingTask {
    link: ComponentLink<App>,
}

impl LoadingTask for AppLoadingTask {
    fn set_text(&self, x: Option<String>) {
        self.link.send_message(Msg::UpdateLoadingText(x));
    }
}

impl Drop for AppLoadingTask {
    fn drop(&mut self) {
        self.link.send_message(Msg::StopLoading);
    }
}
