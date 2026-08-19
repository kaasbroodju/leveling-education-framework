use crate::components::card::Card;
use markdown::{CompileOptions, Options, to_html_with_options};
use tidos::{Component, Page, view};

/// Content comes from our own trusted data files, not user input, so raw HTML
/// (e.g. `<u>`) is allowed through instead of being stripped.
fn to_html(value: &str) -> String {
	to_html_with_options(
		value,
		&Options {
			compile: CompileOptions {
				allow_dangerous_html: true,
				..CompileOptions::default()
			},
			..Options::default()
		},
	)
	.unwrap()
}

pub struct LeeswijzerContent;

impl Component for LeeswijzerContent {
	fn to_render(&self, page: &mut Page) {
		view! {
			<Card>
				<h1>{"Leeswijzer LEF vaardigheden"}</h1>
				@html{to_html(include_str!("leeswijzer.md"))}
			</Card>
		}
	}
}
