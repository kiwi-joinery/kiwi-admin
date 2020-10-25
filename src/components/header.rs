use crate::api::error::APIError;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, RouterAnchor};
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct HeaderComponent {
    props: Props,
    link: ComponentLink<Self>,
    logout_task: Option<FetchTask>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub is_signed_in: bool,
    pub current_user: Option<UserResponseItem>,
    pub on_logout: Callback<()>,
    pub on_loading: LoadingFunction,
    pub api_client: APIClient,
}

pub enum Msg {
    Logout,
    LogoutResult(Result<(), APIError>),
}

impl Component for HeaderComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            logout_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Logout => {
                self.logout_task = Some(self.props.api_client.session_logout(
                    self.props.on_loading.clone(),
                    self.link.callback(Msg::LogoutResult),
                ))
            }
            Msg::LogoutResult(_) => {
                self.props.on_logout.emit(());
            }
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
        let name = self
            .props
            .current_user
            .as_ref()
            .map(|x| x.name.as_str())
            .unwrap_or("Loading...");
        html! {
            <nav class="navbar navbar-light">
                <div class="container-fluid">
                    <RouterAnchor route=AppRoute::Dashboard classes="navbar-brand">
                        { "Kiwi Admin" }
                    </RouterAnchor>
                    {
                        if self.props.is_signed_in {
                            html!{
                                <ul class="nav">
                                    <li class="nav-item">
                                        <p class="nav-text">{name}</p>
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
