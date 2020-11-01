use crate::api::error::APIError;
use crate::api::gallery::Category;
use crate::api::APIClient;
use crate::components::enum_selector::EnumSelectorComponent;
use crate::components::error::ErrorAlert;
use crate::form_data::GetFormData;
use crate::loader_task::{BoxedLoadingTask, LoadingFunction, LoadingTaskConfig};
use crate::routes::{AppRoute, Route, RouteAgentDispatcher};
use imagesize::{ImageError, ImageSize};
use num_rational::Ratio;
use std::rc::Rc;
use thiserror::Error;
use web_sys::{File, FormData};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::reader::{FileData, ReaderService, ReaderTask};
use yew_router::agent::RouteRequest;

const MIN_RECOMMENDED_RESOLUTION: usize = 1920 * 1080;
const RECOMMENDED_ASPECT: Ratio<usize> = Ratio::new_raw(16, 9);
const FIELD_DESCRIPTION: &str = "description";

struct Form {
    description: String,
    category: Category,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            description: Default::default(),
            category: Category::Staircases,
        }
    }
}

#[derive(Debug, Error, Clone)]
enum Error {
    #[error("{0}")]
    APIError(APIError),
    #[error("{0}")]
    ImageError(Rc<ImageError>),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Error::APIError(e) => {
                if let Error::APIError(f) = other {
                    return e.eq(f);
                }
            }
            Error::ImageError(e) => {
                if let Error::ImageError(f) = other {
                    return Rc::ptr_eq(e, f);
                }
            }
        }
        return false;
    }
}

pub struct CreateGalleryItemRoute {
    props: Props,
    link: ComponentLink<Self>,
    error: Option<Error>,
    task: Option<FetchTask>,
    form: Form,
    loading_task: Option<BoxedLoadingTask>,
    read_task: Option<ReaderTask>,
    image: Option<(FileData, ImageSize)>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub loader: LoadingFunction,
}

pub enum Msg {
    Submit(FormData),
    SelectFile(File),
    FileLoaded(FileData),
    Response(Result<(), APIError>),
    CategoryChange(Category),
}

impl Component for CreateGalleryItemRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            error: None,
            task: None,
            form: Default::default(),
            loading_task: None,
            read_task: None,
            image: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {
                self.form.description = fd.get(FIELD_DESCRIPTION).as_string().unwrap();
                self.task = Some(self.props.api_client.gallery_create(
                    &self.image.as_ref().unwrap().0,
                    self.form.description.clone(),
                    &self.form.category,
                    self.props.loader.clone(),
                    self.link.callback(Msg::Response),
                ));
            }
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(_) => {
                        let mut agent = RouteAgentDispatcher::new();
                        agent.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Gallery)));
                    }
                    Err(e) => {
                        self.error = Some(Error::APIError(e));
                    }
                }
            }
            Msg::SelectFile(file) => {
                self.loading_task = Some((*self.props.loader)(
                    LoadingTaskConfig::default().delay_full_appearance(false),
                ));
                let callback = self.link.callback(Msg::FileLoaded);
                let mut service = ReaderService::new();
                self.read_task = Some(service.read_file(file, callback).unwrap());
            }
            Msg::FileLoaded(data) => {
                self.read_task = None;
                self.image = None;
                self.error = None;
                log::info!("{} {}", data.name, data.content.len());
                match imagesize::blob_size(&data.content) {
                    Ok(s) => {
                        self.image = Some((data, s));
                    }
                    Err(e) => {
                        self.error = Some(Error::ImageError(Rc::new(e)));
                    }
                }
                self.loading_task = None;
            }
            Msg::CategoryChange(c) => {
                log::info!("{:?}", c);
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
        let onsubmit = self.link.on_form_submit(|f| Msg::Submit(f));
        let onchange = self.link.callback(|v: ChangeData| match v {
            ChangeData::Files(f) => {
                let file: File = f.get(0).unwrap();
                Msg::SelectFile(file)
            }
            _ => unreachable!(),
        });
        let oncategory = self.link.callback(|x| Msg::CategoryChange(x));
        html! {
            <div class="container">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="mb-3">{ "Upload Image" }</h1>
                        <form onsubmit=onsubmit>
                            <fieldset class="form-group">
                                <label for="file_input">{ "Choose Image" }</label>
                                <input
                                    type="file"
                                    class="form-control-file"
                                    id="file_input"
                                    onchange=onchange
                                    />
                            </fieldset>
                            { self.image_info() }
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
                            <ErrorAlert<Error> error=&self.error />
                            <button
                                class="btn btn-lg btn-primary"
                                type="submit"
                                disabled=self.task.is_some() || self.image.is_none()>
                                { "Upload" }
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        }
    }
}

impl CreateGalleryItemRoute {
    fn image_info(&self) -> Html {
        match &self.image {
            None => return html! {},
            Some((_, size)) => {
                let resolution = size.height * size.width;
                let aspect_ratio = Ratio::new(size.width, size.height);
                let mut warnings = Vec::new();
                if resolution < MIN_RECOMMENDED_RESOLUTION {
                    warnings.push("Resolution is lower than the recommended minimum (1080p)")
                }
                if aspect_ratio != RECOMMENDED_ASPECT {
                    warnings.push("Different aspect ratio to the recommended size (16:9)")
                }
                if warnings.len() > 0 {
                    return html! {
                        <div class="alert alert-warning" role="alert">
                            {"Image Warnings:"}
                            <ul>
                                {warnings
                                .iter()
                                .map(|x| html! {<li>{x}</li>})
                                .collect::<Html>()
                                }
                            </ul>
                        </div>
                    };
                } else {
                    return html! {};
                }
            }
        };
    }
}
