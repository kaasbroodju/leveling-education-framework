use tidos::{Component, Page, scoped_css, view, Slot};

pub struct Card<'a> (pub Slot<'a>);

impl Component for Card<'_> {
	fn to_render(&self, page: &mut Page) {
		view! {
			<div @class={"card.css", "with-padding"}>@slot{self.0}</div>
		}
	}
}

pub struct AboutCard<'a> (pub Slot<'a>);

impl Component for AboutCard<'_> {
	fn to_render(&self, page: &mut Page) {
		view! {
			<div @class={"card.css"}>@slot{&self.0}</div>
		}
	}
}
