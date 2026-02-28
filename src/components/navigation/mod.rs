pub mod beroepstaken_filter_matrix;
pub mod skill_filter_matrix;

use crate::components::icons;
use tidos::{Component, Page, scoped_css, view};

pub struct NavBar<'a> {
	pub current_url: &'a str,
}

impl<'a> Component for NavBar<'a> {
	fn to_render(&self, page: &mut Page) -> String {
		let nav_list = vec![
			("Vaardigheden", "/", icons::FACE_SVG),
			("Beroepstaken / HBO-i", "/beroepstaken", icons::CATEGORY_SVG),
			(
				"Beroepsproducten",
				"/beroepsproducten",
				icons::PACKAGE_2_SVG,
			),
			("Over ons", "/about", icons::INFO_SVG),
		];

		view! {
			<nav class={scoped_css!("nav_bar.css")}>
				<ul>
					{#for (label, href, icon_svg) in nav_list}
						<li>
							<a href={href} class={if self.current_url.eq(href) {"active"} else {""}}>
								<span>@html{icon_svg}</span>
								<span>{label}</span>
							</a>
						</li>
					{/for}
				</ul>
			</nav>
		}
	}
}
