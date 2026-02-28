use crate::components::card::Card;
use crate::components::icons::InfoIcon;
use crate::components::navigation::beroepstaken_filter_matrix::BeroepstakenFilterMatrix;
use crate::data::HBOI_DATA;
use crate::domain::{Level, LevelDescription};
use markdown::to_html;
use std::collections::BTreeMap;
use tidos::{Component, Page, scoped_css, view};

pub struct BeroepstakenContent;

impl Component for BeroepstakenContent {
	fn to_render(&self, page: &mut Page) -> String {
		let content = &(*HBOI_DATA);

		tidos::head! {
			<script>@html{include_str!("beroepstaken_filter.js")}</script>
		}

		view! {
			<BeroepstakenFilterMatrix />
			{#for (skill, levels) in content.iter()}
				<div data-architectuurlaag={format!("{:#?}", skill.architectuurlaag)} data-activiteit={format!("{:#?}", skill.activiteit)}>
					<Card>
						{#slot:content}
							<Description title={skill.to_string()} levels={levels} />
						{/slot}
					</Card>
				</div>
			{/for}
		}
	}
}

struct Description<'a> {
	pub levels: &'a BTreeMap<Level, LevelDescription>,
	pub title: String,
}

impl Component for Description<'_> {
	fn to_render(&self, page: &mut Page) -> String {
		tidos::head! {
			<script defer>@html {include_str!("skill_content.js")}</script>
		}
		view! {
			<section class={scoped_css!("skill_content.css")}>
				<h2>{format!("{}", self.title.replace('_', " "))}</h2>
				<hr/>
				<div>
					{#for (level, description) in self.levels.iter()}
						<section>
							<div class="skill-header">
								<h3>{level.to_text()}</h3>
								{#if let Some(x) = &description.info}
									<button lef-modal={format!("{}-{:#?}", self.title, level)} aria-label={format!("open modal over {} {}", self.title, level.to_text())}>
										<InfoIcon />
									</button>
								{/if}

							</div>
							<p>{&description.title}</p>
							{#if let Some(x) = &description.info}
								<dialog class={scoped_css!("dialog.css")} id={format!("{}-{:#?}", self.title, level)} lef-modal closedby="any">
									<Card>
										{#slot:content}
											<h2>{format!("Extra toelichting {} {}", self.title.replace('_', " ").to_lowercase(), level.to_text().to_lowercase())}</h2>
											@html{to_html(x)}
										{/slot}
									</Card>
								</dialog>
							{/if}
						</section>
					{/for}
				</div>
			</section>
		}
	}
}
