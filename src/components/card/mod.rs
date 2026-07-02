use tidos::{Component, Page, scoped_css, view, Slot};

pub struct Card {
	pub content: Slot,
}

impl Component for Card {
	fn to_render(&self, page: &mut Page) {
		view! {
			<div class={"{} with-padding", scoped_css!("card.css")}>@slot{&self.content}</div>
		}
	}
}

pub struct AboutCard {
	pub content: Slot,
}

impl Component for AboutCard {
	fn to_render(&self, page: &mut Page) {
		view! {
			<div class={scoped_css!("card.css")}>@slot{&self.content}</div>
		}
	}
}
