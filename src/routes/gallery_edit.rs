use crate::api::error::APIError;
use crate::api::gallery::{Category, GalleryFileResponse, GalleryItemResponse};
use crate::api::APIClient;
use crate::components::enum_selector::EnumSelectorComponent;
use crate::components::error::ErrorAlert;
use crate::form_data::GetFormData;
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, Route, RouteAgentDispatcher};
use web_sys::FormData;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::agent::RouteRequest;

const FIELD_DESCRIPTION: &str = "description";

struct Form {
    description: String,
    category: Category,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            description: "".to_string(),
            category: Category::Staircases,
        }
    }
}

pub struct EditGalleryItemRoute {
    props: Props,
    link: ComponentLink<Self>,
    load_task: Option<FetchTask>,
    task: Option<FetchTask>,
    load_error: Option<APIError>,
    edit_error: Option<APIError>,
    delete_error: Option<APIError>,
    form: Form,
    image: Option<GalleryFileResponse>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: LoadingFunction,
    pub item_id: u32,
}

pub enum Msg {
    LoadResponse(Result<GalleryItemResponse, APIError>),
    Submit(FormData),
    EditResponse(Result<(), APIError>),
    ConfirmDelete,
    DeleteResponse(Result<(), APIError>),
    CategoryChange(Category),
}

impl Component for EditGalleryItemRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = props.api_client.gallery_get(
            props.item_id,
            props.on_loading.clone(),
            link.callback(Msg::LoadResponse),
        );
        Self {
            props,
            link,
            load_task: Some(task),
            load_error: None,
            task: None,
            edit_error: None,
            delete_error: None,
            form: Default::default(),
            image: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {
                self.form.description = fd.get(FIELD_DESCRIPTION).as_string().unwrap();
                if self.load_task.is_none() && self.task.is_none() {
                    self.edit_error = None;
                    self.task = Some(self.props.api_client.gallery_update(
                        self.props.item_id,
                        self.form.description.clone(),
                        self.form.category.clone(),
                        None,
                        false,
                        self.props.on_loading.clone(),
                        self.link.callback(Msg::EditResponse),
                    ));
                }
            }
            Msg::LoadResponse(r) => {
                self.load_task = None;
                match r {
                    Ok(x) => {
                        self.image = x.best_matching_width(800).map(|x| x.clone());
                        self.form.description = x.description;
                        self.form.category = x.category;
                    }
                    Err(e) => {
                        self.load_error = Some(e);
                    }
                }
            }
            Msg::EditResponse(r) => {
                self.task = None;
                match r {
                    Ok(_) => {
                        let mut agent = RouteAgentDispatcher::new();
                        agent.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Gallery)));
                    }
                    Err(e) => {
                        self.edit_error = Some(e);
                    }
                }
            }
            Msg::ConfirmDelete => {
                self.delete_error = None;
                self.task = Some(self.props.api_client.gallery_delete(
                    self.props.item_id,
                    self.props.on_loading.clone(),
                    self.link.callback(Msg::DeleteResponse),
                ));
            }
            Msg::DeleteResponse(r) => {
                self.task = None;
                match r {
                    Ok(_) => {
                        let mut agent = RouteAgentDispatcher::new();
                        agent.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Gallery)));
                    }
                    Err(e) => {
                        self.delete_error = Some(e);
                    }
                }
            }
            Msg::CategoryChange(c) => {
                self.form.category = c;
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
            <div class="container">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        {
                            if self.load_task.is_some() {
                                html! {}
                            } else if self.load_error.is_some() {
                                html! {<ErrorAlert<APIError> error=&self.load_error />}
                            } else {
                                self.form()
                            }
                        }
                        {
                            self.delete_modal()
                        }
                    </div>
                </div>
            </div>
        }
    }
}

impl EditGalleryItemRoute {
    fn form(&self) -> Html {
        let oncategory = self.link.callback(|x| Msg::CategoryChange(x));
        let onsubmit = self.link.on_form_submit(|f| Msg::Submit(f));
        html! {
        <>
            <h1 class="mb-3">{ "Edit image" }</h1>
            {
                match &self.image {
                    None => html! {},
                    Some(i) => html! {<img class="img-fluid rounded mb-3" src=i.url/>},
                }
            }
            <form onsubmit=onsubmit>
                <fieldset class="form-group">
                    <label>{ "Category" }</label>
                    <EnumSelectorComponent<Category>
                        callback=oncategory
                        classes="form-control form-control-lg"
                        value=self.form.category.clone()
                    />
                </fieldset>
                <fieldset class="form-group">
                    <label for="description_textarea">{ "Image Description" }</label>
                    <textarea
                        class="form-control form-control-lg"
                        id="description_textarea"
                        rows="4"
                        maxlength="4096"
                        name=FIELD_DESCRIPTION
                        value=&self.form.description
                        />
                </fieldset>
                <ErrorAlert<APIError> error=&self.edit_error />
                <button
                    class="btn btn-lg btn-primary"
                    type="submit"
                    disabled=self.task.is_some()
                    > { "Update" }
                </button>
                <hr/>
                <button
                    type="button"
                    class="btn btn-danger mt-1 mb-3"
                    data-toggle="modal"
                    data-target="#deleteModal"
                    > {"Delete image"}
                </button>
                <ErrorAlert<APIError> error=&self.delete_error />
            </form>
        </>
        }
    }

    fn delete_modal(&self) -> Html {
        let ondelete = self.link.callback(|_: MouseEvent| Msg::ConfirmDelete);
        html! {
            <div id="deleteModal" class="modal" tabindex="-1" role="dialog">
                <div class="modal-dialog" role="document">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">{"Delete Image"}</h5>
                            <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                            </button>
                        </div>
                        <div class="modal-body">
                            <p>{"Are you sure you want to delete this image?"}</p>
                        </div>
                        <div class="modal-footer">
                            <button
                                type="button"
                                class="btn btn-danger"
                                data-dismiss="modal"
                                onclick=ondelete
                                >{"Delete"}</button>
                            <button type="button" class="btn btn-secondary" data-dismiss="modal">{"Cancel"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
