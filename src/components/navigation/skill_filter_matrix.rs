use tidos::{scoped_css, view, Component, Page};
use crate::domain::{Activiteit, Architectuurlaag, Skill};

pub struct SkillFilterMatrix<'a> {
    pub filter: &'a Option<Skill>
}

const PRODUCT_SKILLS: [Skill; 4] = [Skill::Overzicht_creëren, Skill::Kritisch_oordelen, Skill::Juiste_kennis_ontwikkelen, Skill::Kwalitatief_product_maken];
const SOCIAL_SKILLS: [Skill; 3] = [Skill::Plannen, Skill::Boodschap_delen, Skill::Samenwerken];
const PERSONAL_SKILLS: [Skill; 3] = [Skill::Flexibel_opstellen, Skill::Proactief_handelen, Skill::Reflecteren];
impl<'a> Component for SkillFilterMatrix<'a> {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head! {
            <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,400,1,0" />
            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
            <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&family=Ubuntu:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&display=swap" rel="stylesheet" />
        }
        view! {
            <header class={scoped_css!("skill_filter_matrix.css")}>
                <div>
                    {#for x in PRODUCT_SKILLS}
                        <a class="product-skill" href={state_dependent_link(&self.filter, &x)} aria-label={x.to_text()}>
                            <span class="material-symbols-outlined" style="font-size: 48px;">{x.to_icon()}</span>
                            <span>{x.to_text()}</span>
                        </a>
                    {/for}
                </div>
                <div>
                    {#for x in SOCIAL_SKILLS}
                        <a class="social-skill" href={state_dependent_link(&self.filter, &x)} aria-label={x.to_text()}>
                            <span class="material-symbols-outlined" style="font-size: 48px;">{x.to_icon()}</span>
                            <span>{x.to_text()}</span>
                        </a>
                    {/for}
                </div>
                <div>
                    {#for x in PERSONAL_SKILLS}
                        <a class="personal-skill" href={state_dependent_link(&self.filter, &x)} aria-label={x.to_text()}>
                            <span class="material-symbols-outlined" style="font-size: 48px;">{x.to_icon()}</span>
                            <span>{x.to_text()}</span>
                        </a>
                    {/for}
                </div>
            </header>
        }
    }
}

fn state_dependent_link(state: &Option<Skill>, value: &Skill) -> String {
    match state {
        None => {format!("/?vaardigheid={}", value.to_text())}
        Some(state) => {
            if !state.eq(value) {
                format!("/?vaardigheid={}", value.to_text())
            } else {
                "/".to_string()
            }
        }
    }
}
