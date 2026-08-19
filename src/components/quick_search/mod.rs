use crate::components::card::Card;
use crate::components::icons::InfoIcon;
use tidos::{Component, Page, scoped_css, view};

pub struct QuickSearch;

impl Component for QuickSearch {
	fn to_render(&self, page: &mut Page) {
		tidos::head! {
			<script>@html{include_str!("quick_search.js")}</script>
		}

		view! {
			<dialog
				id="quick-search"
				closedby="any"
				@class={"quick_search.css"}
			>
				<Card>
					<div class="search-header">
						<span><InfoIcon /></span>
						<input
							id="search-query"
							type="search"
							autocomplete="off"
							autofocus
							placeholder="Zoek in LEF..."
						/>
					</div>
					<div>
						<ul id="query-results">
						</ul>
					</div>
				</Card>
			</dialog>
		}
	}
}
