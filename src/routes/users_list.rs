use crate::api::error::APIError;
use crate::api::users::UserResponseItem;
use crate::api::{APIClient, Counted};
use crate::components::error::ErrorAlert;
use crate::components::loading::LoadingProps;
use crate::components::pagination::PaginationComponent;
use crate::routes::{AppRoute, Route, RouterAnchor};
use wasm_bindgen::JsValue;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew::services::fetch::FetchTask;

const FIELD_SEARCH: &str = "search";
const PAGE_SIZE: u32 = 20;

pub struct ListUsersRoute {
    props: Props,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    error: Option<APIError>,
    search: Option<String>,
    results: Option<Counted<UserResponseItem>>,
    offset: u32,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: Callback<LoadingProps>,
}

pub enum Msg {
    PageChange(u32),
    SearchChange(String),
    Response(Result<Counted<UserResponseItem>, APIError>),
}

impl Component for ListUsersRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = props.api_client.users_list(
            PAGE_SIZE,
            0,
            None,
            props.on_loading.clone(),
            link.callback(Msg::Response),
        );
        Self {
            props,
            link,
            task: Some(task),
            error: None,
            search: None,
            results: None,
            offset: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SearchChange(_) => {}
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(x) => {
                        self.results = Some(x);
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
            }
            Msg::PageChange(x) => {
                self.offset = x;
            }
        }
        true
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
        let _oninput_search = self
            .link
            .callback(|ev: InputData| Msg::SearchChange(ev.value));
        let page_change = self.link.callback(|x| Msg::PageChange(x));
        html! {
        <>
            <h1>{ "Users" } </h1>
            <RouterAnchor route=AppRoute::UsersCreate classes="btn btn-secondary">
                { "Create new user" }
            </RouterAnchor>
            <PaginationComponent
                total_pages=2
                current_page=self.offset
                callback=page_change
            />
        </>
        }
    }
}
