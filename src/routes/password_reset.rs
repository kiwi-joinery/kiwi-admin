use crate::api::error::APIError;
use crate::api::APIClient;
use crate::components::error::ErrorAlert;
use crate::form_data::GetFormData;
use crate::loader_task::LoadingFunction;
use crate::routes::{AppRoute, Route, RouteAgentDispatcher, RouteService};
use serde::Deserialize;
use thiserror::Error;
use web_sys::FormData;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::agent::RouteRequest;

const FIELD_PWD: &str = "password";
const FIELD_PWD_REPEAT: &str = "repeat_password";

#[derive(Debug, Clone, PartialEq, Error)]
enum Error {
    #[error("{0}")]
    APIError(APIError),
    #[error("Passwords do not match")]
    PasswordsDoNotMatch,
}

#[derive(Deserialize)]
struct Query {
    email: String,
    token: String,
}

struct Form {
    email: String,
    token: String,
    password: String,
    repeat_password: String,
}

impl Form {
    fn new(q: Option<Query>) -> Self {
        let q = q.unwrap_or(Query {
            email: "Invalid email".to_string(),
            token: "Invalid Token".to_string(),
        });
        Self {
            email: q.email,
            token: q.token,
            password: "".to_string(),
            repeat_password: "".to_string(),
        }
    }
}

pub struct PasswordResetRoute {
    props: Props,
    link: ComponentLink<Self>,
    error: Option<Error>,
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
    Response(Result<(), APIError>),
}

impl Component for PasswordResetRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let q = RouteService::new().get_query();
        let q = q.strip_prefix("?").unwrap_or("");
        let q: Option<Query> = serde_urlencoded::from_str(&q).ok();
        Self {
            props,
            link,
            error: None,
            task: None,
            form: Form::new(q),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {
                self.form.password = fd.get(FIELD_PWD).as_string().unwrap();
                self.form.repeat_password = fd.get(FIELD_PWD_REPEAT).as_string().unwrap();
                if self.form.password == self.form.repeat_password {
                    self.error = None;
                    if self.task.is_none() {
                        self.task = Some(self.props.api_client.password_reset_submit(
                            self.form.email.clone(),
                            self.form.token.clone(),
                            self.form.password.clone(),
                            self.props.on_loading.clone(),
                            self.link.callback(Msg::Response),
                        ));
                    }
                } else {
                    self.error = Some(Error::PasswordsDoNotMatch);
                }
            }
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(_) => {
                        let mut agent = RouteAgentDispatcher::new();
                        agent.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Login)));
                    }
                    Err(e) => {
                        self.error = Some(Error::APIError(e));
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
                        <h1 class="mb-3">{ "Sign In" }</h1>
                        <form onsubmit=onsubmit>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value=&self.form.email
                                        disabled=true
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        name=FIELD_PWD
                                        value=&self.form.password
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Repeat Password"
                                        name=FIELD_PWD_REPEAT
                                        value=&self.form.repeat_password
                                        />
                                </fieldset>
                                <ErrorAlert<Error> error=&self.error />
                                <button
                                    class="btn btn-lg btn-primary"
                                    type="submit"
                                    disabled=self.task.is_some()>
                                    { "Change Password" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        }
    }
}
