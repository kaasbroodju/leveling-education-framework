use tidos::{scoped_css, view, Component, Page};

pub struct HeaderBar;

impl Component for HeaderBar {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head! {
            <link rel="preload" as="image" href="/logo_dark.svg" />
        }
        view! {
            <header class={scoped_css!("header_bar.css")}>
            <div class={scoped_css!("logo.css")}>
                <a href="/">
                    <div>
                        <picture>
                          <source srcset="/logo_dark.svg" media="(prefers-color-scheme: dark)" />
                          <img src="/logo_light.svg" alt="" />
                        </picture>

                        {"Leveling Education Framework"}
                    </div>
                </a>
            </div>
            </header>
        }
    }
}