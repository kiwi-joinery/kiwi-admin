use crate::api::error::APIError;
use crate::api::APIClient;
use crate::components::error::ErrorAlert;
use crate::components::loading::LoadingProps;
use wasm_bindgen::JsValue;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew::services::fetch::FetchTask;

const FIELD_EMAIL: &str = "email";

pub struct ForgotPasswordRoute {
    props: Props,
    link: ComponentLink<Self>,
    success: bool,
    error: Option<APIError>,
    email: String,
    task: Option<FetchTask>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: Callback<LoadingProps>,
}

pub enum Msg {
    Submit(FormData),
    Response(Result<(), APIError>),
}

impl Component for ForgotPasswordRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ForgotPasswordRoute {
            props,
            link,
            error: None,
            email: "".to_string(),
            success: false,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {
                self.email = fd.get(FIELD_EMAIL).as_string().unwrap();
                if self.task.is_none() {
                    self.success = false;
                    self.error = None;
                    self.task = Some(self.props.api_client.password_reset_request(
                        self.email.clone(),
                        self.props.on_loading.clone(),
                        self.link.callback(Msg::Response),
                    ));
                }
            }
            Msg::Response(r) => {
                self.task = None;
                match r {
                    Ok(_) => {
                        self.success = true;
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
        html! {
            <div class="container">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Forgot Password" }</h1>
                        {
                            if self.success {
                                html! {
                                    <div class="alert alert-success" role="alert">
                                        {"Success - Please check your emails"}
                                    </div>
                                }
                            } else {
                                self.form()
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }
}

impl ForgotPasswordRoute {
    fn form(&self) -> Html {
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            let f: HtmlFormElement = JsValue::from(e.target().unwrap()).into();
            let fd = FormData::new_with_form(&f).unwrap();
            Msg::Submit(fd)
        });
        html! {
        <>
            <p>{ "Enter your account email address to receive a password reset" }</p>
            <form onsubmit=onsubmit>
                <fieldset>
                    <fieldset class="form-group">
                        <input
                            class="form-control form-control-lg"
                            type="email"
                            placeholder="Email"
                            name=FIELD_EMAIL
                            value=&self.email
                            />
                    </fieldset>
                    <ErrorAlert<APIError> error=&self.error />
                    <button
                        class="btn btn-lg btn-primary"
                        type="submit"
                        disabled=self.task.is_some()>
                    { "Request Reset" }
                    </button>
                </fieldset>
            </form>
        </>
        }
    }
}
