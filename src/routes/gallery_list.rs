use crate::api::error::APIError;
use crate::api::gallery::{Category, GalleryItemResponse, GalleryListResponse};
use crate::api::APIClient;
use crate::bindings::sortable::{OnEndEvent, Sortable, SortableOptions};
use crate::components::error::ErrorAlert;
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, RouterAnchor};
use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct ListGalleryRoute {
    props: Props,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    error: Option<APIError>,
    results: Option<GalleryListResponse>,
    on_ends: HashMap<Category, Closure<dyn FnMut(OnEndEvent)>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: LoadingFunction,
}

pub enum Msg {
    Response(Result<GalleryListResponse, APIError>),
}

impl Component for ListGalleryRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut on_ends = HashMap::new();
        for i in Category::into_enum_iter() {
            on_ends.insert(
                i.clone(),
                Closure::wrap(Box::new(move |e: OnEndEvent| {
                    if e.old_index() != e.new_index() {
                        log::info!("{} {} {}", &i.to_string(), e.old_index(), e.new_index());
                    }
                }) as Box<dyn FnMut(OnEndEvent)>),
            );
        }
        Self {
            props,
            link,
            task: None,
            error: None,
            results: None,
            on_ends,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(r) => {
                        self.error = None;
                        self.results = Some(r);
                    }
                    Err(e) => {
                        self.results = None;
                        self.error = Some(e)
                    }
                }
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
            <RouterAnchor route=AppRoute::GalleryCreate classes="btn btn-secondary mb-3">
                { "Upload new image" }
            </RouterAnchor>
            {
                if self.error.is_some() {
                    html!{<ErrorAlert<APIError> classes="mt-3" error=&self.error />}
                } else {
                    Category::into_enum_iter().map(|c| self.render_category(c)).collect::<Html>()
                }
            }
        </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.refresh();
        }
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        for i in Category::into_enum_iter() {
            let e = document.get_element_by_id(category_to_id(&i).as_str());
            match e {
                Some(e) => {
                    let options = SortableOptions::new();
                    options.set_on_end(&self.on_ends[&i]);
                    Sortable::create(&e, options);
                }
                None => {}
            }
        }
    }
}

impl ListGalleryRoute {
    fn refresh(&mut self) {
        self.task = Some(self.props.api_client.gallery_list(
            self.props.on_loading.clone(),
            self.link.callback(|x| Msg::Response(x)),
        ));
    }

    fn render_category(&self, category: Category) -> Html {
        let default = Vec::new();
        let items = self
            .results
            .as_ref()
            .and_then(|x| x.get(&category))
            .unwrap_or(&default);
        if items.len() > 0 {
            return html! {
                <div class="row">
                    <h4 class="col-12 p-2 mb-3 bg-light text-dark">{category.to_string()}</h4>
                    <div class="col card-deck" id={category_to_id(&category)}>
                        {items.iter().map(|i| self.render_item(i)).collect::<Html>()}
                    </div>
                </div>
            };
        } else {
            return html! {};
        }
    }

    fn render_item(&self, item: &GalleryItemResponse) -> Html {
        let image = item.best_matching_width(200);
        html! {
            <div class="card mb-3" style="min-width: 200px; max-width: 200px; height: 200px">
                {
                    match image {
                        Some(i) => {
                            html! { <img style="max-height: 115px; object-fit: contain" class="card-img-top" src=i.url/>}
                        }
                        None => { html! {} }
                    }
                }
                <div class="card-body p-2">
                    <p
                        class="card-text"
                        style="overflow: hidden; height: 100%;"
                    >{&item.description}</p>
                </div>
            </div>
        }
    }
}

fn category_to_id(category: &Category) -> String {
    format!("sortable-container-{}", category.serialize())
}
