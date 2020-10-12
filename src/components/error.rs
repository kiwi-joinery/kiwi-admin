use crate::api::error::APIError;
use yew::prelude::*;

pub struct ErrorAlert {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub error: Option<APIError>,
}

impl Component for ErrorAlert {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ErrorAlert { props }
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
                if let Some(error) = &self.props.error {
                    html! {
                        <div class="alert alert-danger" role="alert">
                          {error}
                        </div>
                        }
                } else {
                    html! {}
                }
            }
        }
    }
}
