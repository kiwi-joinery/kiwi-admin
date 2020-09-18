use yew::prelude::*;

pub(crate) struct App {
    link: ComponentLink<Self>,
}

pub(crate) enum Msg {
    Submit,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <form onsubmit=self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
            })>
                <div class="row">
                    <div class="col-xs-6 col-sm-offset-3">
                        <div id="message-box"></div>
                        <div class="form-group">
                            <label title="Email Address" for="email_address">{ "Email Address:" }</label>
                            <input name="email_address" type="email" value="" class="form-control"/>
                        </div>
                        <div class="form-group">
                            <label title="Password" for="password">{ "Password:" }</label>
                            <input id="password" name="password" type="password" class="form-control"/>
                        </div>
                        <div class="form-group">
                            <input type="submit" id="submit" value="Login" class="btn btn-black btn-block" />
                        </div>
                        <br/>
                        <div style="text-align: center">
                            <a href="forgot_password">{ "Forgot Password?" }</a>
                        </div>
                    </div>
                </div>
            </form>
        }
    }
}
