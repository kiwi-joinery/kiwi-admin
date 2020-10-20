use crate::api::error::APIError;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use crate::components::error::ErrorAlert;
use crate::components::loading::LoadingProps;
use crate::routes::{AppRoute, Route, RouteAgentDispatcher};
use wasm_bindgen::JsValue;
use web_sys::{FormData, HtmlFormElement};
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

pub struct EditUserRoute {
    props: Props,
    link: ComponentLink<Self>,
    load_task: Option<FetchTask>,
    task: Option<FetchTask>,
    load_error: Option<APIError>,
    edit_error: Option<APIError>,
    delete_error: Option<APIError>,
    form: Form,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub api_client: APIClient,
    pub on_loading: Callback<LoadingProps>,
    pub user_id: u32,
}

pub enum Msg {
    LoadResponse(Result<UserResponseItem, APIError>),
    Submit(FormData),
    EditResponse(Result<UserResponseItem, APIError>),
    DeleteButton,
}

impl Component for EditUserRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = props.api_client.users_get(
            props.user_id,
            Some(props.on_loading.clone()),
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(fd) => {
                self.form.name = fd.get(FIELD_NAME).as_string().unwrap();
                self.form.email = fd.get(FIELD_EMAIL).as_string().unwrap();
                if self.load_task.is_none() && self.task.is_none() {
                    self.edit_error = None;
                    self.task = Some(self.props.api_client.users_update(
                        self.props.user_id,
                        self.form.name.clone(),
                        self.form.email.clone(),
                        self.props.on_loading.clone(),
                        self.link.callback(Msg::EditResponse),
                    ));
                }
            }
            Msg::LoadResponse(r) => {
                self.load_task = None;
                match r {
                    Ok(x) => {
                        self.form.name = x.name;
                        self.form.email = x.email;
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
                        agent.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Users)));
                    }
                    Err(e) => {
                        self.edit_error = Some(e);
                    }
                }
            }
            Msg::DeleteButton => {}
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
                    </div>
                </div>
            </div>
        }
    }
}

impl EditUserRoute {
    fn form(&self) -> Html {
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            let f: HtmlFormElement = JsValue::from(e.target().unwrap()).into();
            let fd = FormData::new_with_form(&f).unwrap();
            Msg::Submit(fd)
        });
        let ondelete = self.link.callback(|e: MouseEvent| Msg::DeleteButton);
        html! {
        <>
            <h1>{ "Edit user" }</h1>
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
                    <p>{"Make sure the email is correct, since it can be used to reset the password!"}</p>
                    <ErrorAlert<APIError> error=&self.edit_error />
                    <button
                        class="btn btn-lg btn-primary"
                        type="submit"
                        disabled=self.task.is_some()
                        > { "Update" }
                    </button>
                </fieldset>
                <hr/>
                <button
                    type="button"
                    class="btn btn-danger mt-1"
                    onclick=ondelete
                    > {"Delete User"}
                </button>
                <ErrorAlert<APIError> error=&self.delete_error />
            </form>
        </>
        }
    }
}
