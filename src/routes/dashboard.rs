use yew::prelude::*;

pub struct DashboardRoute {}

impl Component for DashboardRoute {
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
        <>
            <h1 class="mb-3">{ "Dashboard" }</h1>
            <p>{ "Useful links:" }</p>
            <ul>
                <li>{ "View website stats on " }
                    <a target="_blank" href="https://analytics.google.com/">{ "Google Analytics" }</a>
                </li>
                <li>{ "Manage the domain name registration at " }
                    <a target="_blank" href="https://www.heartinternet.uk/login">{ "Heart Internet" }</a>
                </li>
                <li>{ "Manage Kiwi Joinery panel on Google Search and Maps on " }
                    <a target="_blank" href="https://accounts.google.com/signin/v2/identifier?service=lbc">{ "My Business" }</a>
                </li>
                <li>{ "View the Kiwi Joinery page on " }
                    <a target="_blank" href="https://www.facebook.com/kiwijoinery">{ "Facebook" }</a>
                </li>
                <li>{ "View your email account at " }
                    <a target="_blank" href="https://outlook.office365.com/mail/login.html">{ "Office 365" }</a>
                </li>
            </ul>
            <p>{"Google Reviews Link - share this address with your customers:"}<br/>{"https://g.page/kiwijoinery/review?rc"}</p>
        </>
        }
    }
}
