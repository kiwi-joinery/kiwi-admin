use crate::api::error::APIError;
use crate::api::session::LoginResponse;
use crate::api::APIClient;
use crate::components::error::ErrorAlert;
use crate::components::loading::LoadingProps;
use crate::form_data::GetFormData;
use crate::routes::{AppRoute, RouterAnchor};
use web_sys::FormData;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

const FIELD_EMAIL: &str = "email";
const FIELD_PASSWORD: &str = "password";

#[derive(Default)]
struct Form {
    email: String,
    password: String,
}

pub struct LoginRoute {
    props: Props,
    link: ComponentLink<Self>,
    form: Form,
    task: Option<FetchTask>,
    error: Option<APIError>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_login: Callback<LoginResponse>,
    pub on_loading: Callback<LoadingProps>,
}

pub enum Msg {
    Submit(FormData),
    Response(Result<LoginResponse, APIError>),
}

impl Component for LoginRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
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
                    self.task = Some(self.props.api_client.session_login(
                        self.form.email.clone(),
                        self.form.password.clone(),
                        self.props.on_loading.clone(),
                        self.link.callback(Msg::Response),
                    ));
                }
            }
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(s) => {
                        self.props.on_login.emit(s);
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
                                    <RouterAnchor route=AppRoute::ForgotPassword>
                                        { "Forgot Password?" }
                                    </RouterAnchor>
                                </p>
                                <ErrorAlert<APIError> error=&self.error />
                                <button
                                    class="btn btn-lg btn-primary"
                                    type="submit"
                                    disabled=self.task.is_some()>
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
