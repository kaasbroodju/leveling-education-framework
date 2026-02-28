use tidos::{Component, Page, scoped_css, view};

pub struct HeaderBar;

impl Component for HeaderBar {
	fn to_render(&self, page: &mut Page) -> String {
		view! {
			<header class={scoped_css!("header_bar.css")}>
				<div class={scoped_css!("logo.css")}>
					<a href="/" aria-label="terug naar startpagina">
						<div>
							<div aria-hidden="true">
								@html{include_str!("../../../app/public/logo_light.svg")}
							</div>

							{"Leveling Education Framework"}
						</div>
					</a>
				</div>
			</header>
		}
	}
}
