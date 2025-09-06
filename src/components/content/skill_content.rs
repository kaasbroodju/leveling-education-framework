use std::collections::BTreeMap;
use markdown::to_html;
use tidos::{scoped_css, view, Component, Page};
use crate::components::card::Card;
use crate::components::navigation::skill_filter_matrix::SkillFilterMatrix;
use crate::data::SKILL_DATA;
use crate::domain::{Level, LevelDescription, Skill, VaardighedenResponseBody};

pub struct SkillContent {
    pub filter: Option<Skill>
}

impl Component for SkillContent {
    fn to_render(&self, page: &mut Page) -> String {
        let content = &(*SKILL_DATA);
        
        view! {
            <Card content={
                view!{ <SkillFilterMatrix filter={&self.filter} /> }
            } />
            {#for (skill, levels) in content
                .iter()
                .filter(|(s, _)| self.filter.as_ref().map_or(true, |f| f == *s))
            }
                <Card content={
                    view!{ <Description title={skill.to_text().to_string()} levels={levels} />}
                } />
            {/for}
        }
    }
}

struct Description<'a> {
    pub levels: &'a BTreeMap<Level, LevelDescription>,
    pub title: String,
}

impl Component for Description<'_> {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head! {
            <script defer>@html {include_str!("skill_content.js")}</script>
        }
        view!{
            <section class={scoped_css!("skill_content.css")}>
                <h2>{format!("{}", self.title.replace('_', " "))}</h2>
                <hr/>
                <div>
                    {#for (level, description) in self.levels.iter()}
                        <section>
                            <div class="skill-header">
                                <h3>{level.to_text()}</h3>
                                {#if let Some(x) = &description.info}
                                    <button lef-modal={format!("{}-{:#?}", self.title, level)}>
                                        <span class="material-symbols-outlined">{"info"}</span>
                                    </button>
                                {/if}
                                
                            </div>
                            <p>{&description.title}</p>
                            {#if let Some(x) = &description.info}
                                <dialog class={scoped_css!("dialog.css")} id={format!("{}-{:#?}", self.title, level)} lef-modal closedby="any">
                                    <Card content={view! {
                                        <h2>{format!("Extra toelichting {} {}" , self.title.replace('_', " ").to_lowercase(), level.to_text().to_lowercase())}</h2>
                                        @html{to_html(x)}}
                                    }/>
                                </dialog>
                            {/if}
                            
                            // @html{markdown::to_html(&description.title)}
                        </section>
                    {/for}
                </div>
            </section>
        }
    }
}