function mount() {
    let dialog = document.querySelector('#quick-search')
    addEventListener("keydown",(e) => {
        if (e.key === '/') {
            e.preventDefault();
            dialog.showModal()
        }
    })
    const searchElement = document.querySelector("input#search-query")

    searchElement.oninput = (e) => {
        const minScore = 80
        const query = e.target.value
        if (query.length >= 2) {
            const searchEngine = new SkillSearchEngine(allSkills);
            const queryResults = searchEngine.search(query);

            let results = queryResults.length === 1
                ? queryResults
                : queryResults.filter((e) => e.score >= minScore).slice(0, 5)



            const resultList = document.querySelector("#query-results")
            const children = results.map(renderChild);

            resultList.replaceChildren(...children)
        } else {
            const resultList = document.querySelector("#query-results")
            resultList.replaceChildren(...[])
        }

    }

    // Execute a function when the user presses a key on the keyboard
    searchElement.addEventListener("keypress", function(event) {
        // If the user presses the "Enter" key on the keyboard
        if (event.key === "Enter") {
            // Cancel the default action, if needed
            event.preventDefault();

            const resultList = document.querySelector("#query-results")
            if (resultList.firstElementChild) {
                window.location = resultList.firstElementChild.firstElementChild.href
            }
        }
    });
}

function renderChild(obj) {
    const node = document.createElement('li')
    const link =  document.createElement('a')
    link.href = obj.url

    const text =  document.createElement('span')
    text.innerText = obj.name
    link.appendChild(text)
    node.appendChild(link)

    return node
}

// Open-ICT Vaardigheden Database
const openICTSkills = [
    {
        name: "Overzicht creëren",
        abbreviation: "oc",
        category: "Product",
        type: "open-ict",
        description: "Als HBO-student neem je eerst een stapje terug en ga je opzoek naar wat zouden allemaal verschillende oplossingsrichtingen kunnen zijn.",
        url: "/?vaardigheid=Overzicht creëren"
    },
    {
        name: "Kritisch oordelen",
        abbreviation: "ko",
        category: "Product",
        type: "open-ict",
        description: "Als HBO-student kies je objectief een oplossingsrichting op basis van vooraf gesteld eisen.",
        url: "/?vaardigheid=Kritisch oordelen"
    },
    {
        name: "Juiste kennis ontwikkelen",
        abbreviation: "jko",
        category: "Product",
        type: "open-ict",
        description: "De student toont aan hoe hij aan de juiste kennis is gekomen om sterk in zijn/haar schoenen te staan als professional.",
        url: "/?vaardigheid=Juiste kennis ontwikkelen"
    },
    {
        name: "Kwalitatief product maken",
        abbreviation: "kpm",
        category: "Product",
        type: "open-ict",
        description: "De student toont aan dat hun product van kwaliteit is aan de hand van acceptatiecriteria en kwaliteitscriteria die je van een HBO ICT'er kan verwachten.",
        url: "/?vaardigheid=Kwalitatief product maken"
    },
    {
        name: "Plannen",
        abbreviation: "pl",
        category: "Sociaal",
        type: "open-ict",
        description: "De student is instaat om zijn eigen werk te plannen in de context van scrum/agile werken.",
        url: "/?vaardigheid=Plannen"
    },
    {
        name: "Boodschap delen",
        abbreviation: "bd",
        category: "Sociaal",
        type: "open-ict",
        description: "De student kan zijn/haar eigen werk presenteren en kennis over te dragen.",
        url: "/?vaardigheid=Boodschap delen"
    },
    {
        name: "Samenwerken",
        abbreviation: "sw",
        category: "Sociaal",
        type: "open-ict",
        description: "De student kan samenwerken in de context van scrum, in een team, gilde en bedrijfelijke context, vergelijkbaar met het Spotify model.",
        url: "/?vaardigheid=Samenwerken"
    },
    {
        name: "Flexibel opstellen",
        abbreviation: "fl",
        category: "Persoonsvormend",
        type: "open-ict",
        description: "De student is instaat om mee te bewegen in een (altijd) veranderde context.",
        url: "/?vaardigheid=Flexibel opstellen"
    },
    {
        name: "Pro-actief handelen",
        abbreviation: "ph",
        category: "Persoonsvormend",
        type: "open-ict",
        description: "De student stopt tijd en energie in ontwikkeling van zichzelf, gilde en bedrijf, en doet dit zonder instructie te krijgen van anderen.",
        url: "/?vaardigheid=Pro-actief handelen"
    },
    {
        name: "Reflecteren",
        abbreviation: "rf",
        category: "Persoonsvormend",
        type: "open-ict",
        description: "Student haalt regelmatig feedback op bij anderen en blikt terug op het scrumproces (retro's) en opzichzelf.",
        url: "/?vaardigheid=Reflecteren"
    }
];

// HBO-i Matrix (Activiteiten × Architectuurlagen)
const hboiActivities = ["Analyseren", "Adviseren", "Ontwerpen", "Realiseren", "Manage & Control"];
const hboiLayers = ["Gebruikersinteractie", "Organisatieprocessen", "Infrastructuur", "Software", "Hardwareinterfacing"];

