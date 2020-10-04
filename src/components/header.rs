use crate::routes::AppRoute;
use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};
use yew_router::prelude::*;

pub struct HeaderComponent {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub signed_in: bool,
    pub logout: Callback<()>,
}

pub enum Msg {
    Logout,
}

impl Component for HeaderComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        HeaderComponent { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Logout => self.props.logout.emit(()),
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let logout = self.link.callback(|_: MouseEvent| Msg::Logout);
        html! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Dashboard classes="navbar-brand">
                        { "Kiwi Admin" }
                    </RouterAnchor<AppRoute>>
                    {
                        if self.props.signed_in {
                            html!{
                                <ul class="nav">
                                    <li class="nav-item">
                                        <p class="nav-link nav-text">{"User name"}</p>
                                    </li>
                                    <li class="nav-item">
                                        <button onclick=logout class="btn btn-light">{"Logout"}</button>
                                    </li>
                                </ul>
                            }
                        } else {
                            html!{}
                        }
                    }
                </div>
            </nav>

        }
    }
}
