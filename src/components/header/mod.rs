use crate::components::settings::SettingsButton;
use tidos::{Component, Page, scoped_css, view};

pub struct HeaderBar;

impl Component for HeaderBar {
	fn to_render(&self, page: &mut Page) {
		view! {
			<header @class={"header_bar.css"}>
				<div @class={"logo.css"}>
					<a href="/" aria-label="terug naar startpagina">
						<div>
							<div aria-hidden="true">
								@html{include_str!("../../../app/public/logo_light.svg")}
							</div>

							{"Leveling Education Framework"}
						</div>
					</a>
				</div>
				<SettingsButton />
			</header>
		}
	}
}
