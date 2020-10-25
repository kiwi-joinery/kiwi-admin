use std::time::Duration;
use yew::prelude::*;
use yew::services::timeout::TimeoutTask;
use yew::services::TimeoutService;

pub struct LoadingComponent {
    props: LoadingProps,
    timeout: Option<TimeoutTask>,
    link: ComponentLink<Self>,
    fully_visible: bool,
    became_fully_visible: Option<f64>,
    performance: web_sys::Performance,
}

#[derive(PartialEq, Properties, Clone, Default)]
pub struct LoadingProps {
    pub active: bool,
    pub delay_full_appearance: bool,
    pub text: Option<String>,
}

pub enum Msg {
    EnableFullyVisible,
    DisableFullyVisible,
}

// (Only applies if activated with delay_full_appearance)
// When activated the full screen will still be locked immediately with a transparent overlay
// However it will wait before fully displaying the spinner
// This way for shorter tasks, the spinner will never need to be displayed
const ONLY_SHOW_AFTER_MILLIS: u64 = 400;

// If the loader is shown it must appear for at least this time - so it doesn't do an ugly flash
const SHOW_FOR_AT_LEAST_MILLIS: u64 = 500;

impl Component for LoadingComponent {
    type Message = Msg;
    type Properties = LoadingProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let a = props.active;
        Self {
            props,
            timeout: None,
            link,
            fully_visible: a,
            became_fully_visible: None,
            performance: web_sys::window().unwrap().performance().unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EnableFullyVisible => {
                self.fully_visible = true;
                self.became_fully_visible = Some(self.performance.now());
            }
            Msg::DisableFullyVisible => {
                self.fully_visible = false;
                self.became_fully_visible = None;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            let old = self.props.clone();
            self.props = props;
            self.on_props_change(old);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            {
                if self.fully_visible {
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

impl LoadingComponent {
    fn on_props_change(&mut self, old: LoadingProps) {
        // When activated
        if !old.active && self.props.active {
            self.timeout = None;
            if self.props.delay_full_appearance {
                self.timeout = Some(TimeoutService::spawn(
                    Duration::from_millis(ONLY_SHOW_AFTER_MILLIS),
                    self.link.callback(|_| Msg::EnableFullyVisible),
                ));
            } else {
                self.fully_visible = true;
                self.became_fully_visible = Some(self.performance.now());
            }
        }
        // When deactivated
        if old.active && !self.props.active {
            self.timeout = None;
            match self.became_fully_visible {
                None => {}
                Some(became_fully_visible) => {
                    let elapsed = (self.performance.now() - became_fully_visible) as u64;
                    if elapsed < SHOW_FOR_AT_LEAST_MILLIS {
                        let delay = SHOW_FOR_AT_LEAST_MILLIS - elapsed;
                        self.timeout = Some(TimeoutService::spawn(
                            Duration::from_millis(delay),
                            self.link.callback(|_| Msg::DisableFullyVisible),
                        ));
                    } else {
                        self.fully_visible = false;
                        self.became_fully_visible = None;
                    }
                }
            }
        }
    }
}
