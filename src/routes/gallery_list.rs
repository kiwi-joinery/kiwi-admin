use crate::api::error::APIError;
use crate::api::gallery::GalleryItemResponse;
use crate::api::APIClient;
use crate::bindings::sortable::{OnEndEvent, Sortable, SortableOptions};
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, RouterAnchor};
use wasm_bindgen::closure::Closure;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct ListGalleryRoute {
    props: Props,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    error: Option<APIError>,
    results: Option<Vec<GalleryItemResponse>>,
    on_end: Closure<dyn FnMut(OnEndEvent)>,
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
        let on_end = Closure::wrap(Box::new(move |event: OnEndEvent| {
            log::info!("{} {}", event.old_index(), event.new_index());
        }) as Box<dyn FnMut(OnEndEvent)>);
        Self {
            props,
            link,
            task: None,
            error: None,
            results: None,
            on_end,
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
            <ul id="items">
                <li>{"item 1"}</li>
                <li>{"item 2"}</li>
                <li>{"item 3"}</li>
            </ul>
        </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let element = document.get_element_by_id("items").unwrap();
        let options = SortableOptions::new();
        options.set_on_end(&self.on_end);
        Sortable::create(&element, options);
    }
}
