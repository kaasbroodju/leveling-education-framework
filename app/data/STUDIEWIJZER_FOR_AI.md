# Leveling Education Framework

> Leveling Education Framework, meestal LEF genoemd, is een website die het voor studenten van Open-ICT, de duale HBO-ICT-opleidingen (Software Development en Cyber Security & Cloud) en de master Human-Centered Artificial Intelligence — alle drie van Hogeschool Utrecht — inzichtelijker maakt om de LEF-vaardigheden en HBO-I beroepstaken te navigeren.

> **Let op**: dit document beschrijft primair de Open-ICT werkwijze (squads, tribes, gildes, coaches, gildemeesters, Portflow). De duale HBO-ICT-opleidingen volgen een vergelijkbare opzet. De master Human-Centered Artificial Intelligence wordt hier (nog) niet in meegenomen.

## Open-ICT in vogelvlucht

Open-ICT draait om werken aan wat goed is voor de ontwikkeling van de **individuele student** én wat goed is voor **onze wereld (People/Planet)**.

### Vaardigheden ontwikkelen

Als HBO-ICT student krijgt de student de regie over het ontwikkelen van eigen kennis en vaardigheden, op drie vlakken:
- **Persoonsvormende vaardigheden**: jezelf leren kennen en verantwoordelijkheid leren dragen (je eigen "gebruiksaanwijzing")
- **Productvaardigheden**: ICT-producten kunnen maken waarmee je later je brood verdient of een eigen onderneming opbouwt
- **Sociale vaardigheden**: samenwerken zodat je bijdraagt aan het grotere geheel van Open-ICT, je organisatie en de wereld

### Beroepsrol kiezen en verantwoordelijkheid oppakken

Na een verkennend eerste semester kiest de student elk semester een beroepsrol (dezelfde of een andere), die telkens een deel van het ICT-werkveld bestrijkt. In teams bouwen studenten samen complete ICT-oplossingen: de student neemt verantwoordelijkheid voor de eigen rol en blijft tegelijk verantwoordelijk voor het geheel. Elke beroepsrol kent een bijpassend gilde waarin de student hulp krijgt en geeft om de kwaliteit van het werk te verhogen.

### Steeds uitdagender opdrachten

Gedurende de opleiding ontwikkelen studenten hun vaardigheden door opdrachten te zoeken die passen bij hun beroepsrol en steeds uitdagender worden — in zelfstandigheid, in complexiteit van het product en van de omgeving. Drie niveaus, die ongeveer overeenkomen met de studiejaren:
- **Taakgericht** — jaar 1
- **Probleemgericht** — jaar 2
- **Situatiegericht** — jaar 3/4

Door een bij het eigen niveau passende uitdaging ontstaat flow.

## Ontwikkelflow: het werkingsprincipe

Bij Open-ICT ontwikkelen studenten hun vaardigheden via de ontwikkelflow: ze zoeken telkens ICT-uitdagingen die net boven hun huidige niveau liggen, in de zone tussen saai en stress. Daar ontstaat ruimte om passende oplossingen te vinden en vaardigheden te versterken.

Flow ontstaat door heldere doelen, directe feedback en een goede balans tussen vaardigheden en uitdaging — precies waarop wordt gestuurd in de **sprints van twee weken**. In flow creëren studenten met vallen en opstaan steeds meer producten met kwaliteit, en ontwikkelen ze vertrouwen in hun eigen vaardigheden en die van teamgenoten.

**Exponentiële groei**: ontwikkelflow levert iedere sprint een relaxte, exponentiële groei op. Groeit een student bijvoorbeeld 4% per sprint (heel gebruikelijk), dan telt dat over tien sprints in een semester op tot zo'n 50% groei. Ook als het niet elke sprint lukt, blijft de groei op de lange termijn fors.

**Rol van docenten**: de kans op ontwikkelflow wordt vergroot vanuit twee rollen:
- **Coach** — richt zich op persoonsvormende en sociale vaardigheden
- **Gildemeester** — richt zich op productvaardigheden

Beide fungeren als ervaren klankbord en stellen zichzelf steeds dezelfde vraag: *verhoogt mijn ingreep de kans dat de student leert?* Soms betekent dat even helpen, vaak ook extra uitdagen om het zelf goed uit te zoeken.

## Ontwikkelmethodiek (Scrum-gebaseerd)

Open-ICT gebruikt Scrum als basis voor de ontwikkelmethodiek, aangevuld met elementen die het geschikt maken voor leren: studenten ontwikkelen zo zowel ICT-producten als zichzelf. (Grotendeels gebaseerd op de Scrum Guide (2020), aangevuld met Open-ICT-specifieke elementen.)

### Van uitdaging naar roadmap

