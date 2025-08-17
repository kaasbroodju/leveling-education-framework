use tidos::{scoped_css, view, Component, Page};
use crate::components::card::{AboutCard, Card};

pub struct AboutLef;

impl Component for AboutLef {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div>
                <div class={scoped_css!("about_lef.css")}>
                    <Contributers />
                    <AboutLefCard />
                    <Maintainer />
                    <LEFApi />
                </div>
            </div>
        }
    }
}

struct Contributers;

impl Component for Contributers {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <AboutCard content={
                view! {
                    <div class={scoped_css!("contributers.css")}>
                        <div>
                            <h2>{"Bijdragers"}</h2>
                        </div>    
                        <img src="/miffy_usp.svg" alt=""/>
                        <div>    
                            <ul>
                                <li>
                                    <PersonSection
                                        name="Luca Bergman"
                                        text="Dankjewel Luca voor het bedenken en maken van LEF!"
                                        image={Some("fotovanmij.jpeg")}
                                        linkedIn={Some("https://www.linkedin.com/in/luca-bergman-30b28b203/")}
                                    />
                                </li>
                                <li>
                                    <PersonSection
                                        name="Kevin de Meijer"
                                        text="Dankjewel Kevin voor het maken van het prachtige LEF logo!"
                                        image={Some("kevindemeijer.jpeg")}
                                        linkedIn={Some("https://nl.linkedin.com/in/kevindemeijer")}
                                    />
                                </li>
                                <li>
                                    <PersonSection
                                        name="Gideon Swaak"
                                        text="Dankjewel Gideon voor het integreren van de LEF A.P.I. in Scorion!"
                                        image={None}
                                        linkedIn={None}
                                    />
                                </li>
                            </ul>
                        </div>
                    </div>
                }
            }/>
        }
    }
}

pub struct PersonSection<'a> {
    pub name: &'a str,
    pub text: &'a str,
    pub image: Option<&'a str>,
    pub linkedIn: Option<&'a str>,
}

impl<'a> Component for PersonSection<'a> {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div>
                <div>
                    {#if let Some(href) = self.image}
                        <img src={href} alt="" />
                    {/if}
                </div>
                <div>
                        <b>{self.name}</b>
                        <p>{self.text}</p>
                </div>
                <div>
                    {#if let Some(href) = self.linkedIn}
                        <a href={href} target="_blank"><span class="material-symbols-outlined" style="font-size: 24px;padding: 8px;">open_in_new</span></a>
                    {/if}
                </div>
            </div>
        }
    }
}

struct AboutLefCard;

impl Component for AboutLefCard {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <AboutCard content={
                view! {
                    <div class={scoped_css!("references.css")}>
                        <div>
                            <h2>{"Mede mogelijk gemaakt door"}</h2>
                        </div>
                        <picture>
                            <source srcset="/OI_White.svg" media="(prefers-color-scheme: dark)" alt="Open-ICT" />
                            <img src="/OI_Black.svg" alt="Open-ICT" />
                        </picture>
                        <div>
                            <ul>
                                <li>
                                    <a href="https://www.hu.nl/voltijd-opleidingen/open-ict" target="_blank"><span class="material-symbols-outlined">school</span><span>Opleiding</span></a>
                                </li>
                                <li>
                                    <a href="https://husite.nl/open-ict/" target="_blank"><span class="material-symbols-outlined">web</span><span><abbr title="Hogeschool Utrecht">{"HU"}</abbr>{" website"}</span></a>
                                </li>
                                <li>
                                    <a href="https://github.com/kaasbroodju/leveling-education-framework" target="_blank">
                                        <span><svg focusable="false" aria-hidden="true" viewBox="0 0 24 24"><path d="M12 1.27a11 11 0 00-3.48 21.46c.55.09.73-.28.73-.55v-1.84c-3.03.64-3.67-1.46-3.67-1.46-.55-1.29-1.28-1.65-1.28-1.65-.92-.65.1-.65.1-.65 1.1 0 1.73 1.1 1.73 1.1.92 1.65 2.57 1.2 3.21.92a2 2 0 01.64-1.47c-2.47-.27-5.04-1.19-5.04-5.5 0-1.1.46-2.1 1.2-2.84a3.76 3.76 0 010-2.93s.91-.28 3.11 1.1c1.8-.49 3.7-.49 5.5 0 2.1-1.38 3.02-1.1 3.02-1.1a3.76 3.76 0 010 2.93c.83.74 1.2 1.74 1.2 2.94 0 4.21-2.57 5.13-5.04 5.4.45.37.82.92.82 2.02v3.03c0 .27.1.64.73.55A11 11 0 0012 1.27"></path></svg></span>
                                        <span>Github</span></a>
                                </li>
                            </ul>
                        </div>
                    </div>
                }
            }/>
        }
    }
}

