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

const SEO_JSON_LINKED_DATA: &'static str = r########"{
  "@context": "https://schema.org",
  "@type": "WebApplication",
  "name": "LEF - Leveling Education Framework",
  "description": "Een snelle en overzichtelijke tool om door HBO-i competenties en Open-ICT vaardigheden te navigeren. Gebruikt door alle studenten en docenten van Open-ICT voor competentie-evaluatie met filter systeem en deelbare links.",
  "url": "https://lef.open-ict.hu.nl/",
  "applicationCategory": "EducationalApplication",
  "operatingSystem": "Web Browser",
  "programmingLanguage": ["Rust", "JavaScript", "CSS"],
  "runtimePlatform": ["Rocket.rs Web Framework", "Tidos Component Framework"],
  "dependencies": [
    {
      "@type": "SoftwareApplication",
      "name": "Rocket.rs", 
      "description": "Rust web framework voor backend API en routing",
      "applicationCategory": "Web Development Framework",
      "operatingSystem": "Cross-platform",
      "programmingLanguage": "Rust"
    },
    {
      "@type": "SoftwareApplication",
      "name": "Tidos",
      "description": "High-performance Rust component framework voor dynamic web applications",
      "version": "0.6.8",
      "url": "https://crates.io/crates/tidos/",
      "codeRepository": "https://github.com/kaasbroodju/tidos",
      "documentation": "https://docs.rs/tidos/latest/tidos/",
      "creator": {
        "@type": "Person",
        "name": "Morris Waaijer",
        "sameAs": "https://github.com/kaasbroodju"
      },
      "applicationCategory": "Web Development Framework",
      "operatingSystem": "Cross-platform",
      "programmingLanguage": "Rust"
    }
  ],
  "softwareRequirements": "Modern web browser",
  "storageRequirements": "Embedded binary - geen database vereist",
  "dateCreated": "2021",
  "codeRepository": "https://github.com/kaasbroodju/leveling-education-framework",
  "creator": {
    "@type": "Person",
    "name": "Luca Bergman",
    "description": "Oorspronkelijke ontwikkelaar van LEF uit ergernis over de trage en onoverzichtelijke HBO-i kubus",
    "sameAs": "https://www.linkedin.com/in/luca-bergman-30b28b203/"
  },
  "publisher": {
    "@type": "EducationalOrganization", 
    "name": "Open-ICT",
    "alternateName": "Open-ICT Leerroute",
    "description": "Agile, project-gestuurd ICT onderwijs volgens Spotify-model met squads, tribes en gildes. Winnaar tweede prijs Nederlandse Onderwijspremie 2021 (€800.000).",
    "url": "https://www.hu.nl/voltijd-opleidingen/open-ict",
    "foundingDate": "2019",
    "address": {
      "@type": "PostalAddress",
      "streetAddress": "Heidelberglaan 15",
      "addressLocality": "Utrecht",
      "postalCode": "3584 CS",
      "addressCountry": "NL"
    },
    "parentOrganization": {
      "@type": "CollegeOrUniversity",
      "name": "Hogeschool Utrecht",
      "url": "https://www.hu.nl/",
      "address": {
        "@type": "PostalAddress",
        "addressLocality": "Utrecht",
        "addressCountry": "NL"
      }
    },
    "department": {
      "@type": "EducationalOrganization",
      "name": "HBO-ICT",
      "description": "ICT bachelor opleiding van Hogeschool Utrecht"
    },
    "educationalCredentialAwarded": "Bachelor of ICT",
    "educationalProgramMode": "Full-time",
    "learningResourceType": "Project-based Learning",
    "teachingMethodology": [
      "Agile/Scrum methodology", 
      "Spotify organizational model",
      "Praktijkgericht onderwijs",
      "Peer learning",
      "Continuous feedback"
    ],
    "award": {
      "@type": "Award",
      "name": "Nederlandse Onderwijspremie 2021 - Tweede Prijs",
      "description": "Winnaar van €800.000 voor vernieuwend agile ICT-onderwijs",
      "dateReceived": "2021-03-01",
      "awarder": "Ministerie van Onderwijs, Cultuur en Wetenschap"
    }
  },
  "maintainer": [
    {
      "@type": "Person",
      "name": "Morris Waaijer", 
      "jobTitle": ["Teacher", "Coach", "Data Scientist", "Fullstack Developer", "Backend Gildemeester"],
      "affiliation": "Open-ICT, Hogeschool Utrecht",
      "startDate": "2024-08-31",
      "description": "Coach jaar 2 studenten en Backend gildemeester bij Open-ICT sinds augustus 2024",
      "sameAs": [
        "https://nl.linkedin.com/in/morris-waaijer-0894b52b7",
        "https://github.com/kaasbroodju"
      ]
    },
    {
      "@type": "Person",
      "name": "Arno Kamphuis",
      "description": "Onderhield LEF voorafgaand aan augustus 2024",
      "endDate": "2024-08-31"
    }
  ],
  "contributor": [
    {
      "@type": "Person", 
      "name": "Kevin de Meijer",
      "jobTitle": "Logo Designer",
      "contribution": "Ontwerp van het LEF logo",
      "sameAs": "https://nl.linkedin.com/in/kevindemeijer"
    },
    {
      "@type": "Person",
      "name": "Gideon Swaak", 
      "contribution": "Integratie van LEF API in Scorion platform"
    }
  ],
  "isBasedOn": {
    "@type": "EducationalFramework",
    "name": "HBO-i Domeinbeschrijving",
    "description": "Landelijke competentiestandaarden voor HBO-ICT opleidingen in Nederland, gedefinieerd door architectuurlagen en activiteiten",
    "publisher": {
      "@type": "Organization",
      "name": "Stichting HBO-i",
      "description": "Samenwerkingsverband van ICT-opleidingen in het hoger beroepsonderwijs in Nederland sinds 1992",
      "url": "https://www.hbo-i.nl/",
      "foundingDate": "1992",
      "memberOf": "57 HBO-ICT gerelateerde opleidingen",
      "mission": "ICT-onderwijs in Nederland verbeteren door verbinding en kennisdeling"
    },
    "url": "https://www.hbo-i.nl/",
    "educationalFrameworkName": "HBO-i Bachelor of ICT Domeinbeschrijving"
  },
  "audience": {
    "@type": "EducationalAudience",
    "educationalRole": ["student", "teacher", "administrator", "coach", "gildemeester"],
    "audienceType": "Open-ICT community"
  },
  "applicationSubCategory": [
    "Competentie Management",
    "Educational Assessment Tool", 
    "Learning Analytics",
    "Portfolio Support"
  ],
  "featureList": [
    "Filter HBO-i competenties en vaardigheden",
    "Deelbare links naar specifieke competenties", 
    "Overzichtelijke weergave van niveau-beschrijvingen",
    "API voor integratie met andere systemen",
    "Snelle navigatie door complexe competentieframeworks"
  ],
  "usageInfo": "Gebruikt door alle studenten en docenten van Open-ICT voor competentie-evaluatie en -ontwikkeling. Integreert met externe systemen via API en MCP (Model Context Protocol).",
  "keywords": [
    "HBO-i", "competenties", "ICT onderwijs", "Open-ICT", 
    "vaardigheden", "agile education", "scrum learning",
    "competentie-evaluatie", "Spotify model", "gildemeester",
    "backend development", "educational technology", "Rust framework",
    "Tidos", "component framework", "web development", "MCP integration"
  ],
  "inLanguage": "nl",
  "contentLocation": {
    "@type": "Place",
    "address": {
      "@type": "PostalAddress", 
      "addressLocality": "Utrecht",
      "addressCountry": "Netherlands"
    }
  }
}"########;