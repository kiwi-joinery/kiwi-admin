use enum_iterator::IntoEnumIterator;
use std::fmt::Display;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub enum Msg {
    OnChange(HtmlSelectElement),
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props<T: Clone> {
    pub callback: Callback<T>,
    pub value: T,
    #[prop_or_default]
    pub classes: String, // Classes to be added to component.
}

pub struct EnumSelectorComponent<T: IntoEnumIterator + Clone + PartialEq + Display + 'static> {
    link: ComponentLink<Self>,
    props: Props<T>,
}

impl<T: IntoEnumIterator + Clone + PartialEq + Display + 'static> Component
    for EnumSelectorComponent<T>
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange(x) => {
                let v = T::into_enum_iter()
                    .nth(x.selected_index() as usize)
                    .unwrap();
                self.props.callback.emit(v.clone());
            }
        }
        true
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
        let onchange = self.link.callback(|e: ChangeData| {
            if let ChangeData::Select(x) = e {
                Msg::OnChange(x)
            } else {
                unreachable!()
            }
        });
        html! {
            <select class=&self.props.classes onchange=onchange>
                {
                    T::into_enum_iter()
                        .map(|x| html! {<option selected=(x == self.props.value)>{x.to_string()}</option>})
                        .collect::<Html>()
                }
            </select>
        }
    }
}
