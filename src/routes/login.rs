use crate::api::session::LoginResponse;
use crate::routes::AppRoute;
use std::convert::TryInto;
use wasm_bindgen::JsValue;
use web_sys::Event;
use web_sys::FormData;
use yew::{html, Callback, Component, ComponentLink, FocusEvent, Html, Properties, ShouldRender};
use yew_router::prelude::*;

#[derive(Default)]
struct LoginForm {
    email: String,
    password: String,
}

/// Login page
pub struct Login {
    props: Props,
    link: ComponentLink<Self>,
    form: LoginForm,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub callback: Callback<LoginResponse>,
}

pub enum Msg {
    SubmitForm(FormData),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            props,
            link,
            form: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SubmitForm(fd) => {
                self.form.email = fd.get("email").as_string().unwrap();
                self.form.password = fd.get("password").as_string().unwrap();
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
            let fd = FormData::from(JsValue::from(e.target().unwrap()));
            Msg::SubmitForm(fd)
        });

        html! {
            <div class="container">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        // <ListErrors error=&self.error />
                        <form onsubmit=onsubmit>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value=&self.form.email
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value=&self.form.password
                                        />
                                </fieldset>
                                <p class="text-xs-center">
                                    <RouterAnchor<AppRoute> route=AppRoute::ForgotPassword>
                                        { "Forgot Password?" }
                                    </RouterAnchor<AppRoute>>
                                </p>
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
