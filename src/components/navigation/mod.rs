pub mod skill_filter_matrix;
pub mod beroepstaken_filter_matrix;

use tidos::{scoped_css, view, Component, Page};

pub struct NavBar<'a> {
    pub current_url: &'a str,
}

impl<'a> Component for NavBar<'a> {
    fn to_render(&self, page: &mut Page) -> String {
        let nav_list = vec![
            ("Vaardigheden", "/", "face"),
            ("Beroepstaken / HBO-i","/beroepstaken", "category"),
            ("Beroepsproducten","/beroepsproducten", "package_2"),
            ("Over ons","/about", "info")
        ];
        
        view! {
            <nav class={scoped_css!("nav_bar.css")}>
                <ul>
                    {#for (label, href, icon) in nav_list}
                        <li>
                            <a href={href} class={if self.current_url.eq(href) {"active"} else {""}}>
                                <span class="material-symbols-outlined">{icon}</span>
                                <span>{label}</span>
                            </a>
                        </li>
                    {/for}
                </ul>
            </nav>
        }
    }
}