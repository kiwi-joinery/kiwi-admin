use std::error::Error;
use yew::prelude::*;

pub struct ErrorAlert<T: Error + Clone + PartialEq> {
    props: Props<T>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: Error + Clone + PartialEq> {
    pub error: Option<T>,
    #[prop_or_default]
    pub classes: String, // Classes to be added to component.
}

impl<T: 'static + Error + Clone + PartialEq> Component for ErrorAlert<T> {
    type Message = ();
    type Properties = Props<T>;

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
                if let Some(error) = &self.props.error {
                    html! {
                        <div class=format!{"alert alert-danger {}", self.props.classes} role="alert">
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