pub struct Maintainer;

impl Component for Maintainer {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <AboutCard content={
                view! {
                    <div class={scoped_css!("references.css")}>
                        <div>
                            <h2>{"Maintainer: Morris Waaijer"}</h2>
                        </div>
                        <div>
                            <ul>
                                <li>
                                    <a href="https://github.com/kaasbroodju" target="_blank">
                                        <span><svg focusable="false" aria-hidden="true" viewBox="0 0 24 24"><path d="M12 1.27a11 11 0 00-3.48 21.46c.55.09.73-.28.73-.55v-1.84c-3.03.64-3.67-1.46-3.67-1.46-.55-1.29-1.28-1.65-1.28-1.65-.92-.65.1-.65.1-.65 1.1 0 1.73 1.1 1.73 1.1.92 1.65 2.57 1.2 3.21.92a2 2 0 01.64-1.47c-2.47-.27-5.04-1.19-5.04-5.5 0-1.1.46-2.1 1.2-2.84a3.76 3.76 0 010-2.93s.91-.28 3.11 1.1c1.8-.49 3.7-.49 5.5 0 2.1-1.38 3.02-1.1 3.02-1.1a3.76 3.76 0 010 2.93c.83.74 1.2 1.74 1.2 2.94 0 4.21-2.57 5.13-5.04 5.4.45.37.82.92.82 2.02v3.03c0 .27.1.64.73.55A11 11 0 0012 1.27"></path></svg></span>
                                        <span>Github</span>
                                    </a>
                                </li>
                                <li>
                                    <a href="https://nl.linkedin.com/in/morris-waaijer-0894b52b7" target="_blank">
                                        <span>
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 50 50" width="24px" height="24px"><path d="M41,4H9C6.24,4,4,6.24,4,9v32c0,2.76,2.24,5,5,5h32c2.76,0,5-2.24,5-5V9C46,6.24,43.76,4,41,4z M17,20v19h-6V20H17z M11,14.47c0-1.4,1.2-2.47,3-2.47s2.93,1.07,3,2.47c0,1.4-1.12,2.53-3,2.53C12.2,17,11,15.87,11,14.47z M39,39h-6c0,0,0-9.26,0-10 c0-2-1-4-3.5-4.04h-0.08C27,24.96,26,27.02,26,29c0,0.91,0,10,0,10h-6V20h6v2.56c0,0,1.93-2.56,5.81-2.56 c3.97,0,7.19,2.73,7.19,8.26V39z"/></svg>
                                        </span>
                                        <span>LinkedIn</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                }
            } />
        }
    }
}

pub struct LEFApi;

impl Component for LEFApi {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <AboutCard content={
                view! {
                    <div class={scoped_css!("references.css")}>
                        <div>
                            <h2>{"LEF API"}</h2>
                        </div>
                        <div>
                            <ul>
                                <li>
                                    <a href="/api/v1/vaardigheden" target="_blank"><span class="material-symbols-outlined">face</span><span>Vaardigheden</span></a>
                                </li>
                                <li>
                                    <a href="/api/v1/hboi" target="_blank"><span class="material-symbols-outlined">category</span><span>Beroepstaken / HBO-i</span></a>
                                </li>
                                <li>
                                    <a href="/api/v1/beroepsproducten" target="_blank"><span class="material-symbols-outlined">package_2</span><span>Beroepsproducten</span></a>
                                </li>
                            </ul>
                        </div>
                    </div>
                }
            } />
        }
    }
}