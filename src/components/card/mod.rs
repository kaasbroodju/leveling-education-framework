use tidos::{scoped_css, view, Component, Page};

pub struct Card {
    pub content: String
}

impl Component for Card {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div class={scoped_css!("card.css")}>@html{&self.content}</div>
        }
    }
}