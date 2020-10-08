use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer>
                <div class="container">
                    <div class="row">
                        <div class="col">
                            { "© 2020 " }
                            <a href="https://www.kiwijoinerydevon.co.uk"> { "Kiwi Joinery" } </a>
                        </div>
                        <div class="col text-right">
                            { " Source code available on " }
                            <a href="https://github.com/kiwi-joinery/kiwi-admin"> { "Github" } </a>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
