use std::cmp::{max, min};
use yew::prelude::*;

pub struct PaginationComponent {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub total_pages: u32,
    pub current_page: u32, // Starts at zero, like an array
    #[prop_or(5)]
    pub displayed_pages: u32, // How many page numbers should be visible while navigating.
    #[prop_or(1)]
    pub edges: u32, // How many page numbers are visible at the beginning/ending of the pagination.
    pub callback: Callback<u32>,
}

impl Props {
    fn assert_valid(&self) {
        assert!(
            self.current_page < self.total_pages,
            "current_page is out of bounds"
        );
        assert!(
            self.displayed_pages >= 3,
            "displayed_page must be greater than or equal to 3"
        );
        assert!(self.total_pages >= 1, "Must be at least 1 page");
    }
}

pub enum Msg {
    PageChange(u32),
}

// Inspired by https://flaviusmatis.github.io/simplePagination.js/
impl Component for PaginationComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.assert_valid();
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PageChange(i) => self.props.callback.emit(i),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            props.assert_valid();
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let total_pages = self.props.total_pages;
        let edges = self.props.edges;

        let (start, end) = self.interval();

        let mut items = Vec::new();
        // Generate Prev link
        items.push(self.create_item(ItemType::Previous));
        // Generate start edges
        for i in 0..min(edges, start) {
            items.push(self.create_item(ItemType::Regular(i)));
        }
        if edges > 0 && start > edges {
            items.push(self.create_item(ItemType::Ellipsis))
        }
        //Generate interval links
        for idx in start..end + 1 {
            items.push(self.create_item(ItemType::Regular(idx)));
        }
        // Generate end edges
        if edges > 0 && end < total_pages - edges {
            items.push(self.create_item(ItemType::Ellipsis))
        }
        for i in max(total_pages - edges, end + 1)..total_pages {
            items.push(self.create_item(ItemType::Regular(i)));
        }
        // Generate Next link
        items.push(self.create_item(ItemType::Next));
        html! {
            <nav class="mt-3">
                <ul class="pagination">
                    {items}
                </ul>
            </nav>
        }
    }
}

impl PaginationComponent {
    // Generate a range of indexes for the regular interval buttons (inclusive)
    fn interval(&self) -> (u32, u32) {
        let pivot = self.props.current_page;
        let last_page = self.props.total_pages - 1;
        // We cannot select a range bigger than the total pages
        let select = min(self.props.displayed_pages, self.props.total_pages);
        let lhs = ((select - 1) as f32 / 2.0).ceil() as u32;
        let rhs = ((select - 1) as f32 / 2.0).floor() as u32;
        assert_eq!(lhs + rhs + 1, select);

        let (start, end) = if pivot < lhs {
            let rhs = lhs + rhs - pivot;
            let end = if pivot + rhs > last_page {
                last_page
            } else {
                pivot + rhs
            };
            (0, end)
        } else if pivot + rhs > last_page {
            let lhs = lhs + rhs - (last_page - pivot);
            let start = if lhs > pivot { 0 } else { pivot - lhs };
            (start, last_page)
        } else {
            let start = pivot - lhs;
            let end = pivot + rhs;
            (start, end)
        };
        (start, end)
    }

    fn create_item(&self, item_type: ItemType) -> Html {
        let enabled_link = match item_type {
            ItemType::Previous => {
                if self.props.current_page > 0 {
                    Some(self.props.current_page - 1)
                } else {
                    None
                }
            }
            ItemType::Next => {
                let last_page = self.props.total_pages - 1;
                if self.props.current_page >= last_page {
                    None
                } else {
                    Some(self.props.current_page + 1)
                }
            }
            ItemType::Regular(idx) => Some(idx),
            ItemType::Ellipsis => None,
        };
        let is_active = match item_type {
            ItemType::Regular(idx) if idx == self.props.current_page => true,
            _ => false,
        };
        let page_name = match item_type {
            ItemType::Previous => "«".to_string(),
            ItemType::Next => "»".to_string(),
            ItemType::Regular(idx) => (idx + 1).to_string(),
            ItemType::Ellipsis => "…".to_string(),
        };
        let mut classes = Vec::new();
        classes.push("page-item");
        if is_active {
            classes.push("active");
        }
        if enabled_link.is_none() {
            classes.push("disabled");
        }
        let onclick = enabled_link.map(|x| {
            self.link.callback(move |e: MouseEvent| {
                e.prevent_default();
                Msg::PageChange(x)
            })
        });
        html! {
            <li class={classes.join(" ")}>
                {
                    match onclick {
                        Some(c) => html! {<a class="page-link" href="#" onclick=c>{page_name}</a>},
                        None => html! {<a class="page-link" href="#">{page_name}</a>}
                    }
                }
            </li>
        }
    }
}

enum ItemType {
    Previous,
    Next,
    Regular(u32),
    Ellipsis,
}
