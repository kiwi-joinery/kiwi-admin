use crate::routes::{AppRoute, RouterAnchor};
use yew::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum SidebarActive {
    Dashboard,
    Users,
    Gallery,
}

pub struct SidebarComponent {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub active: Option<SidebarActive>,
}

impl Component for SidebarComponent {
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

    // https://getbootstrap.com/docs/4.1/examples/dashboard/
    fn view(&self) -> Html {
        html! {
            <div class="container-fluid">
                <div class="row">
                    <nav class="col-md-2 d-none d-md-block bg-light sidebar">
                        <div class="sidebar-sticky">
                            <ul class="nav flex-column">
                                <li class="nav-item">
                                    <RouterAnchor route=AppRoute::Dashboard classes={self.li_class(SidebarActive::Dashboard)}>
                                        { "Dashboard" }
                                    </RouterAnchor>
                                </li>
                                <li class="nav-item">
                                    <RouterAnchor route=AppRoute::Users classes={self.li_class(SidebarActive::Users)}>
                                        { "Users" }
                                    </RouterAnchor>
                                </li>
                                <li class="nav-item">
                                    <RouterAnchor route=AppRoute::Gallery classes={self.li_class(SidebarActive::Gallery)}>
                                        { "Gallery" }
                                    </RouterAnchor>
                                </li>
                            </ul>
                        </div>
                    </nav>
                    <main class="col-md-9 ml-sm-auto col-lg-10 px-4">
                        {self.props.children.clone()}
                    </main>
                </div>
            </div>
        }
    }
}

impl SidebarComponent {
    fn li_class(&self, s: SidebarActive) -> &'static str {
        match &self.props.active {
            Some(a) if *a == s => "nav-link active",
            _ => "nav-link",
        }
    }
}