const hboiSkills = [];
hboiLayers.forEach(layer => {
    hboiActivities.forEach(activity => {
        // Maak afkortingen
        const activityAbbr = activity.toLowerCase().split(' ').map(w => w[0]).join('');
        const layerAbbr = layer.toLowerCase().split(/(?=[A-Z])/).map(w => w[0]).join('');
        const url = new URL('/beroepstaken', window.location.origin)
        url.searchParams.set('architectuurlaag', layer)
        url.searchParams.set('activiteit', activity)

        hboiSkills.push({
            name: `${layer} - ${activity}`,
            abbreviation: `${layerAbbr}${activityAbbr}`,
            category: layer,
            subcategory: activity,
            type: "hboi",
            description: `HBO-i competentie: ${activity} toegepast op de ${layer} architectuurlaag.`,
            url: url.toString()
        });
    });
});

// Combineer alle vaardigheden
const allSkills = [...openICTSkills, ...hboiSkills];

// Zoekalgoritme
class SkillSearchEngine {
    constructor(skills) {
        this.skills = skills;
    }

    // Bereken Levenshtein distance voor fuzzy matching
    levenshteinDistance(str1, str2) {
        const matrix = [];

        for (let i = 0; i <= str2.length; i++) {
            matrix[i] = [i];
        }

        for (let j = 0; j <= str1.length; j++) {
            matrix[0][j] = j;
        }

        for (let i = 1; i <= str2.length; i++) {
            for (let j = 1; j <= str1.length; j++) {
                if (str2.charAt(i - 1) === str1.charAt(j - 1)) {
                    matrix[i][j] = matrix[i - 1][j - 1];
                } else {
                    matrix[i][j] = Math.min(
                        matrix[i - 1][j - 1] + 1,
                        matrix[i][j - 1] + 1,
                        matrix[i - 1][j] + 1
                    );
                }
            }
        }

        return matrix[str2.length][str1.length];
    }

    // Bereken fuzzy match score (0-1, waar 1 = perfecte match)
    calculateFuzzyScore(query, text) {
        const distance = this.levenshteinDistance(query.toLowerCase(), text.toLowerCase());
        const maxLength = Math.max(query.length, text.length);
        return maxLength === 0 ? 1 : 1 - (distance / maxLength);
    }

    // Zoek in vaardigheden
    search(query) {
        if (!query || query.trim().length === 0) {
            return this.skills.map(skill => ({ ...skill, score: 0, matches: [] }));
        }

        const searchTerms = query.toLowerCase().trim().split(/\s+/);
        const results = [];

        this.skills.forEach(skill => {
            let totalScore = 0;
            const matches = [];

            searchTerms.forEach(term => {
                let bestScore = 0;
                let bestMatch = null;

                // Exacte match op afkorting (hoogste score)
                if (skill.abbreviation.toLowerCase() === term) {
                    bestScore = 100;
                    bestMatch = { field: 'abbreviation', text: skill.abbreviation, type: 'exact' };
                }
                // Exacte match in naam
                else if (skill.name.toLowerCase().includes(term)) {
                    bestScore = 90;
                    bestMatch = { field: 'name', text: skill.name, type: 'contains' };
                }
                // Exacte match in categorie
                else if (skill.category.toLowerCase().includes(term)) {
                    bestScore = 70;
                    bestMatch = { field: 'category', text: skill.category, type: 'contains' };
                }
                // Exacte match in subcategorie (HBO-i)
                else if (skill.subcategory && skill.subcategory.toLowerCase().includes(term)) {
                    bestScore = 70;
                    bestMatch = { field: 'subcategory', text: skill.subcategory, type: 'contains' };
                }
                // Fuzzy match op naam
                else {
                    const nameWords = skill.name.toLowerCase().split(/\s+/);
                    nameWords.forEach(word => {
                        if (word.length >= 3) { // Alleen woorden van 3+ karakters voor fuzzy
                            const fuzzyScore = this.calculateFuzzyScore(term, word);
                            if (fuzzyScore > 0.7 && fuzzyScore * 60 > bestScore) {
                                bestScore = fuzzyScore * 60;
                                bestMatch = { field: 'name', text: word, type: 'fuzzy', score: fuzzyScore };
                            }
                        }
                    });

                    // Fuzzy match op afkorting
                    const abbrFuzzyScore = this.calculateFuzzyScore(term, skill.abbreviation);
                    if (abbrFuzzyScore > 0.6 && abbrFuzzyScore * 80 > bestScore) {
                        bestScore = abbrFuzzyScore * 80;
                        bestMatch = { field: 'abbreviation', text: skill.abbreviation, type: 'fuzzy', score: abbrFuzzyScore };
                    }
                }

                if (bestMatch) {
                    totalScore += bestScore;
                    matches.push({ term, match: bestMatch, score: bestScore });
                }
            });

            // Bonus voor het matchen van meerdere termen
            if (matches.length > 1) {
                totalScore *= 1.2;
            }

            // Bonus voor Open-ICT vaardigheden (ze zijn bekender)
            if (skill.type === 'open-ict') {
                totalScore *= 1.1;
            }

            if (totalScore > 0) {
                results.push({
                    ...skill,
                    score: totalScore,
                    matches: matches
                });
            }
        });

        // Sorteer op score (hoogste eerst)
        return results.sort((a, b) => b.score - a.score);
    }
}

setTimeout(mount, 0);