use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct LoadingComponent {
    props: Props,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub text: Option<String>,
}

impl Component for LoadingComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
            {
                if self.props.active {
                    html!{
                        <div id="loading-component">
                            <div class="spinner"></div>
                            {
                                match &self.props.text {
                                    Some(t) => html!{<p>{t}</p>},
                                    None => html!{}
                                }
                            }
                        </div>
                    }
                } else {
                    html!{}
                }
            }
        }
    }
}
