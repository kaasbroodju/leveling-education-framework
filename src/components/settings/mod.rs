use crate::components::card::Card;
use crate::components::icons::SettingsIcon;
use tidos::{Component, Page, scoped_css, view};

/// Runs unminified/non-`defer`red in `<head>` so the theme is applied before
/// first paint, avoiding a light/dark flash on load.
const THEME_INIT_SCRIPT: &str = "(function(){var t=localStorage.getItem('lef-theme');if(t==='light'||t==='dark'){document.documentElement.setAttribute('data-theme',t);}})();";

pub struct SettingsButton;

impl Component for SettingsButton {
	fn to_render(&self, page: &mut Page) {
		view! {
			<button id="settings-button" aria-label="Instellingen" aria-haspopup="dialog">
				<SettingsIcon />
			</button>
		}
	}
}

pub struct SettingsModal;

impl Component for SettingsModal {
	fn to_render(&self, page: &mut Page) {
		tidos::head! {
			<script>@html{THEME_INIT_SCRIPT}</script>
		}
		tidos::head! {
			<script defer>@html{include_str!("settings.js")}</script>
		}

		view! {
			<dialog id="settings-modal" closedby="any" class={scoped_css!("../content/dialog.css")}>
				<Card>
					<div class={scoped_css!("settings.css")}>
						<h2>{"Instellingen"}</h2>
						<fieldset>
							<legend>{"Thema"}</legend>
							<label><input type="radio" name="lef-theme" value="system" />{"Systeem"}</label>
							<label><input type="radio" name="lef-theme" value="light" />{"Licht"}</label>
							<label><input type="radio" name="lef-theme" value="dark" />{"Donker"}</label>
						</fieldset>
						<fieldset>
							<legend>{"Weergave niveaus"}</legend>
							<label><input type="radio" name="lef-level-view" value="all" />{"Alle niveaus"}</label>
							<label><input type="radio" name="lef-level-view" value="1" />{"Niveau 1"}</label>
							<label><input type="radio" name="lef-level-view" value="1-2" />{"Niveau 1 en 2"}</label>
							<label><input type="radio" name="lef-level-view" value="1-2-3" />{"Niveau 1, 2 en 3"}</label>
							<label><input type="radio" name="lef-level-view" value="2-3" />{"Niveau 2 en 3"}</label>
							<label><input type="radio" name="lef-level-view" value="3-4" />{"Niveau 3 en 4"}</label>
						</fieldset>
					</div>
				</Card>
			</dialog>
		}
	}
}
