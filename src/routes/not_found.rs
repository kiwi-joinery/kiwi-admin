use crate::routes::{AppRoute, RouterAnchor};
use yew::prelude::*;

pub struct NotFoundRoute {}

impl Component for NotFoundRoute {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h1 class="mb-3">{ "Not found" }</h1>
                <p>
                    {"The requested page could not be found. "}
                    <br/>
                    <RouterAnchor route=AppRoute::Dashboard>
                        { "Return to home" }
                    </RouterAnchor>
                </p>
            </div>
        }
    }
}
