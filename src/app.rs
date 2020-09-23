use crate::routes::login::Login;
use crate::routes::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub(crate) struct App {
    link: ComponentLink<Self>,
    current_route: AppRoute,
    #[allow(unused)] // A component that owns this can send and receive messages from the agent.
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub(crate) enum Msg {
    ChangeRoute(Route),
    LoggedIn(bool),
    Logout,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::ChangeRoute));
        let route: Route = RouteService::new().get_route();
        Self {
            link,
            current_route: AppRoute::switch(route).unwrap(),
            router_agent,
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
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
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
                        AppRoute::Login => html!{<Login callback=callback_login />},
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
