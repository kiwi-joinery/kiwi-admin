use crate::api::session::LoginResponse;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::auth::PersistedAuth;
use crate::components::footer::Footer;
use crate::components::header::HeaderComponent;
use crate::components::loading::{LoadingComponent, LoadingProps};
use crate::routes::login::LoginRoute;
use crate::routes::{AppRoute, AppRouter};
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(debug_assertions)]
const API_URL: &str = "http://localhost:8001/api/";
#[cfg(not(debug_assertions))]
const API_URL: &str = "https://api.kiwijoinerydevon.co.uk";

pub struct App {
    link: ComponentLink<Self>,
    api_client: APIClient,
    loading: LoadingProps,
    current_user: Option<UserResponseItem>,
}

pub enum Msg {
    LoggedIn(LoginResponse),
    Logout,
    GlobalLoader(LoadingProps),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut client = APIClient::new(API_URL);
        PersistedAuth::load().map(|a| {
            client.add_auth_header(a.into());
        });
        Self {
            link,
            api_client: client,
            loading: LoadingProps::default(),
            current_user: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoggedIn(r) => {
                let auth = PersistedAuth::persist(r.user.id, r.token);
                self.api_client.add_auth_header(auth.into());
            }
            Msg::Logout => {
                PersistedAuth::remove();
                self.api_client.remove_auth_header();
            }
            Msg::GlobalLoader(p) => {
                self.loading = p;
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
                    is_signed_in=self.api_client.has_auth_header()
                    current_user=self.current_user.clone()
                    on_logout=self.link.callback(|_| Msg::Logout)
                    api_client=self.api_client.clone()
                />
                <LoadingComponent with loading_props/>
                <div>
                    <AppRouter
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::Login => html! {<LoginRoute
                                    on_loading=on_loading.clone()
                                    on_login=on_login.clone()
                                    api_client=api_client.clone()
                                />},
                                AppRoute::Login => html! { {"Login"} },
                                AppRoute::Dashboard => html! { {"Dashboard"} },
                                _ => html! {},
                            }
                        })
                    />
                </div>
                <Footer />
            </>
        }
    }
}
