use crate::api::session::LoginResponse;
use crate::api::APIClient;
use crate::auth::PersistedAuth;
use crate::components::loading::LoadingComponent;
use crate::routes::login::Login;
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
    client: APIClient,
}

pub enum Msg {
    ChangeRoute(Route),
    LoggedIn(LoginResponse),
    Logout,
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
            client,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeRoute(r) => {
                self.current_route = AppRoute::switch(r).unwrap();
            }
            Msg::LoggedIn(r) => {
                let auth = PersistedAuth::persist(r.user.id, r.token);
                self.client.add_auth_header(auth.into());
            }
            Msg::Logout => {
                PersistedAuth::remove();
                self.client.remove_auth_header();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                //<Header current_user=&self.current_user/>
                <LoadingComponent active=false text=Option::<String>::None/>
                {
                    // Routes to render sub components
                    match &self.current_route {
                        AppRoute::Login => html!{<Login callback=self.link.callback(Msg::LoggedIn) client=self.client.clone() />},
                        // AppRoute::Register => html!{<Register callback=callback_register />},
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
