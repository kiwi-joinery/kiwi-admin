use yew::prelude::*;

pub struct FooterComponent {}

impl Component for FooterComponent {
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

    // https://getbootstrap.com/docs/4.1/examples/sticky-footer/
    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="container">
                    <div class="row">
                        <div class="col">
                            { "© 2020 " }
                            <a target="_blank" href="https://www.kiwijoinerydevon.co.uk"> { "Kiwi Joinery" } </a>
                        </div>
                        <div class="col text-right">
                            { " Source code available on " }
                            <a target="_blank" href="https://github.com/kiwi-joinery/kiwi-admin"> { "Github" } </a>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
