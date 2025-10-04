use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewAreaProps {
    pub content: String,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn PreviewArea(props: PreviewAreaProps) -> Element {
    rsx! {
        article {
            class: "prose max-w-none {props.class}",
            dangerous_inner_html: "{props.content}"
        }
    }
}
