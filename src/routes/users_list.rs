use crate::api::error::APIError;
use crate::api::users::UserResponseItem;
use crate::api::{APIClient, Counted};
use crate::components::error::ErrorAlert;
use crate::components::pagination::PaginationComponent;
use crate::components::search::SearchBarComponent;
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, RouterAnchor};
use yew::prelude::*;
use yew::services::fetch::FetchTask;

const PAGE_SIZE: u32 = 10;

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
    pub on_loading: LoadingFunction,
}

pub enum Msg {
    PageChange(u32),
    SearchChange(Option<String>),
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
            Msg::Response(r) => {
                self.task = None;
                self.results = None;
                match r {
                    Ok(x) => {
                        self.results = Some(x);
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
            }
            Msg::SearchChange(s) => {
                self.search = s;
                self.reload();
            }
            Msg::PageChange(x) => {
                self.offset = x * PAGE_SIZE;
                self.reload();
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
        let page_change = self.link.callback(|x| Msg::PageChange(x));
        let page = self.offset / PAGE_SIZE;
        let total_pages = match &self.results {
            None => 1,
            Some(x) => (x.total as f32 / PAGE_SIZE as f32).ceil() as u32,
        };
        let search_change = self.link.callback(|s| Msg::SearchChange(s));
        html! {
        <>
            <h1 class="mb-3">{ "Users" } </h1>
            <RouterAnchor route=AppRoute::UsersCreate classes="btn btn-secondary">
                { "Create new user" }
            </RouterAnchor>
            <SearchBarComponent callback=search_change classes="w-50 mt-3"/>
            {
                if self.results.is_some() {
                    let items = &self.results.as_ref().unwrap().results;
                    if total_pages >= 1 {
                        html! {
                        <>
                            <PaginationComponent
                                total_pages=total_pages
                                current_page=page
                                callback=page_change
                            />
                            <table width="100%" class="table table-striped table-bordered">
                                <thead>
                                    <tr>
                                        <th>{"Name"}</th>
                                        <th>{"Email Address"}</th>
                                        <th></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { items.iter().map(render_item).collect::<Html>() }
                                </tbody>
                            </table>
                        </>
                        }
                    } else {
                        html! {
                            <div class="alert alert-info mt-3" role="alert">
                                 {"No results"}
                            </div>
                        }
                    }
                } else { html! {
                    <ErrorAlert<APIError> classes="mt-3" error=&self.error />
                }}
            }

        </>
        }
    }
}

fn render_item(x: &UserResponseItem) -> Html {
    html! {
        <tr>
            <td>{&x.name}</td>
            <td>{&x.email}</td>
            <td>
                <RouterAnchor route=AppRoute::UserEdit(x.id) classes="btn btn-secondary">
                    { "Edit" }
                </RouterAnchor>
            </td>
        </tr>
    }
}

impl ListUsersRoute {
    fn reload(&mut self) {
        self.task = Some(self.props.api_client.users_list(
            PAGE_SIZE,
            self.offset,
            self.search.clone(),
            self.props.on_loading.clone(),
            self.link.callback(Msg::Response),
        ));
    }
}
