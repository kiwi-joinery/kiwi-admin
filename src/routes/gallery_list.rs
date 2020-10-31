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
    do_refresh: bool, // Hack to force yew to rerender the lists
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: LoadingFunction,
}

pub enum Msg {
    Response(Result<GalleryListResponse, APIError>),
    PositionChange(Category, u32, u32),
    PositionChangeResponse(Result<(), APIError>),
    CompleteRefresh,
}

impl Component for ListGalleryRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut on_ends = HashMap::new();
        for category in Category::into_enum_iter() {
            let link_clone = link.clone();
            let category_clone = category.clone();
            let f: Box<dyn FnMut(OnEndEvent)> = Box::new(move |e: OnEndEvent| {
                if e.old_index() != e.new_index() {
                    link_clone.send_message(Msg::PositionChange(
                        category_clone.clone(),
                        e.old_index(),
                        e.new_index(),
                    ));
                }
            });
            on_ends.insert(category.clone(), Closure::wrap(f));
        }
        Self {
            props,
            link,
            task: None,
            error: None,
            results: None,
            on_ends,
            do_refresh: false,
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
            Msg::PositionChange(category, old, new) => {
                let category_list = self.results.as_mut().unwrap().get_mut(&category).unwrap();
                let item = category_list.get(old as usize).unwrap().clone();
                let (move_to_front, move_after_id) = if new == 0 {
                    (true, None)
                } else {
                    let prev_index = if new > old { new } else { new - 1 };
                    let id = category_list.get(prev_index as usize).unwrap().id;
                    (false, Some(id))
                };
                // Do the swap in local storage
                let removed = category_list.remove(old as usize);
                category_list.insert(new as usize, removed);
                // Send the swap details to the server
                self.task = Some(self.props.api_client.gallery_update(
                    item.id,
                    item.description.clone(),
                    item.category.clone(),
                    move_after_id,
                    move_to_front,
                    self.props.on_loading.clone(),
                    self.link.callback(|x| Msg::PositionChangeResponse(x)),
                ));
                self.do_refresh = true;
                self.link.send_message(Msg::CompleteRefresh);
            }
            Msg::PositionChangeResponse(_) => {
                self.refresh();
                return false; // Defer until the response from refresh
            }
            Msg::CompleteRefresh => {
                self.do_refresh = false;
            }
        };
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
                } else if !self.do_refresh {
                    Category::into_enum_iter().map(|c| self.render_category(c)).collect::<Html>()
                } else {
                    html! {}
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
                <div class="row gallery-list-category">
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
            <RouterAnchor route=AppRoute::GalleryEdit(item.id) classes="card mb-3">
                {
                    match image {
                        Some(i) => {
                            html! { <img class="card-img-top" src=i.url/>}
                        }
                        None => { html! {} }
                    }
                }
                <div class="card-body p-2">
                    <p class="card-text">{&item.description}</p>
                </div>
            </RouterAnchor>
        }
    }
}

fn category_to_id(category: &Category) -> String {
    format!("sortable-container-{}", category.serialize())
}
