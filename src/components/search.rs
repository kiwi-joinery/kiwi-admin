use std::time::Duration;
use yew::prelude::*;
use yew::services::timeout::TimeoutTask;
use yew::services::TimeoutService;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<Option<String>>,
    #[prop_or_default]
    pub classes: String, // Classes to be added to component.
}

pub enum Msg {
    OnChange(String),
    OnTimer,
}

pub struct SearchBarComponent {
    props: Props,
    link: ComponentLink<Self>,
    value: String,
    delay: Option<TimeoutTask>,
}

impl Component for SearchBarComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            value: "".to_string(),
            delay: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange(s) => {
                self.value = s;
                self.delay = Some(TimeoutService::spawn(
                    Duration::from_millis(300),
                    self.link.callback(|_| Msg::OnTimer),
                ));
            }
            Msg::OnTimer => {
                let s = if self.value.len() > 0 {
                    Some(self.value.clone())
                } else {
                    None
                };
                self.props.callback.emit(s);
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
        let oninput = self.link.callback(|e: InputData| Msg::OnChange(e.value));
        html! {
            <input
                class=format!{"form-control {}", self.props.classes}
                type="search"
                placeholder="Search..."
                autocomplete="off"
                spellcheck="false"
                oninput=oninput
                value=&self.value
            />
        }
    }
}
