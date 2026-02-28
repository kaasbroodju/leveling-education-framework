use crate::components::header::HeaderBar;
use crate::components::navigation::NavBar;
use crate::components::quick_search::QuickSearch;
use tidos::{Component, Page, view};

pub struct Layout<'a> {
	pub content: String,
	pub current_url: &'a str,
}

impl<'a> Component for Layout<'a> {
	fn to_render(&self, page: &mut Page) -> String {
		tidos::head! {
			<style>@html{include_str!("main.css")}</style>
			<link rel="icon" href="/logo_light.svg" media="(prefers-color-scheme: light)" />
			<link rel="icon" href="/logo_dark.svg" media="(prefers-color-scheme: dark)" />
			<link rel="sitemap" type="application/xml" href="/sitemap.xml" />
			<link rel="llms-txt" type="text/markdown" href="/llms.txt" />
			<meta name="description" content="LEF is een tool voor open-ict studenten om onze vaardigheden en HBO-i competenties beter te navigeren." />
			<script type="application/ld+json">@html{SEO_JSON_LINKED_DATA}</script>
			<script type="speculationrules">@html{include_str!("speculation_api.json")}</script>
		}
		tidos::head! {
			<link rel="preconnect" href="https://fonts.googleapis.com" />
			<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
			<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&display=swap" />
			<link rel="manifest" href="/manifest.json" />
			// <script>@html{"navigator.serviceWorker.register('/service-worker.js')"}</script>
		}

		view! {
			<QuickSearch />
			<div class="main-layout">
				<HeaderBar />
				<NavBar current_url={self.current_url} />
				<div class="main-container">
					<main>@html{&self.content}</main>
				</div>
			</div>
		}
	}
}

const SEO_JSON_LINKED_DATA: &'static str = include_str!("SEO_linked_data.json");
