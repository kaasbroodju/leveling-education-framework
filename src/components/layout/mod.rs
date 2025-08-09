use tidos::{scoped_css, view, Component, Page};
use crate::components::header::HeaderBar;
use crate::components::navigation::NavBar;

pub struct Layout {
    pub content: String
}

impl Component for Layout {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head! {
            <style>@html{include_str!("main.css")}</style>
            <link rel="icon" href="/logo_light.svg" media="(prefers-color-scheme: light)" />
            <link rel="icon" href="/logo_dark.svg" media="(prefers-color-scheme: dark)" />
        }
        view! {
            <div class="main-layout">
                <HeaderBar />
                <NavBar />
                <main>@html{&self.content}</main>
            </div>
        }
    }
}