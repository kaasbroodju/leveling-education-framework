use crate::components::icons;
use crate::domain::{Icon, PERSONAL_SKILLS, PRODUCT_SKILLS, SOCIAL_SKILLS};
use tidos::{Component, Page, scoped_css, view};

pub struct SkillFilterMatrix;

impl Component for SkillFilterMatrix {
	fn to_render(&self, page: &mut Page) {
		view! {
			<header @class={"skill_filter_matrix.css"}>
				<section>
					<h2 style="color: #8c9ebb;">{"productvaardigheden"}</h2>
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
				</section>
				<section>
					<h2 style="color: #a2b490;">{"sociale vaardigheden"}</h2>
					<div>
						{#for x in SOCIAL_SKILLS}
							<button class="social-skill" data-filter-vaardigheid={x.to_text()} aria-label={x.to_text()}>
								<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
								<span>{x.to_text()}</span>
							</button>
						{/for}
					</div>
				</section>
				<section>

					<div>
						{#for x in PERSONAL_SKILLS}
							<button class="personal-skill" data-filter-vaardigheid={x.to_text()} aria-label={x.to_text()}>
								<span style="height: 48px;">@html{icons::svg_by_name(x.to_icon())}</span>
								<span>{x.to_text()}</span>
							</button>
						{/for}
					</div>
					<h2 style="color: #d49e81;">{"persoonsvormende vaardigheden"}</h2>
				</section>
			</header>
		}
	}
}
