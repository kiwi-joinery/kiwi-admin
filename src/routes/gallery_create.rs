use crate::api::error::APIError;
use crate::api::APIClient;
use crate::components::error::ErrorAlert;
use crate::form_data::GetFormData;
use crate::loader_task::{BoxedLoadingTask, LoadingFunction};
use crate::routes::{AppRoute, Route, RouteAgentDispatcher};
use web_sys::{File, FormData};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::reader::{FileData, ReaderService, ReaderTask};
use yew_router::agent::RouteRequest;

#[derive(Default)]
struct Form {
    name: String,
    email: String,
}

pub struct CreateGalleryItemRoute {
    props: Props,
    link: ComponentLink<Self>,
    error: Option<APIError>,
    task: Option<FetchTask>,
    form: Form,
    loading_task: Option<BoxedLoadingTask>,
    read_task: Option<ReaderTask>,
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {}
            Msg::Response(r) => {}
            Msg::SelectFile(file) => {
                self.loading_task = Some((*self.props.loader)());
                let callback = self.link.callback(Msg::FileLoaded);
                let mut service = ReaderService::new();
                self.read_task = Some(service.read_file(file, callback).unwrap());
            }
            Msg::FileLoaded(data) => {
                self.loading_task = None;
                self.read_task = None;
                log::info!("{} {}", data.name, data.content.len())
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
                            <fieldset class="form-group">
                                <label for="exampleFormControlSelect1">{ "Category" }</label>
                                <select class="form-control form-control-lg" id="exampleFormControlSelect1">
                                    <option>{"1"}</option>
                                </select>
                            </fieldset>
                            <fieldset class="form-group">
                                <label for="description_textarea">{ "Image Description" }</label>
                                <textarea
                                    class="form-control form-control-lg"
                                    id="description_textarea"
                                    rows="4"
                                    maxlength="4096"
                                    />
                            </fieldset>
                            <ErrorAlert<APIError> error=&self.error />
                            <button
                                class="btn btn-lg btn-primary"
                                type="submit"
                                disabled=self.task.is_some()>
                                { "Upload" }
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        }
    }
}
