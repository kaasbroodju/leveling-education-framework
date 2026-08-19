use crate::components::card::Card;
use crate::domain::GUILDS;
use tidos::{Component, Page, scoped_css, view};

pub struct GuildFilterMatrix;

impl Component for GuildFilterMatrix {
	fn to_render(&self, page: &mut Page) {
		view! {
			<header>
				<Card>
					<h2>{"Bachelor"}</h2>
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
				<div style="margin-top: 24px;">
					<Card>
						<h2>{"Master"}</h2>
						<div @class={"guild_filter_matrix.css"}>
							<button
								data-filter-guild={"HCAI"}
								aria-label={"HCAI"}
								style={"--guild-color: #4B0082;"}
							>
								<span>{"HCAI"}</span>
							</button>
						</div>
					</Card>
				</div>
			</header>
		}
	}
}
