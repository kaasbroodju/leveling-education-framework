use crate::components::card::Card;
use crate::components::icons;
use crate::domain::{ACTIVITEITEN, ARCHITECTUURLAGEN, Icon};
use tidos::{Component, Page, scoped_css, view};

pub struct BeroepstakenFilterMatrix;

impl Component for BeroepstakenFilterMatrix {
	fn to_render(&self, page: &mut Page) -> String {
		let css = scoped_css!("beroepstaken_filter_matrix.css");
		view! {
			<header>
				<Card>
					{#slot:content}
						<div class={css}>
							{#for x in ARCHITECTUURLAGEN}
								<button data-filter-architectuurlaag={format!("{x:#?}")} aria-label={format!("{x:#?}")}>
									<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
									<span>{format!("{x:#?}")}</span>
								</button>
							{/for}
						</div>
					{/slot}
				</Card>

				<Card>
					{#slot:content}
						<div class={css}>
							{#for x in ACTIVITEITEN}
								<button
									data-filter-activiteit={format!("{x:#?}")}
									aria-label={x.to_text()}
								>
									<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
									<span>{x.to_text()}</span>
								</button>
							{/for}
						</div>
					{/slot}
				</Card>
			</header>
		}
	}
}
