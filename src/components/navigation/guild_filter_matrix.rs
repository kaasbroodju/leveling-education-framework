use crate::components::card::Card;
use crate::domain::GUILDS;
use tidos::{Component, Page, scoped_css, view};

pub struct GuildFilterMatrix;

impl Component for GuildFilterMatrix {
	fn to_render(&self, page: &mut Page) {
		view! {
			<header>
				<Card>
					<div @class={"guild_filter_matrix.css"}>
						{#for x in GUILDS}
							<button
								data-filter-guild={x.get_short_name()}
								aria-label={x.get_short_name()}
								style={"--guild-color: {};", x.get_color()}
							>
								<span>{x.get_short_name()}</span>
							</button>
						{/for}
					</div>
				</Card>
			</header>
		}
	}
}
