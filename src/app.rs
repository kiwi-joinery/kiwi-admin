use crate::api::session::LoginResponse;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::components::auth::PersistedAuth;
use crate::routes::login::Login;
use crate::routes::AppRoute;
use core::cell::RefCell;
use std::rc::Rc;
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
    state: AppState,
}

pub struct State {
    pub api_client: APIClient,
    pub user: Option<UserResponseItem>,
}

pub type AppState = Rc<RefCell<State>>;

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
            state: Rc::new(RefCell::new(State {
                api_client: client,
                user: None,
            })),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeRoute(r) => {
                self.current_route = AppRoute::switch(r).unwrap();
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback_login = self.link.callback(Msg::LoggedIn);
        //let callback_logout = self.link.callback(|_| Msg::Logout);

        html! {
            <>
                //<Header current_user=&self.current_user/>
                {
                    // Routes to render sub components
                    match &self.current_route {
                        AppRoute::Login => html!{<Login callback=callback_login state=Rc::clone(&self.state) />},
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
