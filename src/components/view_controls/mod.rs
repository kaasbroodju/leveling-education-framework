use crate::components::icons::{MonitorIcon, MoonIcon, SunIcon};
use tidos::{Component, Page, scoped_css, view};

/// Runs unminified/non-`defer`red in `<head>` so the theme is applied (and
/// the matching icon shown) before first paint, avoiding a flash on load.
const THEME_INIT_SCRIPT: &str = "(function(){var t=localStorage.getItem('lef-theme')||'system';if(t==='light'||t==='dark'){document.documentElement.setAttribute('data-theme',t);}document.documentElement.setAttribute('data-theme-choice',t);})();";

pub struct ViewControls;

impl Component for ViewControls {
	fn to_render(&self, page: &mut Page) {
		tidos::head! {
			<script>@html{THEME_INIT_SCRIPT}</script>
		}
		tidos::head! {
			<script defer>@html{include_str!("view_controls.js")}</script>
		}

		view! {
			<div @class={"view_controls.css"}>
				<select id="level-view-select" aria-label="Weergave niveaus">
					<option value="all">{"Alle niveaus"}</option>
					<option value="1">{"Niveau 1"}</option>
					<option value="1-2">{"Niveau 1 en 2"}</option>
					<option value="1-2-3">{"Niveau 1, 2 en 3"}</option>
					<option value="2-3">{"Niveau 2 en 3"}</option>
					<option value="3-4">{"Niveau 3 en 4"}</option>
				</select>
				<button id="theme-toggle" aria-label="Thema wisselen">
					<span class="icon-system"><MonitorIcon /></span>
					<span class="icon-light"><SunIcon /></span>
					<span class="icon-dark"><MoonIcon /></span>
				</button>
			</div>
		}
	}
}
