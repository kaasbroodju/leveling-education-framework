use crate::components::card::Card;
use crate::components::icons::InfoIcon;
use crate::components::navigation::skill_filter_matrix::SkillFilterMatrix;
use crate::data::SKILL_DATA;
use crate::domain::{Level, LevelDescription, Skill, VaardighedenResponseBody};
use markdown::to_html;
use std::collections::BTreeMap;
use tidos::{Component, Page, scoped_css, view};

pub struct SkillContent {
	pub filter: Option<Skill>,
}

impl Component for SkillContent {
	fn to_render(&self, page: &mut Page) -> String {
		let content = &(*SKILL_DATA);

		view! {
			<Card>
				{#slot:content}
					<SkillFilterMatrix filter={&self.filter} />
				{/slot}
			</Card>
			{#for (skill, levels) in content
				.iter()
				.filter(|(s, _)| self.filter.as_ref().map_or(true, |f| f == *s))
			}
				<Card>
					{#slot:content}
						<Description title={skill.to_text().to_string()} levels={levels} />
					{/slot}
				</Card>
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

							// @html{markdown::to_html(&description.title)}
						</section>
					{/for}
				</div>
			</section>
		}
	}
}
