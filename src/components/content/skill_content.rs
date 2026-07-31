use crate::components::card::Card;
use crate::components::icons::InfoIcon;
use crate::components::navigation::skill_filter_matrix::SkillFilterMatrix;
use crate::data::SKILL_DATA;
use crate::domain::{Level, LevelDescription};
use markdown::{CompileOptions, Options, to_html_with_options};
use std::collections::BTreeMap;
use tidos::{Component, Page, scoped_css, view};

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

pub struct SkillContent;

impl Component for SkillContent {
	fn to_render(&self, page: &mut Page) {
		let content = &(*SKILL_DATA);

		tidos::head! {
			<script>@html{include_str!("skill_filter.js")}</script>
		}

		view! {
			<Card>
				<SkillFilterMatrix />
			</Card>
			{#for (skill, levels) in content.iter()}
				<div data-vaardigheid={skill.to_text()}>
					<Card>
						<Description title={skill.to_text().to_string()} description={&levels.description} levels={&levels.level_description} />
					</Card>
				</div>
			{/for}
		}
	}
}

struct Description<'a> {
	pub levels: &'a BTreeMap<Level, LevelDescription>,
	pub title: String,
	pub description: &'a String,
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
				<h2>{"{}", title.replace('_', " ")}</h2>
				<hr/>
				@html{to_html(&self.description)}
				<hr/>
				<div>
					{#for (level, description) in levels.clone().into_iter()}
						<LevelSection title={title.clone()} level={level} description={description} />
					{/for}
				</div>
				<div>
					{#for (level, description) in levels.into_iter()}
						{#if let Some(x) = description.extra_description}
							<details data-level={format!("{:#?}", level)}>
								<summary><h3>{"Extra context {}", level.to_text()}</h3></summary>
								@html{to_html(&x)}
							</details>
						{:else}
							<div data-level={format!("{:#?}", level)} />
						{/if}
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
			<section data-level={format!("{:#?}", level)}>
				<div class="skill-header">
					<h3>{level.to_text()}</h3>
					// {#if info.is_some()}
					// 	<button lef-modal={"{}-{:#?}", &title, &level} aria-label={"open modal over {} {}", &title, level.to_text()}>
					// 		<InfoIcon />
					// 	</button>
					// {/if}
				</div>
				{#if let Some(subtitle) = &self.description.subtitle}
					<p class="skill-subtitle">{subtitle}</p>
				{/if}
				@html{to_html(&self.description.description)}
			</section>
		}
	}
}