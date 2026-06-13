use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewAreaProps {
    pub content: String,
}

#[component]
pub fn PreviewArea(props: PreviewAreaProps) -> Element {
    rsx! {
        article {
            class: "markdown-body",
            dangerous_inner_html: "{props.content}"
        }
    }
}
