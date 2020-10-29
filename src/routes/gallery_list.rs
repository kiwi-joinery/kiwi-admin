use crate::api::error::APIError;
use crate::api::gallery::GalleryItemResponse;
use crate::api::APIClient;
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, RouterAnchor};
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct ListGalleryRoute {
    props: Props,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    error: Option<APIError>,
    results: Option<Vec<GalleryItemResponse>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: LoadingFunction,
}

pub enum Msg {
    Response(Result<(), APIError>),
}

impl Component for ListGalleryRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            task: None,
            error: None,
            results: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(_r) => {
                self.task = None;
                self.results = None;
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
        html! {
        <>
            <h1 class="mb-3">{ "Gallery" } </h1>
            <RouterAnchor route=AppRoute::GalleryCreate classes="btn btn-secondary">
                { "Upload new image" }
            </RouterAnchor>
        </>
        }
    }
}
