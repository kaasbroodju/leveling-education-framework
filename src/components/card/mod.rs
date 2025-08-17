use tidos::{scoped_css, view, Component, Page};

pub struct Card {
    pub content: String
}

impl Component for Card {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div class={format!("{} {}", scoped_css!("card.css"), "with-padding")}>@html{&self.content}</div>
        }
    }
}

pub struct AboutCard {
    pub content: String
}

impl Component for AboutCard {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div class={scoped_css!("card.css")}>@html{&self.content}</div>
        }
    }
}