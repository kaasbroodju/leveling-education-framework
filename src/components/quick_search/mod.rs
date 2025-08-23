use tidos::{scoped_css, view, Component, Page};
use crate::components::card::Card;

pub struct QuickSearch;

impl Component for QuickSearch {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head! {
            <script>@html{include_str!("quick_search.js")}</script>
        }
        
        view! {
            <dialog id="quick-search" closedby="any" class={scoped_css!("quick_search.css")}>
                <Card content={
                view! {
                    <div class="search-header">
                        <span class="material-symbols-outlined">{"info"}</span>
                        <input id="search-query" type="search" autocomplete="off" autofocus placeholder="Zoek in LEF..." />
                    </div>
                    <div>
                        <ul id="query-results">
                        </ul>
                    </div>
                }
            }/>
            </dialog>
        }
    }
}