Elk semester begint met het verkennen van de uitdaging in de opdracht: wie heeft een probleem, waarom moet het opgelost worden, wat zijn mogelijke oplossingen. Dit gebeurt in een **design sprint** met drie fasen: (1) probleemdefinitie, (2) van idee naar prototype, (3) de systemen inrichten (backlog + roadmap) voor de eerste echte sprintplanning. Elke fase begint met **overzicht creëren** (opties bedenken) en sluit af met **kritisch oordelen** (die opties beoordelen).
- Jaar 2: designsprint van 3 weken, met hulpmiddelen zoals een Blinkli project-kickoff en een Miro-werkblad; de Scrum Master zorgt dat het team deze werkwijze volgt.
- Jaar 3/4 (situatiegericht): de ontwikkelmethodiek wordt meer bepaald door de aard van de opdracht en de bedrijfscontext — niet per se een volledige designsprint. Andere veelvoorkomende methodieken: Agile/Scrum (Software Development), Agile/Kanban (Ops/continu werk), ITIL & Ticketbeheer (ITSM/Operations), Plan-Based/Waterval, Research methodiek — elk met eigen terminologie, aanpak en voor-/nadelen; in de praktijk ontstaan ook hybride vormen. De Open-ICT Ontwikkelmethodiek dient hierbij als referentiepunt.

