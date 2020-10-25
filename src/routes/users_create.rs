use crate::api::error::APIError;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::components::error::ErrorAlert;
use crate::form_data::GetFormData;
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, Route, RouteAgentDispatcher};
use web_sys::FormData;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::agent::RouteRequest;

const FIELD_NAME: &str = "name";
const FIELD_EMAIL: &str = "email";

#[derive(Default)]
struct Form {
    name: String,
    email: String,
}

pub struct CreateUserRoute {
    props: Props,
    link: ComponentLink<Self>,
    error: Option<APIError>,
    task: Option<FetchTask>,
    form: Form,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: LoadingFunction,
}

pub enum Msg {
    Submit(FormData),
    Response(Result<UserResponseItem, APIError>),
}

impl Component for CreateUserRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            error: None,
            task: None,
            form: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {
                self.form.name = fd.get(FIELD_NAME).as_string().unwrap();
                self.form.email = fd.get(FIELD_EMAIL).as_string().unwrap();
                if self.task.is_none() {
                    self.error = None;
                    self.task = Some(self.props.api_client.users_create(
                        self.form.name.clone(),
                        self.form.email.clone(),
                        self.props.on_loading.clone(),
                        self.link.callback(Msg::Response),
                    ));
                }
            }
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(_) => {
                        let mut agent = RouteAgentDispatcher::new();
                        agent.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Users)));
                    }
                    Err(e) => {
                        self.error = Some(e);
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
        let onsubmit = self.link.on_form_submit(|f| Msg::Submit(f));
        html! {
            <div class="container">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="mb-3">{ "Create new user" }</h1>
                        <form onsubmit=onsubmit>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        placeholder="Name"
                                        value=&self.form.name
                                        name=FIELD_NAME
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value=&self.form.email
                                        name=FIELD_EMAIL
                                        />
                                </fieldset>
                                <ErrorAlert<APIError> error=&self.error />
                                <p>{"Note: The user will be sent an email allowing them to set their password"}</p>
                                <button
                                    class="btn btn-lg btn-primary"
                                    type="submit"
                                    disabled=self.task.is_some()>
                                    { "Create" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        }
    }
}
