use crate::api::session::LoginResponse;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::auth::PersistedAuth;
use crate::components::header::HeaderComponent;
use crate::components::loading::{LoadingComponent, LoadingProps};
use crate::routes::login::LoginRoute;
use crate::routes::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(debug_assertions)]
const API_URL: &str = "http://localhost:8001/api/";
#[cfg(not(debug_assertions))]
const API_URL: &str = "https://api.kiwijoinerydevon.co.uk";

pub struct App {
    link: ComponentLink<Self>,
    current_route: AppRoute,
    #[allow(unused)] // A component that owns this can send and receive messages from the agent.
    router_agent: Box<dyn Bridge<RouteAgent>>,
    api_client: APIClient,
    loading: LoadingProps,
    current_user: Option<UserResponseItem>,
}

pub enum Msg {
    ChangeRoute(Route),
    LoggedIn(LoginResponse),
    Logout,
    GlobalLoader(LoadingProps),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::ChangeRoute));
        let route: Route = RouteService::new().get_route();
        let mut client = APIClient::new(API_URL);
        PersistedAuth::load().map(|a| {
            client.add_auth_header(a.into());
        });
        Self {
            link,
            current_route: AppRoute::switch(route).unwrap(),
            router_agent,
            api_client: client,
            loading: LoadingProps::default(),
            current_user: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeRoute(r) => {
                self.current_route = AppRoute::switch(r).unwrap();
            }
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
        let on_loading = self.link.callback(|x| Msg::GlobalLoader(x));
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
                {
                    // Routes to render sub components
                    match &self.current_route {
                        AppRoute::Login => html!{<LoginRoute
                            on_loading=on_loading
                            on_login=self.link.callback(|x| Msg::LoggedIn(x))
                            api_client=self.api_client.clone()
                        />},
                        AppRoute::Dashboard => html!{ {"Dashboard"} },
                        // AppRoute::Editor(slug) => html!{<Editor slug=Some(slug.clone())/>},
                        // AppRoute::EditorCreate => html!{<Editor />},
                        // AppRoute::Article(slug) => html!{<Article slug=slug current_user=&self.current_user />},
                        // AppRoute::Settings => html!{<Settings callback=callback_logout />},
                        // AppRoute::ProfileFavorites(username) => html!{
                        //     <Profile username=username current_user=&self.current_user tab=ProfileTab::FavoritedBy />
                        // },
                        // AppRoute::Profile(username) => html!{
                        //     <Profile username=username current_user=&self.current_user tab=ProfileTab::ByAuthor />
                        // },
                        _ => html!{}
                    }
                }
                //<Footer />
            </>
        }
    }
}