**Backlog**: een levende, geordende lijst van wat nodig is om het product te verbeteren — de enige bron van werk voor teamleden. Bovenaan staat het belangrijkste. Opgebouwd vanuit de productvisie (ontstaan in de design sprint), opgesplitst in:
- **Epic** — een grotere klus over meerdere sprints, op abstract niveau de "reis" van de gebruiker
- **Feature** — tussenstations binnen die reis
- **Story** — beschrijft wat er daadwerkelijk gemaakt wordt, in het format *"Als …, wil ik …, zodat …"*, met acceptatie- en kwaliteitscriteria
- Type epics naar wie er waarde uit haalt: **User epic** (waarde voor de gebruiker), **Research epic** (grotere uitzoekklus, waarde vooral voor de Product Owner), **Enabler epic** (inrichten van de eigen werkomgeving, bijv. Jira/DevOps of GitHub + CI/CD), **Learning epic** (leerwaarde voor de student — per student een eigen feature met learning stories + evaluatie-story's; dit vormt in wezen de **semester roadmap** van die student)
- Kwaliteitscriteria voor het backlog: **DEEP** (jaar 1) en **INVEST** (vanaf jaar 2)

**Roadmap**: een heel semester aan sprints; de eerstkomende sprint is volledig uitgewerkt, latere sprints bevatten grotere/vagere story's.

### De sprintcyclus (elke 2 weken)

1. **Roadmap & Backlog Refinement** (PL, OC, JKO, KO) — doorlopend de epics/features/story's verder verkleinen en preciseren (beschrijving, AC/KC, volgorde, inschatting). Story's die binnen een sprint 'Done' gemaakt kunnen worden, zijn klaar voor sprintplanning. Developers schatten zelf de omvang in; de Product Owner mag hen daarbij helpen afwegingen te maken, niet de inschatting overnemen. Een story vraagt vaak **vertical slicing** (UX + front-end + back-end samen), niet horizontaal per laag. Naast User Stories bestaan er **Learning Stories**: persoonlijke leerdoelen die een student afleidt uit de eigen taken, met bijbehorende leertaken, ook opgenomen in de sprintplanning.
2. **Sprint Planning** (PL, OC) — door het hele team; behandelt drie vragen: (1) *waarom* is deze sprint waardevol → het **Sprint Doel**, voorgesteld door de PO en vastgesteld door het team vóór het einde van de planning; (2) *wat* kan afgerond worden → Developers selecteren story's in overleg met de PO (Plan Poker helpt bij het inschatten); (3) *hoe* gebeurt het werk → Developers breken story's zelf op in taken van een dag of minder; niemand anders bepaalt hoe. Sprint Doel + geselecteerde story's + uitvoeringsplan = de **Sprint Backlog**.
3. **Check-in / check-out** (OC, JKO, KPM) — elke werkdag start met een check-in (motivatie, wat gedaan, wat vandaag, hulpvragen) en eindigt met een check-out; voortgang t.o.v. het Sprint Doel wordt geïnspecteerd en de Sprint Backlog zo nodig aangepast. Verbetert communicatie, signaleert blokkades snel en vermindert de noodzaak voor andere overleggen.
4. **Peer review en Sprint Review** (KO) — de **peer review** is de laatste taak van elke story: een peer-student of peer-expert beoordeelt het werk voordat het naar de opdrachtgever gaat; feedback wordt verwerkt in de story en kan als bewijs dienen. De **Sprint Review** is een werksessie (geen presentatie) waarin het team de sprintresultaten — het liefst met een werkende demo, ook als die nog niet af is — toont aan belanghebbenden; samen wordt bepaald wat de volgende stap is, en het backlog kan worden aangepast op nieuwe inzichten.
5. **Retrospective** (RE) — het team blikt terug op teamleden, interacties, processen, tools en de Definition of Done: wat ging goed, welke problemen kwamen op, hoe (niet) opgelost. De meest impactvolle verbeteringen worden opgepakt, eventueel direct in de volgende Sprint Backlog. Ook ruimte voor persoonlijke feedback (samenwerken, pro-actief handelen, flexibel opstellen), bruikbaar als ontwikkeldoel of bewijs. (Hulpmiddel: Retromat.)
6. **Ontwikkelgesprek** (RE, PL) — elke sprint een individueel gesprek met de coach, voorbereid met eigen reflectie en ontvangen feedback. Onderwerpen: hoe gaat het, zit je in flow (genoeg nuttige feedback?), en groei je voldoende in niveau om het semester te halen. De coach helpt met keuzes, leerdoelen, feedback op de 10 vaardigheden, voortgang van portfolio-datapunten, doorverwijzing naar mentoren/gildes, en professionele/persoonlijke ontwikkeling (tijdmanagement, presenteren, zelfredzaamheid) of studieproblemen (bij ernstige zaken eventueel doorverwijzing naar studentenpsycholoog of decaan).
   - **On-track gesprek** (week 10-12): tussentijdse feedback of de student op koers ligt, op basis van hoeveelheid/niveau van evaluaties, balans uitdaging-vaardigheden, voortgang op plandoelen, en het regelmatig ophalen én geven van feedback. Vastgelegd in het portfoliosysteem.

### Waarom deze cyclus vaardigheden ontwikkelt

Elke sprint doorloop je impliciet de LEF-vaardigheden: acceptatie-/kwaliteitscriteria opstellen hoort bij **plannen**; de backlog en context uitzoeken bij **overzicht creëren** en **juiste kennis ontwikkelen**; bouwen binnen de criteria bij **kwalitatief product maken**; toetsen of het werk aan de criteria voldoet (zelf en via peer review/acceptatietest) ook bij **kritisch oordelen**; documenteren en demonstreren in de sprint review bij **boodschap delen**.
- Jaar 2 werkt hierbij methodisch en onderzoekend met het **DOT-onderzoeksframework**.
- Jaar 3/4: het DOT-framework (ICT-researchmethods) past goed bij praktijkgericht onderzoek; er bestaan varianten voor andere domeinen en veel andere onderzoekstradities, elk gekleurd door hun eigen werkveld.

### Teamrollen

Een Open-ICT team is een kleine, zelfsturende, multidisciplinaire eenheid zonder subteams of hiërarchie, gericht op één Product Doel: één Scrum Master, één Product Owner, en Developers die elk hun eigen beroepsrol inbrengen. Het hele team is elke sprint samen verantwoordelijk voor een waardevol, bruikbaar increment.

- **Developers** (de beroepsrollen) — samen verantwoordelijk voor: meewerken aan roadmap/backlog refinement, het Sprint Backlog-plan maken, kwaliteit borgen via de Definition of Done, het plan dagelijks bijstellen richting het Sprint Doel, en elkaar aanspreekbaar houden.
- **Product Owner** — maximaliseert de productwaarde en beheert het backlog: user journey/gebruikerswensen helder krijgen, stakeholders in kaart brengen en onderhouden, teamontwikkeling volgen, de productvisie ontwikkelen en overbrengen, backlog-items creëren/ordenen/prioriteren, kwaliteitscontrole (Definition of Done) en budgetcontrole. Eén persoon, geen comité — kan werk delegeren maar blijft eindverantwoordelijk. *De PO-rol vormt de kern van de BIT-beroepsrol, maar is ook vanuit een andere beroepsrol op te pakken; voor de kwaliteit ervan is het belangrijk hulp en feedback te vragen/geven in het BIT-gilde.*
  - Jaar 1 — **PO-Scribe**: ondersteunt de coach, die feitelijk de PO van alle teams in de tribe is; zorgt dat PO-producten op orde zijn en neemt initiatief. Gaandeweg het jaar neemt de student steeds meer PO-werk van de coach over.
  - Jaar 2 — **PO by Proxy**: vervangt de daadwerkelijke (externe) PO in het team op productvisie, prioritering/planning, stakeholdercommunicatie en kwaliteit/budgetcontrole; gedrag: verantwoordelijkheid nemen, feedback vragen, de lat hoog leggen.
  - Jaar 3/4 — **daadwerkelijke PO of PO-assistent**, bij Open Innovatie of als stagiair.
- **Scrum Master** — richt de Open-ICT ontwikkelmethodiek in het team in en helpt iedereen Scrum-theorie en -praktijk begrijpen. Dient het team door te coachen in zelfsturing/multidisciplinair werken, te focussen op waardevolle increments die aan de Definition of Done voldoen, belemmeringen weg te nemen, en te zorgen dat alle Scrum-events plaatsvinden (positief, productief, binnen tijd). Dient de PO door te helpen bij effectieve Product Doel-definitie en backlog-management, en door samenwerking met stakeholders te faciliteren.
  - Jaar 1 — de coach is de Scrum Master ("Super Scrum Master") van alle teams in de tribe; teamleden nemen dit werk gaandeweg over.
  - Jaar 2 — een teamlid pakt de Scrum Master-rol op; de coach denkt als Super Scrum Master mee en coacht de student hierin.
  - Jaar 3/4 — bij Open Innovatie-projecten pakt een teamlid de rol op (coach coacht mee); op stage krijgen sommige studenten de kans om Scrum Master van één of meerdere teams te zijn.

## Opbouw per jaar

### Jaar 1

Het eerste jaar werken studenten intern op de HU op een werkvloer die zoveel mogelijk de ICT-ontwikkelafdeling van een organisatie nabootst. Studenten werken samen in door de opleiding samengestelde teams van 4-6 studenten aan diverse opdrachten, verdeeld over drie fasen:

1. **Kennismaking Open-ICT** (eerste 8 weken) — kennismaking met elkaar en met de concepten van Open-ICT, vanuit de waarden 'leren door doen' en 'samen leren'. Vier sprints: (1) elkaar goed leren kennen, (2) de agile werk- en leerwijze doorleven, (3) een beter beeld krijgen van wat een ICT'er doet, en (4) je bewuster worden van de vaardigheden die je inzet om je doelen te bereiken. Ook aandacht voor feedback geven/ontvangen en ontwikkeldoelen stellen. Na deze 8 weken laat de student oude schoolsystemen los en is klaar voor het eerste project.
2. **Boerencamping** (volgende 12 weken) — studenten werken samen aan het realiseren van een complete applicatie voor de fictieve opdrachtgever Boer Bert, die een boerencamping begint en daarvoor ICT-oplossingen nodig heeft (bijv. camping-wifi, een online boekingssysteem, chip-aangestuurde douches, of een slim reserveringsalgoritme). Een squad van vijf tot zes studenten kiest zelf welke richting ze willen uitproberen.
3. **Het eerste echte project** — na het oriënterende eerste half jaar kiest de student een nieuw project voor een heel semester, uit een door de coaches samengestelde lijst (of via een eigen pitch). Iedere student heeft al een beroepsrol (bijv. front-end developer, business & IT developer). Coaches fungeren in dit project vaak nog als product owner en bepalen grotendeels wat er gemaakt wordt; studenten met meer inhoudelijke kennis en een goede professionele houding mogen al zelfsturender werken.

### Jaar 2

- **Externe projecten**: studenten werken extern in een ontwikkelteam voor een door de opleiding geworven opdrachtgever. Op basis van studentvoorkeur wordt een passende indeling van projecten gemaakt. Binnen het team wordt de taakverdeling zo besproken dat iedereen zich verder kan ontwikkelen in de eigen beroepsrol, al is het ook nodig om taken buiten de beroepsrol op te pakken. Studenten werken minimaal één en maximaal drie dagen per week op locatie bij de opdrachtgever.
- **Gilde**: daarnaast werken studenten één dagdeel per week in het gilde op de HU, waar onder andere feedback wordt opgehaald over producten uit de projecten. Vanuit het gilde kunnen aanvullende **rodedraad-projecten** ontstaan — meestal producten die van belang zijn voor de ontwikkeling van de beroepsrol maar niet nodig zijn in het lopende project. De extra tijdsbesteding hiervoor is maximaal een halve dag per week (in overleg met de coach kan hiervan worden afgeweken). De stories uit rodedraad-projecten worden ook meegenomen in de planning van het projectteam, zodat zicht blijft op de inzet en totale capaciteit van het team.

### Jaar 3/4: Open Innovatie, profileringsruimte en afstuderen

In jaar 3 en 4 kiezen studenten uit de volgende invullingen:

- **Open Innovatie: Project** — studenten werken in teamverband aan projecten gericht op het starten van een eigen bedrijf of een actieve bijdrage aan de Open Source gemeenschap. Locatie: Nederland. Periode: jaar 3 (semester 5 en 6) of jaar 4 (semester 7).
- **Open Innovatie: Stage** — studenten gaan individueel aan de slag bij een externe opdrachtgever. Locatie: binnen- of buitenland. Periode: jaar 3 (semester 5 en 6) of jaar 4 (semester 7).
- **Invulling profileringsruimte** — de profileringsruimte (zoals een minor) geeft studenten eenmalig een half jaar vrije keuze in wat ze willen studeren. Elke hogeschool en universiteit heeft een breed aanbod van minoren; het is ook mogelijk om, na goedkeuring door de Examencommissie, zelf een minor samen te stellen. Meer info: zie "Invulling Profileringsruimte" in de studiegids.
- **Open Innovatie: studeren in het buitenland** — een internationaliseringsvariant binnen Open Innovatie.
- **Afstuderen** — studenten doen hun meesterproef individueel bij een externe opdrachtgever, binnen- of buitenland, in jaar 4 (semester 8). Kent eigen regels (zie de aparte afstudeerleidraad/cursus Afstuderen).

## Organisatiestructuur

Studenten werken en leren in een netwerk: een student heeft een plaats in verschillende teams met medestudenten, een coach, een gildemeester en contacten met opdrachtgevers en de buitenwereld.

### Squad (ontwikkelteam)
- Een student werkt elke werkdag samen in een ontwikkelteam met ongeveer **vijf andere studenten** aan inhoudelijke projecten voor een opdrachtgever
- Iedere student heeft een eigen gekozen beroepsrol en werkt zelfstandig aan deeltaken die terugkomen in een gezamenlijk eindproduct

### Tribe (leergemeenschap met diverse ontwikkelteams)
- Meerdere squads zijn verenigd in een tribe; studenten in één tribe zijn gezamenlijk ingedeeld op de werkvloer van Open-ICT
- **Eerstejaars tribes** komen minimaal **vier keer per week** een dagdeel op locatie; **ouderejaars** minimaal **één dagdeel**

### Begeleiding door coach
Iedere student krijgt een vaste coach gedurende een studieperiode, die zowel de professionele ontwikkeling van de student als de teamontwikkeling ondersteunt:
- Is tijdens tribe-momenten aanwezig als vast gezicht en aanspreekpunt
- Woont regelmatig sprint events bij en geeft feedback
- Spreekt teams over de samenwerking
- Geeft waar nodig feedback op gedrag en vaardigheden
- Voert **eens per twee weken** een individueel coachinggesprek

De coach helpt daarnaast bij studiekeuzes, studieproblemen, focus houden en het behalen van de HBO-i beroepstaken binnen de gekozen beroepsrol.

### Gilde (specialisatie)
Vanaf jaar 2 maken studenten deel uit van een gilde, gericht op samen leren en kennis opbouwen die hoort bij een specifieke beroepsrol. Ieder semester kiest de student een beroepsrol; gildeleden ontmoeten elkaar wekelijks op locatie. Binnen een gilde bestaan leerteams met studenten uit verschillende studiejaren en kennisniveaus, begeleid door een **gildemeester** — een docent-expert binnen het vakgebied.

Zes kernactiviteiten van het gilde:
1. **Sociale activiteiten voor teambuilding** — versterken van onderlinge relaties en een positieve groepsdynamiek
2. **Intervisie** — gezamenlijk bespreken van vraagstukken, dilemma's en uitdagingen uit de praktijk
3. **Product previews** — studenten tonen werk aan elkaar en ontvangen feedback op product en proces
4. **Kennisdelingen** — studenten verzorgen presentaties, workshops, blogs of andere vormen van kennisoverdracht
5. **Content curatie en vastlegging (BoKS)** — belangrijke kennis wordt verzameld, vastgelegd en gedeeld binnen het gilde
6. **Rode draad-projecten** — extra projecten die bijdragen aan de beroepsontwikkeling naast het reguliere projectwerk

## Beroepsrollen en beroepstaken

In het eerste semester verkennen studenten wat er allemaal in de ICT te doen is; daarna kiezen ze elk semester een beroepsrol. Ter inspiratie en referentie voor de beroepsrollen wordt gebruikgemaakt van de HBO-i beroepstaken en voorbeeldproducten.

Open-ICT kent (dit collegejaar) negen beroepsrollen en bijpassende gildes:
- **UI/UX designer** — analyseert gebruikersbehoeften, ontwerpt intuïtieve interfaces en optimaliseert oplossingen voor naadloze gebruikerservaringen
- **Business IT engineer** — vertaalt behoeftes van business en eindgebruikers naar bruikbare ICT-oplossingen via configureerbare platforms (low-code)
- **Cyber Security engineer** — ontwerpt, implementeert en onderhoudt beveiligingsmaatregelen om digitale systemen te beschermen
- **Cloud en Infra engineer** — ontwerpt, automatiseert, implementeert en beheert moderne IT-infrastructuren
- **Front-end developer** — bouwt de zichtbare kant van ICT-oplossingen op basis van ontwerpen en business requirements
- **Back-end developer** — bouwt en beheert server, database en logica, ontwikkelt API's voor communicatie met de front-end
- **AI engineer** — bedenkt, bouwt en visualiseert slimme algoritmen die business processen verder automatiseren
- **Game developer** — ontwerpt en bouwt interactieve games en simulaties met behulp van gespecialiseerde game-engines
- **Embedded Technology engineer** — ontwerpt en programmeert software die direct op hardware draait en systemen die slim reageren op hun omgeving

Zie de `beroepsrollen`-endpoints hieronder voor de volledige beschrijving per rol (omschrijving, roadmap per niveau, voorbeeldproducten en voorbeeldfuncties).

**HBO-i beroepstaken**: een beroepstaak is de combinatie van een architectuurlaag en een activiteit. Het model kent vijf architectuurlagen (gebruikersinteractie, organisatieprocessen, infrastructuur, software, hardwareinterfacing) en vijf activiteiten (analyseren, adviseren, ontwerpen, realiseren, manage & control), elk uitgewerkt in vier niveaus. Een ICT-project omvat altijd de volle breedte van de vijf activiteiten, in samenhang uitgevoerd. Binnen een beroepsrol werkt een student aan een specifieke hoofdlaag en daarnaast aan één of meer extra lagen om zich te verbreden of te verdiepen — zo ontstaat een eigen kleuring van de ICT-beroepsrol, met een gezamenlijke basis als uitgangspunt.

De HBO-i beroepstaken zijn uitgewerkt door Stichting HBO-i, een samenwerkingsverband tussen de hogescholen die ICT-onderwijs verzorgen en het bedrijfsleven; elke vier jaar verschijnt een nieuwe versie van deze competentiematrix.

## LEF-vaardigheden: niveaus en kaders

Om aan de drie ontwikkelvlakken (persoonsvormend, product, sociaal) invulling te geven zijn de tien LEF-vaardigheden ontwikkeld, ter inspiratie en als referentie beschreven in vier niveaus. Ze zijn gebaseerd op (inter)nationale kaders voor hoger onderwijs — HBO-i, het European Qualifications Framework, het European e-Competence Framework (e-CF), EDISON en ICT Ethics — en gelden voor de bachelor Open-ICT, de bachelor Duaal én de master HCAI.

**Wat betekenen de niveaus**: elke vaardigheid kent vier niveaus die op elkaar voortbouwen en iets zeggen over hoe je handelt, keuzes maakt en de omstandigheden waarin je werkt:
- **Niveau 1** — propedeuse niveau
- **Niveau 2 en 3** — hoofdfase bachelor niveau
- **Niveau 4** — master niveau

**Wat betekent groeien in niveau**: een hoger niveau betekent niet alleen dat je iets beter uitvoert. De opbouw zit ook in de groei van zelfstandigheid, complexiteit van de context en complexiteit van de inhoud. Vaardigheden van een hoger niveau zijn daarom alleen te oefenen en zichtbaar te maken in situaties die meer vragen — bijvoorbeeld wanneer de situatie minder vastligt, er wezenlijkere keuzes te maken zijn, of je vaker moet omgaan met onzekerheid, belangen of afhankelijkheden. Dat sluit aan bij de drie opdrachtniveaus: in een taakgerichte situatie laat je vooral basisvaardigheden zien, in een probleemgerichte situatie worden afwegingen en keuzes zichtbaar, en in meer open of professionele (situatiegerichte) situaties wordt zichtbaar hoe je handelt bij onzekerheid. Examinatoren betrekken deze samenhang bij de beoordeling.

## Essentiële ICT-waarden

Duurzaamheid, security en de sustainable development goals (SDG's) zijn brede, abstracte begrippen. Om dit tastbaar en toepasbaar te maken zijn, vanuit drie duurzaamheidsperspectieven (ecologisch, economisch, sociaal), vijf essentiële waarden gedefinieerd:

1. **Duurzame energie** (bron, efficiëntie, koeling, restwarmte) — ecologisch
2. **Verantwoorde hardware** (materialen, werkomstandigheden, kosten, milieu-impact) — ecologisch, sociaal en economisch
3. **Privacy** (vertrouwelijkheid, betrouwbaarheid, autonomie) — sociaal en economisch
4. **Security** (weerbare infrastructuur, datazuinigheid, digitale autonomie) — economisch
5. **Toegankelijkheid en inclusie** (geen uitsluiting van beperkte of minder machtige groepen) — sociaal

Dit sluit aan bij de waarden uit de landelijke domeinbeschrijving van HBO-i, met als verschil dat duurzaamheid concreet wordt ingevuld voor ICT, specifiek gericht op energie en hardware. Deze vijf waarden vormen een gemeenschappelijke basis die alle studenten meekrijgen, ongeacht hun studierichting.

## Feedback

Als studenten goed in de flow zitten, zoeken ze uitdagingen op en maken ze in korte sprints mooie dingen. In het team en tijdens de gilde-bijeenkomsten krijgen en geven studenten hulp en feedback — over zowel producten als proces. Dat gebeurt vaak in het werk zelf en mondeling, zodat directe toelichting mogelijk is. Het regelmatig ophalen en geven van informatieve feedback is een belangrijk onderdeel van de vaardigheden zelf.

## (Zelf)evaluatie

De Open-ICT ontwikkelmethode is erop gericht studenten te laten groeien van *onbewust-onbekwaam* naar *bewust-bekwaam*, door bewust te worden van wat ze al kunnen én van wat ze nog moeten versterken. Een semester kent daarom **minimaal twee evaluaties per vaardigheid**, doorgaans na een periode van 8-10 weken (grofweg halverwege en aan het einde van het semester).

**Evaluatieproces**: een evaluatie begint met een zelfevaluatie in Portflow, waarin de student onderbouwt op welk niveau hij/zij de vaardigheden heeft laten zien, aan de hand van de LEF-beschrijvingen en op basis van bewijsmateriaal (producten, verzamelde feedback, eigen reflecties). De daadwerkelijke evaluatie — de niveaubepaling — wordt vervolgens gedaan door de coach of een gildemeester op basis van die onderbouwing; de docent mag daarbij ook eigen waarnemingen meenemen.

Er zijn twee soorten evaluaties:
- **Productevaluaties** — de student onderbouwt de vier productvaardigheden (Overzicht Creëren, Kritisch Oordelen, Juiste Kennis Ontwikkelen, Kwalitatief Product Maken) aan de hand van meerdere producten. In jaar 1 gaat dit naar de coach, in jaar 2-4 naar de gildemeester, bij het afstuderen naar de examinatoren. Kwalitatief Product Maken toetst in hoeverre de student, vanuit de beroepsrol, in staat is producten passend bij een beroepsopdracht op het gewenste niveau te maken.
- **Evaluaties van sociale en persoonsvormende vaardigheden** — de student onderbouwt aan de hand van één of meer situaties (rol/taak, ondernomen acties, resultaat), meestal geëvalueerd door de coach.

**Voortgangsfeedback**: tijdens de tweewekelijkse ontwikkelgesprekken geeft de coach regelmatig feedback op de voortgang aan de hand van de semester roadmap. De student bereidt dit voor door zichzelf te evalueren op hoeveelheid en niveau van de evaluaties, en beantwoordt vragen als: heb je regelmatig feedback gekregen/gegeven, is er een goede balans tussen leerdoelen en vaardigheden, en kun je het semester halen met de huidige werkwijze? De coach legt dit vast in Portflow, inclusief de verwachte assessbaarheid van de student.

## (Zelf)assessment

Assessment gebeurt door een assessmentportfolio in Portflow, gekoppeld aan het semester waarvoor de student is ingeschreven en het bijbehorende niveau vanuit de beroepsrol.

**Zelfassessment**: de student beschrijft bondig het project en de producten waaraan is gewerkt, en onderbouwt — op basis van de uitgevoerde semester roadmap en de feedback van de gildemeester — waarom de beroepsrol voldoende is ingekleurd (Kwalitatief Product Maken).

**Toetsniveaus van opdrachten**: het startpunt is HBO-i. Open-ICT kent drie niveaus: taakgericht (T — jaar 1), probleemgericht (P — jaar 2) en situatiegericht (S — jaar 3/4). Voor het aantonen van een niveau moet de student minimaal twee van de drie assen op het gewenste eindniveau laten zien: complexiteit van de inhoud, complexiteit van de context, en zelfstandigheid.

**Niveaus per semester**: per studiefase en semester ligt vast welke niveaus minimaal behaald moeten zijn voor een "Op Niveau"-beoordeling; voor "Boven Niveau" moeten de vaardigheden voldoen aan de niveaus van het volgende semester. "KPM op X" betekent dat een student vanuit de beroepsrol passende HBO-i beroepstaken heeft uitgevoerd op niveau X.

- **Stage**: minstens één semester in jaar 3 of 4 omvat een stage, tenzij de jaarcoördinator akkoord gaat met een alternatief (een startup for-profit, of een bijdrage aan een open source gemeenschap not-for-profit) — dit project moet vooraf goedgekeurd worden.
- **Vrije profileringsruimte**: kan binnen of buiten Open-ICT worden ingevuld; de studiepunten vallen in semester 6, met de eisen van semester 6.

**Beoordeling door de docent**: het assessment moet minimaal voldoen aan twee voorwaarden — (1) voor alle vereiste vaardigheden van het semester zijn er minimaal twee afgeronde evaluaties, en (2) voor de vaardigheid "boodschap delen" heeft de student in de semesters van jaar 2 en jaar 3 kennisdeling in het gilde gedaan (afstuderen kent hiervoor aparte regels). Na het zelfassessment doet de coach het daadwerkelijke assessment op basis van de beschrijving en onderbouwing van de student, met een eigen beoordeling.

*Vier-ogen-principe*: in jaar 1 zijn studenten onderdeel van een tribe met meerdere coaches; vanaf jaar 2 hebben studenten zowel een coach als een gildemeester (die geen examinator hoeft te zijn). Alle studenten worden door de coach besproken met collega's uit hetzelfde jaar tijdens een beslisvergadering.

*Semesterbesluit*: na 17 weken doet de coach een voorlopige beoordeling, met drie mogelijke uitkomsten — (1) student is op niveau en mag tijdens twee verbeterweken doorwerken voor een boven-niveau-beoordeling, (2) student is nog niet op niveau maar verbeteren wordt haalbaar geacht, dus verbeterweken richting op-niveau, of (3) student is nog niet op niveau en verbeteren wordt niet haalbaar geacht, waarna de coach direct het definitieve semesterbesluit neemt. Na eventuele verbeterweken neemt de coach het definitieve semesterbesluit.

**Beoordeling verwerken**:
1. **Assessment** (Portflow) — zelfassessment maken → examinatoren beoordelen het zelfassessment → snapshot maken van het portfolio met het assessment erin.
2. **Vastleggen** (Canvas en Osiris) — snapshot inleveren via de opdrachtenpagina in Canvas → inschrijven in Osiris met de cursuscode van het semester en het gilde → coach checkt het snapshot in Canvas → coach zet de beoordeling in Osiris.

## Planning

Iedere student heeft elke twee weken een *ontwikkelgesprek* met de coach, waarin de ontwikkeling van de student centraal staat. Tijdens de eerste twee ontwikkelgesprekken bespreken coach en student de door de student gemaakte **semester roadmap**: het document waarin de student de voortgang op producten, evaluaties en feedback plant, en dat het hele semester de leidraad vormt waarmee de student de regie voert op het eigen leer- en werkproces.

De student krijgt uiterlijk in week 4 feedback op de semester roadmap — in semester 1 en 2 van de coach, in semester 3-7 van coach en gildemeester — vastgelegd in Portflow. De roadmap wordt daarna regelmatig besproken, minimaal één keer tussen week 9 en 11 en bij behoefte vaker. De student past de roadmap waar nodig aan tijdens het semester; propedeusestudenten krijgen hierbij meer ondersteuning van hun coach, bijvoorbeeld op basis van een blauwdruk of voorbeelden.

## Huisregels

Bij Open-ICT gelden in het werk- en leerproces de volgende huisregels:

**Aanwezigheid en ziekte**
- Online aanwezig tussen 09.00 en 12.30 uur wanneer je niet op school bent ingepland
- Zelfstandig werken aan taken buiten de contactmomenten
- Teammeetings en gildes zijn verplicht
- Bij ziekte minimaal één uur van tevoren afmelden

**Inzet**
- Wekelijks 36 uur onderwijsactiviteiten verantwoorden
- Bij een relevante bijbaan mogen maximaal 4 uur meetellen

**Bijbaan**
- Maximaal twee middagdagdelen per week
- Voor urenverantwoording is toestemming van de coach nodig

**Fraude en plagiaat**
- Lever uitsluitend eigen werk in
- Verwerk ontvangen feedback voordat nieuw bewijs wordt aangeboden
- Gebruik altijd correcte bronvermelding
- Fraude kan leiden tot zware sancties

**Kanalen voor berichtgeving**
- Officiële communicatie verloopt via Teams en Outlook
- Discord wordt gebruikt als aanvullend communicatiekanaal

## AI-gebruik in studiemateriaal

Onderstaand kader geldt alleen als je Open-ICT student bent.

Voor al het materiaal dat je in het kader van je studie maakt, gelden deze uitgangspunten:

1. Je bent volledig verantwoordelijk voor wat je inlevert. Als je je baseert op output van generatieve AI die later onjuist, geplagieerd of vervalst blijkt te zijn, word jij, als gebruiker, daarvoor verantwoordelijk gehouden. Jij bent immers de auteur en niet de generatieve AI-tool.
2. Je zorgt ervoor dat je ingeleverde werk het docententeam in staat stelt te beoordelen welke competenties jij als student hebt verworven.

### Producten

Het gebruik van AI is toegestaan voor het maken van producten. Als je bij het werken in je project documenten, code, andere producten laat genereren door AI, dan moet je aangeven in welke mate je AI hebt gebruikt en je moet in een bijlage je prompts noemen. Voor het bepalen van de mate waarin je AI hebt gebruikt kun je gebruik maken van de volgende indeling: https://mmmlabel.tech/

### Reflecties

Het gebruik van AI is niet toegestaan voor het formuleren of herschrijven van reflecties.

Je kunt AI wel gebruiken om tips te vragen waarover je kunt reflecteren, of hoe je je reflectie meer diepgang geeft; je moet dan wel zorgen dat je alle tekst zelf hebt geschreven.

Voeg dan je prompt toe in een bijlage.

### Zelf-evaluaties

Om goed te kunnen beoordelen in hoeverre jij je vaardigheden beheerst, hebben we jouw eigen woorden nodig. Het gebruik van AI is daarom niet toegestaan voor het formuleren of herschrijven van zelf-evaluaties of feedback aan anderen.

Je kunt AI wel gebruiken om tips te vragen, hoe je je evaluatie meer diepgang geeft; of hoe je naar hogere niveaus kunt groeien. Als je dat doet, voeg dan je prompt toe in een bijlage.
