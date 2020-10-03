use crate::api::error::APIError;
use crate::api::session::LoginResponse;
use crate::api::APIClient;
use crate::app::{App, AppMessage};
use crate::components::error::ErrorAlert;
use crate::routes::AppRoute;
use wasm_bindgen::JsValue;
use web_sys::{FormData, HtmlFormElement};
use yew::services::fetch::FetchTask;
use yew::{html, Component, ComponentLink, FocusEvent, Html, Properties, ShouldRender};
use yew_router::prelude::*;

const FIELD_EMAIL: &str = "email";
const FIELD_PASSWORD: &str = "password";

#[derive(Default)]
struct LoginForm {
    email: String,
    password: String,
}

pub struct Login {
    props: Props,
    link: ComponentLink<Self>,
    form: LoginForm,
    task: Option<FetchTask>,
    error: Option<APIError>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub client: APIClient,
    pub app: ComponentLink<App>,
}

pub enum Msg {
    Submit(FormData),
    Response(Result<LoginResponse, APIError>),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            props,
            link,
            form: Default::default(),
            task: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {
                self.form.email = fd.get(FIELD_EMAIL).as_string().unwrap();
                self.form.password = fd.get(FIELD_PASSWORD).as_string().unwrap();
                if self.task.is_none() {
                    self.error = None;
                    self.task = Some(self.props.client.session_login(
                        self.form.email.clone(),
                        self.form.password.clone(),
                        self.props.app.callback(AppMessage::GlobalLoader),
                        self.link.callback(Msg::Response),
                    ));
                }
            }
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(s) => {
                        self.props.app.send_message(AppMessage::LoggedIn(s));
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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            let f: HtmlFormElement = JsValue::from(e.target().unwrap()).into();
            let fd = FormData::new_with_form(&f).unwrap();
            Msg::Submit(fd)
        });

        html! {
            <div class="container">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        <form onsubmit=onsubmit>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        name=FIELD_EMAIL
                                        value=&self.form.email
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        name=FIELD_PASSWORD
                                        value=&self.form.password
                                        />
                                </fieldset>
                                <p class="text-xs-center">
                                    <RouterAnchor<AppRoute> route=AppRoute::ForgotPassword>
                                        { "Forgot Password?" }
                                    </RouterAnchor<AppRoute>>
                                </p>
                                <ErrorAlert error=&self.error />
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        }
    }
}
