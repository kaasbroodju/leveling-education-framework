pub mod skill_filter_matrix;
pub mod beroepstaken_filter_matrix;

use tidos::{scoped_css, view, Component, Page};

pub struct NavBar<'a> {
    pub current_url: &'a str,
}

impl<'a> Component for NavBar<'a> {
    fn to_render(&self, page: &mut Page) -> String {
        let nav_list = vec![
            ("Vaardigheden", "/"),
            ("Beroepstaken / HBO-i","/beroepstaken"),
            ("Beroepsproducten","/beroepsproducten"),
            ("Over ons","/about")
        ];
        
        view! {
            <nav class={scoped_css!("nav_bar.css")}>
                <ul>
                    {#for (label, href) in nav_list}
                        <li><a href={href} class={if self.current_url.eq(href) {"active"} else {""}}>{label}</a></li>
                    {/for}
                </ul>
            </nav>
        }
    }
}