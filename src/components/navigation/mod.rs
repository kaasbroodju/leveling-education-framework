pub mod skill_filter_matrix;
pub mod beroepstaken_filter_matrix;

use tidos::{scoped_css, view, Component, Page};

pub struct NavBar;

impl Component for NavBar {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <nav class={scoped_css!("nav_bar.css")}>
                <ul>
                    <li><a href="/" class="active">{"Vaardigheden"}</a></li>
                    <li><a href="/beroepstaken">{"Beroepstaken / HBO-i"}</a></li>
                    <li><a href="/beroepsproducten">{"Beroepsproducten"}</a></li>
                    <li><a href="/about">{"Over ons"}</a></li>
                </ul>
            </nav>
        }
    }
}