use crate::components::card::Card;
use crate::components::navigation::beroepstaken_filter_matrix::BeroepstakenFilterMatrix;
use crate::data::EXAMPLES_DATA;
use crate::domain::{Activiteit, Architectuurlaag, Guild, HBOIExampleResponse, HBOIKey};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use tidos::{Component, Page, scoped_css, view};

pub struct BeroepsproductenContent;

impl Component for BeroepsproductenContent {
	fn to_render(&self, page: &mut Page) {
		let content = &(*EXAMPLES_DATA);

		let mut grouped_content = BTreeMap::<HBOIKey, Vec<&HBOIExampleResponse>>::new();
		for example in content {
			let key = HBOIKey {
				architectuurlaag: example.architecture_layer.clone(),
				activiteit: example.activity.clone(),
			};
			match grouped_content.get_mut(&key) {
				None => {
					grouped_content.insert(key, vec![example]);
				}
				Some(examples) => {
					examples.push(example);
				}
			}
		}

		grouped_content.values_mut().for_each(|examples| {
			examples.sort_by(|a, b| match a.guild.cmp(&b.guild) {
				Ordering::Less => Ordering::Less,
				Ordering::Equal => a.title.cmp(&b.title),
				Ordering::Greater => Ordering::Greater,
			})
		});

		tidos::head! {
			<script>@html{include_str!("../beroepstaken_filter.js")}</script>
		}

		view! {
			<Card>
				<p>{"Per beroepstaak zijn voorbeelden van beroepsproducten beschreven. Deze voorbeelden geven je een beeld van wat je binnen een beroepstaak kunt maken of opleveren. Je kunt de productnamen gebruiken als inspiratie en als startpunt bij het zoeken naar passende oplossingen en manieren van werken."}</p>
			</Card>
			<BeroepstakenFilterMatrix />
			{#if !grouped_content.is_empty()}
				{#for (key, examples) in grouped_content}
					<div data-architectuurlaag={"{:#?}", key.architectuurlaag} data-activiteit={"{:#?}", key.activiteit}>
						<Card>
							<Description architectuurlaag={&key.architectuurlaag} activiteit={&key.activiteit} examples={&examples} />
						</Card>
					</div>
				{/for}
			{/if}
		}
	}
}

struct Description<'a> {
	pub architectuurlaag: &'a Architectuurlaag,
	pub activiteit: &'a Activiteit,
	pub examples: &'a Vec<&'a HBOIExampleResponse>,
}

impl Component for Description<'_> {
	fn to_render(&self, page: &mut Page) {
		view! {
			<section @class={"beroepsproducten_content.css"}>
				<h2>{"{:#?} {}", self.architectuurlaag, self.activiteit.to_text()}</h2>
				<hr/>
				<div>
					<ul>
						{#for example in self.examples}
							<li @class={"example_card.css"}>
								<ExampleCard guild={&example.guild} title={&example.title} />
							</li>
						{/for}
					</ul>
				</div>
			</section>
		}
	}
}

struct ExampleCard<'a> {
	pub guild: &'a Guild,
	pub title: &'a String,
}

impl<'a> Component for ExampleCard<'a> {
	fn to_render(&self, page: &mut Page) {
		view! {
			<span class="guild-tag" style={"background-color: {};", self.guild.get_color()}>{self.guild.get_short_name()}</span><span class="title">{self.title}</span>
		}
	}
}
