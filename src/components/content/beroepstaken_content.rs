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
	fn to_render(&self, page: &mut Page) {
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
	fn to_render(&self, page: &mut Page) {
		tidos::head! {
			<script defer>@html {include_str!("skill_content.js")}</script>
		}

		let title = self.title.clone();
		let levels: Vec<(Level, LevelDescription)> = self.levels
			.iter()
			.map(|(k, v)| (k.clone(), v.clone()))
			.collect();

		view! {
			<section class={scoped_css!("skill_content.css")}>
				<h2>{format!("{}", title.replace('_', " "))}</h2>
				<hr/>
				<div>
					{#for (level, description) in levels.into_iter()}
						<LevelSection title={title.clone()} level={level} description={description} />
					{/for}
				</div>
			</section>
		}
	}
}

struct LevelSection {
	title: String,
	level: Level,
	description: LevelDescription,
}

impl Component for LevelSection {
	fn to_render(&self, page: &mut Page) {
		let title = self.title.clone();
		let level = self.level.clone();
		let info = self.description.extra_description.clone();

		view! {
			<section>
				<div class="skill-header">
					<h3>{level.to_text()}</h3>
					{#if info.is_some()}
						<button lef-modal={"{}-{:#?}", &title, &level} aria-label={"open modal over {} {}", &title, level.to_text()}>
							<InfoIcon />
						</button>
					{/if}
				</div>
				<p>{&self.description.description}</p>
				{#if let Some(x) = info}
					<dialog class={scoped_css!("dialog.css")} id={"{}-{:#?}", &title, &level} lef-modal closedby="any">
						<Card>
							{#slot:content}
								<h2>{"Extra toelichting {} {}", title.replace('_', " ").to_lowercase(), level.to_text().to_lowercase()}</h2>
								@html{to_html(&x)}
							{/slot}
						</Card>
					</dialog>
				{/if}
			</section>
		}
	}
}