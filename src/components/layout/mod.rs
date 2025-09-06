use std::collections::HashSet;
use tidos::{scoped_css, view, Component, Page};
use crate::components::header::HeaderBar;
use crate::components::navigation::NavBar;
use crate::components::quick_search::QuickSearch;
use crate::domain::{Icon, ACTIVITEITEN, ARCHITECTUURLAGEN, PERSONAL_SKILLS, PRODUCT_SKILLS, SOCIAL_SKILLS};

pub struct Layout<'a> {
    pub content: String,
    pub current_url: &'a str,
}



impl<'a> Component for Layout<'a> {
    fn to_render(&self, page: &mut Page) -> String {
        let mut icons = PRODUCT_SKILLS
            .iter()
            .map(Icon::to_icon)
            .chain(SOCIAL_SKILLS.iter().map(Icon::to_icon))
            .chain(PERSONAL_SKILLS.iter().map(Icon::to_icon))
            .chain(ARCHITECTUURLAGEN.iter().map(Icon::to_icon))
            .chain(ACTIVITEITEN.iter().map(Icon::to_icon))
            .chain(["info", "open_in_new", "school", "web", "face", "category", "package_2"])
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        
        icons.sort();
        
        let icons = icons.join(",");
        
        tidos::head! {
            <style>@html{include_str!("main.css")}</style>
            <link rel="icon" href="/logo_light.svg" media="(prefers-color-scheme: light)" />
            <link rel="icon" href="/logo_dark.svg" media="(prefers-color-scheme: dark)" />
            <meta name="description" content="LEF is een tool voor open-ict studenten om onze vaardigheden en HBO-i competenties beter te navigeren." />
            <script type="application/ld+json">@html{SEO_JSON_LINKED_DATA}</script>
            <script type="speculationrules">@html{include_str!("speculation_api.json")}</script>
        }
        tidos::head! {
            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
            <link rel="stylesheet" href={format!("https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,1,-1&icon_names={icons}")} />
            <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&display=swap" rel="stylesheet" />
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