use crate::components::card::Card;
use crate::components::icons;
use crate::domain::Activiteit::{Adviseren, Analyseren, Manage_and_Control, Ontwerpen, Realiseren};
use crate::domain::Architectuurlaag::{
	Gebruikersinteractie, Hardwareinterfacing, Infrastructuur, Organisatieprocessen, Software,
};
use crate::domain::{ACTIVITEITEN, ARCHITECTUURLAGEN, Activiteit, Architectuurlaag, Icon, Skill};
use tidos::{Component, Page, scoped_css, view};

pub struct BeroepstakenFilterMatrix<'a> {
	pub base_url: &'a str,
	pub architectuurlaag: &'a Option<Architectuurlaag>,
	pub activiteit: &'a Option<Activiteit>,
}

impl<'a> Component for BeroepstakenFilterMatrix<'a> {
	fn to_render(&self, page: &mut Page) -> String {
		let css = scoped_css!("beroepstaken_filter_matrix.css");
		view! {
			<header>
				<Card>
					{#slot:content}
						<div class={css}>
							{#for x in ARCHITECTUURLAGEN}
								<a href={state_dependent_link(self.base_url, (&self.architectuurlaag, &self.activiteit), (Some(&x), None))} aria-label={format!("{x:#?}")} :lef-link-active={self.architectuurlaag.as_ref().map_or(false, |o| o.eq(&x))}>
									<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
									<span>{format!("{x:#?}")}</span>
								</a>
							{/for}
						</div>
					{/slot}
				</Card>

				<Card>
					{#slot:content}
						<div class={css}>
							{#for x in ACTIVITEITEN}
								<a
									href={state_dependent_link(self.base_url, (&self.architectuurlaag, &self.activiteit), (None, Some(&x)))}
									aria-label={x.to_text()}
									:lef-link-active={self.activiteit.as_ref().map_or(false, |o| o.eq(&x))}
								>
									<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
									<span>{x.to_text()}</span>
								</a>
							{/for}
						</div>
					{/slot}
				</Card>
			</header>
		}
	}
}

fn state_dependent_link(
	base_url: &str,
	(state_laag, state_activiteit): (&Option<Architectuurlaag>, &Option<Activiteit>),
	(value_laag, value_activiteit): (Option<&Architectuurlaag>, Option<&Activiteit>),
) -> String {
	match (state_laag, state_activiteit, value_laag, value_activiteit) {
		// nothing is selected
		(None, None, Some(value_laag), _) => {
			format!("{base_url}?architectuurlaag={value_laag:#?}")
		}
		(None, None, _, Some(value_activiteit)) => {
			format!("{base_url}?activiteit={value_activiteit:#?}")
		}

		// one of them is already selected
		(None, Some(state_activiteit), _, Some(value_activiteit)) => {
			if !state_activiteit.eq(&value_activiteit) {
				format!("{base_url}?activiteit={value_activiteit:#?}")
			} else {
				format!("{base_url}")
			}
		}
		(Some(state_laag), None, Some(value_laag), _) => {
			if !state_laag.eq(&value_laag) {
				format!("{base_url}?architectuurlaag={value_laag:#?}")
			} else {
				format!("{base_url}")
			}
		}

		// the other one is selected
		(None, Some(state_activiteit), Some(value_laag), _) => {
			format!("{base_url}?architectuurlaag={value_laag:#?}&activiteit={state_activiteit:#?}")
		}
		(Some(state_laag), None, _, Some(value_activiteit)) => {
			format!("{base_url}?architectuurlaag={state_laag:#?}&activiteit={value_activiteit:#?}")
		}

		// Both are already selected
		(Some(state_laag), Some(state_activiteit), Some(value_laag), _) => {
			if !state_laag.eq(&value_laag) {
				format!(
					"{base_url}?architectuurlaag={value_laag:#?}&activiteit={state_activiteit:#?}"
				)
			} else {
				format!("{base_url}?activiteit={state_activiteit:#?}")
			}
		}
		(Some(state_laag), Some(state_activiteit), _, Some(value_activiteit)) => {
			if !state_activiteit.eq(&value_activiteit) {
				format!(
					"{base_url}?architectuurlaag={state_laag:#?}&activiteit={value_activiteit:#?}"
				)
			} else {
				format!("{base_url}?architectuurlaag={state_laag:#?}")
			}
		}
		_ => {
			format!("{base_url}")
		}
	}
}
