use std::time::Duration;
use yew::prelude::*;
use yew::services::timeout::TimeoutTask;
use yew::services::TimeoutService;

pub struct LoadingComponent {
    props: LoadingProps,
    timeout: Option<TimeoutTask>,
    link: ComponentLink<Self>,
    delayed_active: bool,
}

#[derive(PartialEq, Properties, Clone, Default)]
pub struct LoadingProps {
    pub active: bool,
    pub text: Option<String>,
}

impl LoadingProps {
    pub fn disabled() -> LoadingProps {
        LoadingProps {
            active: false,
            text: None,
        }
    }
    pub fn enabled(text: Option<String>) -> LoadingProps {
        LoadingProps { active: true, text }
    }
}

pub enum Msg {
    Update,
}

// Wait before displaying the spinner - if it finishes before this time then it is never shown.
const ONLY_SHOW_AFTER_MILLIS: u64 = 400;
// If the loader is shown it must appear for at least this time - so it doesn't do an ugly flash
const SHOW_FOR_AT_LEAST_MILLIS: u64 = 300;

impl Component for LoadingComponent {
    type Message = Msg;
    type Properties = LoadingProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let a = props.active;
        Self {
            props,
            timeout: None,
            link,
            delayed_active: a,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update => self.delayed_active = self.props.active,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.active != self.props.active {
            self.timeout = None;
            let mil = if props.active {
                ONLY_SHOW_AFTER_MILLIS
            } else {
                SHOW_FOR_AT_LEAST_MILLIS
            };
            self.props = props;
            self.timeout = Some(TimeoutService::spawn(
                Duration::from_millis(mil),
                self.link.callback(|_| Msg::Update),
            ));
        } else {
            self.props = props;
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            {
                if self.delayed_active {
                    html!{
                        <div id="loading-component">
                            <div id="loading-component-spinner"></div>
                            {
                                match &self.props.text {
                                    Some(t) => html!{<p>{t}</p>},
                                    None => html!{}
                                }
                            }
                        </div>
                    }
                } else if self.props.active {
                    html! {
                        <div id="loading-component" style="background-color:transparent"></div>
                    }
                } else {
                    html!{}
                }
            }
        }
    }
}
