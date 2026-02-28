use crate::components::icons;
use crate::domain::{Icon, PERSONAL_SKILLS, PRODUCT_SKILLS, SOCIAL_SKILLS};
use tidos::{Component, Page, scoped_css, view};

pub struct SkillFilterMatrix;

impl Component for SkillFilterMatrix {
	fn to_render(&self, page: &mut Page) -> String {
		view! {
			<header class={scoped_css!("skill_filter_matrix.css")}>
				<div>
					{#for x in PRODUCT_SKILLS}
						<button
							class="product-skill"
							data-filter-vaardigheid={x.to_text()}
							aria-label={x.to_text()}
						>
							<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
							<span>{x.to_text()}</span>
						</button>
					{/for}
				</div>
				<div>
					{#for x in SOCIAL_SKILLS}
						<button class="social-skill" data-filter-vaardigheid={x.to_text()} aria-label={x.to_text()}>
							<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
							<span>{x.to_text()}</span>
						</button>
					{/for}
				</div>
				<div>
					{#for x in PERSONAL_SKILLS}
						<button class="personal-skill" data-filter-vaardigheid={x.to_text()} aria-label={x.to_text()}>
							<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
							<span>{x.to_text()}</span>
						</button>
					{/for}
				</div>
			</header>
		}
	}
}
