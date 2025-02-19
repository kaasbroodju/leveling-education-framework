--
-- PostgreSQL database dump
--

-- Dumped from database version 17.3 (Debian 17.3-1.pgdg120+1)
-- Dumped by pg_dump version 17.0

-- Started on 2025-02-17 17:26:08 CET

--
-- TOC entry 3393 (class 0 OID 16419)
-- Dependencies: 221
-- Data for Name: Activity; Type: TABLE DATA; Schema: public; Owner: lef
--

INSERT INTO "Activity" (activity) VALUES ('Analyseren');
INSERT INTO "Activity" (activity) VALUES ('Adviseren');
INSERT INTO "Activity" (activity) VALUES ('Ontwerpen');
INSERT INTO "Activity" (activity) VALUES ('Realiseren');
INSERT INTO "Activity" (activity) VALUES ('Manage & Control');


--
-- TOC entry 3392 (class 0 OID 16412)
-- Dependencies: 220
-- Data for Name: ArchitectureLayer; Type: TABLE DATA; Schema: public; Owner: lef
--

INSERT INTO "ArchitectureLayer" ("architectureLayer") VALUES ('Gebruikersinteractie');
INSERT INTO "ArchitectureLayer" ("architectureLayer") VALUES ('Organisatieprocessen');
INSERT INTO "ArchitectureLayer" ("architectureLayer") VALUES ('Infrastructuur');
INSERT INTO "ArchitectureLayer" ("architectureLayer") VALUES ('Software');
INSERT INTO "ArchitectureLayer" ("architectureLayer") VALUES ('Hardwareinterfacing');


--
-- TOC entry 3394 (class 0 OID 16426)
-- Dependencies: 222
-- Data for Name: HBOIDescription; Type: TABLE DATA; Schema: public; Owner: lef
--

INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Analyseren', 1, 'Identificeren van de kernelementen van een externe opdracht waarbij verduidelijking wordt gezocht bij opdrachtgever, eindgebruikers en experts

of

Inventariseren van opdrachtgevers- en eindgebruikersbehoeften en deze vertalen naar ict-middelen

of

Oriënteren op bestaande interactieve concepten, diensten en producten', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Analyseren', 2, 'Benchmarken van functionaliteit, gebruikerservaring, toegankelijkheid, en andere ontwerpaspecten voor een opdrachtgever

of

Analyseren van bestaande producten of diensten in relatie tot gebruikersbehoeften en kernwaarden van de opdrachtgever

of

Inventariseren van relevante data-visualisaties voor een dataset', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Analyseren', 3, 'Analyseren van de eindgebruiker en gebruikersinteractie en -ervaring, zowel individueel (fysiek, psychologisch, persoonlijke karakteristieken) als in grotere maatschappelijke context (sociaal, cultureel, ethisch, technologisch)

of

Analyseren van actuele en opkomende interactieve technologieën

of

Doorlopend evalueren van het effect van de interventie op de gebruikerservaring', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Analyseren', 4, 'Analyseren van maatschappelijke en/of domeinspecifieke trends & kansen en hierover op strategisch niveau communiceren met de key stakeholders

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Adviseren', 1, 'Adviseren over interactieontwerp passend bij opdracht, opdrachtgever, gebruikersbehoeften en voorafgaande inventarisaties

of

Adviseren over interactieontwerp op basis van een eenvoudige, eigen of gegeven bruikbaarheidsanalyse

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Adviseren', 2, 'Geven van een gemotiveerd, concreet advies over de te gebruiken interactieconcepten en/of -technieken

of

Voorstellen doen over de realisatiekeuzes, zoals de te gebruiken technologieën, daarbij rekening houdend met de professionele context en eindgebruikers

of

Adviseren over de doelstellingen van de huidige en volgende iteraties', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Adviseren', 3, 'Vertalen van een analyse naar strategische aanbevelingen voor realisatie van gebruikersinteractie, inclusief advies voor een geschikt ontwerpproces

of

Adviseren welke vormen van datavisualisatie voor de opdrachtgever het gewenste effect zullen hebben, rekening houdend met kwaliteitseisen en ethische randvoorwaarden

of

Adviseren over interventies op de gebruikerservaring in de huidige en volgende iteraties', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Adviseren', 4, 'Extrapoleren van technologische en maatschappelijke trends en deze vertalen naar een advies voor het ontwerp op strategisch niveau dat een visie omvat op de gebruikerservaring en de relatie tussen gebruiker en product/dienst

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Ontwerpen', 1, 'Vertalen van adviezen in een eenvoudige gebruikersinteractie via een gegeven interactieproces, bijv. met een gangbare prototypingtechniek

of

Ontwerpen van een test waarmee essentiële interactieproblemen geïdentificeerd kunnen worden

of

Ontwerpen van een visualisatie van een eenvoudige dataset', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Ontwerpen', 2, 'Vertalen van adviezen in een ontwerp van gedetailleerde gebruikersinteractie met verschillende prototypingtechnieken

of

Ontwerpen van testen waarmee de doelstellingen van de iteratie geëvalueerd worden

of

Onder eigen regie toepassen van een interactieontwerpproces in samenwerking met stakeholders', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Ontwerpen', 3, 'Vertalen van adviezen naar een concreet en gedetailleerd gebruikersinteractieontwerp, passend bij de projectfasering, hierbij gebruik makend van een adequaat en gemotiveerd ontwerpproces

of

Ontwerpen van een voor de fase passende teststrategie waarmee de doelstellingen vanuit het perspectief van de beoogde gebruikers geëvalueerd worden

of

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Ontwerpen', 4, 'Ontwerpen van een gebruikersinteractie, rekening houdend met langetermijnstrategie en organisatiedoelstellingen van de opdrachtgever en anticiperend op relevante maatschappelijke trends en technologische ontwikkelingen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Realiseren', 1, 'Realiseren en testen van eenvoudige interactieve producten of diensten op basis van een interactieontwerp, waarbij gebruik wordt gemaakt van gangbare tools, toegankelijkheidsrichtlijnen en/of huisstijl

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Realiseren', 2, 'Met verschillende tools en technieken realiseren en testen van het eigen interactieontwerp voor interactieve producten of diensten voor meerdere type eindgebruikers

of

Realiseren van een visualisatie van een dataset voor verschillende typen eindgebruikers

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Realiseren', 3, 'Realiseren, testen en overdragen van de gebruikerservaring van een interactief product, prototype, systeem of dienst op basis van het ontwerp met voor de projectfasering passende tools en technieken

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Realiseren', 4, 'Realiseren van toekomstbestendige producten, diensten of prototypes die innovatief en duurzaam zijn op basis van gebruikersinteractieontwerp en tools en technieken

of

Validatie met key stakeholders van visie en strategie

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Manage & Control', 1, 'Vastleggen van de belangrijkste beslissingen, resultaten en inzichten met betrekking tot het interactieontwerp in een iteratief proces
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Manage & Control', 2, 'Toepassen van standaarden (interactieontwerprichtlijnen, technieken en methoden), passend binnen de professionele context

of

Bewaken en overdragen van het interactie-ontwerp bij het gerealiseerde interactieve product of dienst
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Manage & Control', 3, 'Bewaken van de kernwaarden en gebruikerservaring van product, organisatie of dienst in iedere fase van het ontwikkel- en productieproces

of

Planmatig en methodisch vastleggen van designkeuzes voor alle stakeholders binnen een bedrijf

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Gebruikersinteractie', 'Manage & Control', 4, 'Vanuit gebruikersinteractie-perspectief aansturen van een complex project op strategisch niveau met inachtneming van korte- en langetermijndoelen en met afstemming met alle betrokken stakeholders

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Analyseren', 1, 'Analyseren van een enkel(e) organisatie, organisatieproces of procesbesturing op operationeel niveau met bijbehorende gegevensstromen en (gestructureerde) databehoeften

of

Analyseren van knelpunten en oorzaak-gevolgrelaties vanuit de invalshoek van de informatievoorziening

of

Analyseren van beschikbare ict- mogelijkheden in het veld', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Analyseren', 2, 'Analyseren van meerdere operationele en tactische organisatieprocessen, inclusief kwaliteit van de bijbehorende data en van de huidige en toekomstige ict-voorziening

of

Analyseren van de samenhang van knelpunten en oorzaak-gevolgrelaties

of

Vaststellen van de ict-requirements vanuit de behoefte van relevante stakeholders', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Analyseren', 3, 'Analyseren van de consequenties van een (strategische) koerswijziging voor organisatieprocessen en bijbehorende informatievoorziening

of

Analyseren (kwantitatief en/of kwalitatief) van de huidige en toekomstige situatie op het gebied van bijvoorbeeld beleid, strategie, alignment en architectuur

of

Analyseren van de acceptatie van en eventuele weerstanden tegen de huidige en vernieuwde technologie en organisatieprocessen', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Analyseren', 4, 'Uitvoeren van grondig theoretisch onderbouwd toegepast onderzoek naar technologische (interorganisationele) procesinnovaties

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Adviseren', 1, 'Adviseren over verbeteringen voor een enkel organisatieproces op het terrein van organisatie(structuur), processen en gestructureerde data, met inachtneming van de mogelijkheden van ict

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Adviseren', 2, 'Samenhangende oplossingen adviseren voor knelpunten op het terrein van organisatiestructuur, processtructuur en informatievoorziening

of

Adviseren over nieuwe ict-mogelijkheden, waaronder pakketselectie en advies

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Adviseren', 3, 'Adviseren over de inrichting van en afstemming tussen business en ict (alignment en governance), rekening houdend met de doelstellingen van de organisatie

of

Adviseren over een veranderkundige aanpak bij de invoering van nieuwe, duurzame ict-mogelijkheden en organisatieprocessen

of

Adviseren over oplossingen voor gestructureerde en ongestructureerde data, rekening houdend met ethische en juridische aspecten', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Adviseren', 4, 'Adviseren van organisatorische en technologische (interorganisationele) procesinnovaties, waarbij rekening wordt gehouden met alle relevante interne en externe stakeholders, de sociale context (mens, maatschappij en organisatie) en ethische en juridische aspecten

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Ontwerpen', 1, 'Ontwerpen van enkele organisatieprocessen, enkele gegevensstromen van gestructureerde data, de inrichting van een organisatieonderdeel en/of een deel van de informatievoorziening

of

Opstellen van een eenvoudig datamanagementplan

of

Opstellen van een eenvoudig implementatieplan', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Ontwerpen', 2, 'Ontwerpen van samenhangende organisatieprocessen: functionele organisatiestructuur, procesmanagement en informatievoorziening, rekening houdend met security en privacywetgeving

of

Ontwerpen van de interfaces voor een applicatie in het applicatielandschap (mappings)
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Ontwerpen', 3, 'Ontwerpen van de architectuur van organisatieprocessen en/of besturingsmodellen, inclusief bijbehorende beheersing met behulp van data-oplossingen, informatievoorziening en veranderproces

of

Ontwerpen van een veranderkundige aanpak met bijbehorende interventies

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Ontwerpen', 4, 'Evalueren en valideren van mogelijke procesinnovaties, onder andere aan de hand van data

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Realiseren', 1, 'Beschrijven en opstellen van werkinstructies, functie- en rolbeschrijvingen en procedures voor een (aangepast) proces

of

Testen van de aansluiting van de organisatieprocessen met de opgeleverde informatievoorziening

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Realiseren', 2, 'Realiseren van de invoering en acceptatie van procedures in samenhang met nieuwe of gewijzigde informatievoorziening en besturing

of

Genereren en valideren van een proof of concept van een applicatie

of

Inrichten van een standaardapplicatie', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Realiseren', 3, 'Realiseren van invoering en acceptatie van gewijzigde organisatieprocessen op basis van een implementatieplan

of

Creëren van draagvlak voor veranderingen onder alle relevante (interne) stakeholders

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Realiseren', 4, 'Bouwen en valideren van (prototypen van) nieuwe organisatieprocessen en technologische oplossingen voor (interorganisationele) procesinnovaties

of

Creëren van breed draagvlak voor veranderingen onder alle relevante interne en externe stakeholders

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Manage & Control', 1, 'Verrichten van onderhoudswerkzaamheden aan de procesdocumentatie (bijv. business rules, principes en procesmodellen)

of

Meten en bewaken van ict-processen aan de hand van data

of

Beschrijven van de veranderbehoefte voor een enkel deelproces', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Manage & Control', 2, 'Inrichten, onderhouden en actualiseren van (functionele) beheerprocessen

of

Meten en bewaken van organisatieprocessen aan de hand van data

of

Signaleren van de veranderbehoefte van meerdere operationele en tactische organisatieprocessen', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Manage & Control', 3, 'Formuleren en actualiseren van principes, business rules en modellen van procesarchitectuur

of

Meten en bewaken van de strategische doelen van de organisatie aan de hand van data

of

Proactief signaleren van behoefte aan verandering in alle organisatieprocessen en bijbehorende veranderprocessen in gang zetten', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Organisatieprocessen', 'Manage & Control', 4, 'Bedenken van nieuwe organisatorische en technologische oplossingen voor het beheren van (interorganisationele) procesinnovaties

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Analyseren', 1, 'Analyseren van een eenvoudige infrastructuur volgens een standaardmethode en op basis van gegeven kwaliteitseisen, bijvoorbeeld security

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Analyseren', 2, 'Analyseren van de kwaliteit van een bestaande infrastructuur en de daarop aanwezige services aan de hand van gangbare methoden en standaarden

of

Analyseren van aan infrastructuur gerelateerde incidenten, problemen en security-bedreigingen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Analyseren', 3, 'Analyseren van trends en best practices en deze vertalen naar gewenste of benodigde ontwikkelingen in de enterprise infrastructuur

of

Uitvoeren van een requirementsimpact- of gap-analyse voor een enterprise infrastructuur om kwaliteitseisen op te stellen

of

Analyseren van technische mogelijkheden en privacy concerns van systemen voor beheer en verwerking van data', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Analyseren', 4, 'Uitvoeren van een grondig, theoretisch onderbouwd en toegepast onderzoek naar referentiearchitecturen, best practices en standaarden voor cloud-agnostische enterprise infrastructuren of high performance computing om het volwassenheidsniveau bij verschillende organisaties te kunnen bepalen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Adviseren', 1, 'Aanbevelingen doen over een opzet van, of aanpassingen aan, een eenvoudige infrastructuur
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Adviseren', 2, 'Adviseren over inrichting en beheer van een (cloudgebaseerde) infrastructuur met onderbouwde keuzes vanuit kwaliteitseisen, beschikbare technologie en beheermethodes

of

Maatregelen voorstellen die de informatiebeveiliging van een infrastructuur verbeteren

of

Adviseren over de mogelijke migratie naar of keuze voor bijvoorbeeld een private, hybride of publieke cloud', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Adviseren', 3, 'Adviseren over zakelijke rechtvaardiging en business-IT alignment van enterprise infrastructuren, inclusief beheer-, beveiliging- en privacy aspecten, in relatie tot informatie- en referentiearchitecturen

of

Adviseren over de inrichting van de infrastructuur voor de verwerking van grote hoeveelheden data

of

Adviseren over een cloudmanagementplatform voor DevOps', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Adviseren', 4, 'Adviseren over de architectuur van een enterprise infrastructuur of high performance computing, inclusief beheer-, beveiliging- en privacyaspecten, in relatie tot informatie- en referentiearchitecturen, innovatie, maatschappelijke en internationale ontwikkelingen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Ontwerpen', 1, 'Opstellen van specificaties voor een eenvoudige, bijvoorbeeld cloudgebaseerde, infrastructuur volgens een standaardmethode

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Ontwerpen', 2, 'Ontwerpen van een infrastructuur door specificatie van technieken op basis van gegeven requirements met betrekking tot kwaliteitseisen zoals beschikbaarheid, performance, schaalbaarheid, security. privacy en duurzaamheid

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Ontwerpen', 3, 'Ontwerpen van een cloud-agnostische enterprise infrastructuur door specificatie van cloudtechnieken op basis van zelf opgestelde requirements binnen de kaders van enterprise architectuur, referentiearchitecturen en/of standaarden

of

Ontwerpen van een security operations center (SOC) voor het voorkomen en oplossen van security-incidenten en -problemen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Ontwerpen', 4, 'Ontwerpen van cloud-agnostische enterprise infrastructuren, inclusief de processen, om een hoger volwassenheidsniveau te bereiken
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Realiseren', 1, 'Inrichten, testen en beschikbaar stellen van (een proof of concept van) een eenvoudige infrastructuur
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Realiseren', 2, 'Inrichten en testen van (een proof of concept van) een cloudgebaseerde infrastructuur met gebruik van (cloud)technieken waarmee voldaan wordt aan ontwerp en daarin gestelde eisen

of

Opstellen en uitvoeren van een testplan voor een infrastructuur om de kwaliteit te toetsen op basis van de risico''s

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Realiseren', 3, 'Realiseren van (een proof of concept van) een cloud-agnostische infrastructuur door specificatie van cloudtechnieken binnen de kaders van enterprise architectuur, referentiearchitecturen en/of standaarden

of

Realiseren van een cloudmanagementplatform voor DevOps

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Realiseren', 4, 'Toepassen van volwassenheidsmodellen bij het realiseren van cloud-agnostische enterprise infrastructuren
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Manage & Control', 1, 'Opzetten en documenteren van standaardbeheerprocessen en werkprocedures voor beheer van een eenvoudige, bijvoorbeeld cloudgebaseerde, infrastructuur

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Manage & Control', 2, 'Implementeren van het beheer van technologische ontwikkelingen m.b.t. de (cloudgebaseerde) infrastructuur

of

Implementeren van delen van beheerprocessen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Manage & Control', 3, 'Implementeren van beheerprocessen voor een cloud-agnostische enterprise infrastructuur

of

Inbedden van een vernieuwde infrastructuur in de organisatie

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Infrastructuur', 'Manage & Control', 4, 'Vormgeven van Business - IT alignment en IT governance in relatie tot een cloud-agnostische enterprise infrastructuur

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Analyseren', 1, 'Verzamelen en valideren van functionele eisen voor een softwaresysteem met één stakeholder volgens een standaardmethode en het opstellen van acceptatiecriteria

of

Uitvoeren van een analyse van de functionaliteit van een bestaand softwaresysteem of bestaande component om de (on)mogelijkheden voor aanpassing vast te stellen

of

Analyseren of een gegeven dataset informatie oplevert voor een gegeven toepassing', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Analyseren', 2, 'Uitvoeren van een requirementsanalyse voor een softwaresysteem met verschillende stakeholders, rekening houdend met duurzaamheidsaspecten en andere kwaliteitseigenschappen waaronder security

of

Uitvoeren van een analyse om de functionaliteit, veiligheid, ontwerp, interfaces e.d. van een bestaand softwaresysteem of bestaande component te formuleren en te valideren

of

Beoordelen van de kwaliteit van een dataset met onder andere beschrijvende statistiek en visualisaties', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Analyseren', 3, 'Uitvoeren van een requirementsanalyse voor een softwaresysteem met verschillende stakeholders in een context van bestaande systemen

of

Definiëren van acceptatiecriteria aan de hand van kwaliteitseigenschappen en een uitgevoerde risicoanalyse met onder andere aandacht voor duurzaamheids-, security- en privacyaspecten en toegankelijkheid
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Analyseren', 4, 'Analyse uitvoeren op complexe software-in-software systemen om alle niet-functionele requirements, waaronder safety, security en privacy en de compliance daarvan met wet- en regelgeving te inventariseren om te komen tot best practices en vernieuwde ict-standaarden

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Adviseren', 1, 'Aanbevelingen doen over specifieke requirements van een softwaresysteem op grond van onderzoek naar bestaande, vergelijkbare systemen
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Adviseren', 2, 'Adviseren over aanschaf en selectie van softwarecomponenten bij het ontwikkelen van een softwaresysteem op basis van functionaliteit en kosten

of

Adviseren over een onderdeel van een architectuur of een beperkt softwaresysteem

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Adviseren', 3, 'Adviseren in de keuze van een data-architectuur voor een dataoplossing, opgebouwd uit bestaande en nieuwe databronnen, waarbij kostenaspecten en kwaliteitseisen zoals beschikbaarheid, performance, security en schaalbaarheid een rol spelen

of

Adviseren over de toepassing van nieuwe technologieën (als machine learning en artificial intelligence) alsmede de impact daarvan op aspecten als duurzaamheid, security en privacy

of

Adviseren over de inrichting van een softwareontwikkelproces, waaronder het test- en releaseproces', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Adviseren', 4, 'Definiëren van een visie ten aanzien van toekomstige technologie in afstemming met stakeholders en compliance met wet- en regelgeving op aspecten van security en privacy
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Ontwerpen', 1, 'Maken van een ontwerp voor een softwaresysteem, inclusief database, met modelleertechnieken volgens een standaardmethode

of

Opstellen van testscripts voor eindgebruikers/acceptatietests

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Ontwerpen', 2, 'Opstellen van een ontwerp voor een softwaresysteem, rekening houdend met het gebruik van bestaande componenten en libraries

of

Toepassen van ontwerpkwaliteitscriteria rekening houdend met (duurzaamheids)aspecten zoals privacy, grote hoeveelheden data en gebruik op diverse devices

of

Vaststellen van de kwaliteit van het ontwerp, bijvoorbeeld door toetsing of prototyping, rekening houdend met de geformuleerde kwaliteitseigenschappen

of

Opstellen van testontwerpen volgens een gegeven teststrategie', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Ontwerpen', 3, 'Opstellen van een softwarearchitectuur voor een softwaresysteem, opgebouwd uit bestaande en nieuwe systemen, rekening houdend met meerdere stakeholders en kwaliteitseisen

of

Opstellen van teststrategie voor systeem- en compliancytesten

of

Ontwerpen van de data-architectuur en de modelarchitectuur inclusief een teststrategie voor de data en de machine learning-modellen', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Ontwerpen', 4, 'Opstellen van een herbruikbare softwarearchitectuur voor (cross-platform) softwaresystemen opgebouwd uit bestaande en nieuwe (cloud-based) systemen, rekening houdend met meerdere stakeholders, kwaliteitskenmerken en compliance met wet- en regelgeving

of

Ontwerpen van een softwaresysteem of framework voor het oplossen van een generieke klasse van problemen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Realiseren', 1, 'Op gestructureerde wijze bouwen, testen en beschikbaar stellen van een eenvoudig softwaresysteem dat werkt met gestructureerde data en voldoet aan de basis kwaliteitseisen

of

Opstellen en uitvoeren van (geautomatiseerde) unittesten

', '# Opmerkingen

Denk bij het bevragen van een database ook aan het bevragen van een (REST API) backend vanuit de frontend.

# Beroepsproducten

Bij het uitvoeren van deze beroepstaak kunnen onder andere de volgende beroepstaken worden gemaakt:
- Werkende code;
- Werkende database;
- Webpagina''s/website.

# Links en bronnen

- [Backend development](https://www.youtube.com/playlist?list=PLKssF68zVDQ7Byksmqyi69MyTtxunGBEN)');
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Manage & Control', 2, 'Beoordelen van een gegeven ontwikkelomgeving op basis van kwaliteitseisen

of

Monitoren, rapporteren en beoordelen van een gegeven testontwikkelomgeving

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Realiseren', 2, 'Bouwen van een softwaresysteem dat bestaat uit meerdere subsystemen, gebruikmakend van bestaande of gegenereerde componenten en de gemaakte keuze kunnen beredeneren

of

Integreren van softwarecomponenten in een bestaand softwaresysteem, waarbij onder andere de integriteit, veiligheid en systeemprestaties bewaakt worden

of

Opstellen en uitvoeren van (geautomatiseerde) unit- en UI-testen', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Realiseren', 3, 'Bouwen, testen en beschikbaar stellen van een schaalbaar softwaresysteem dat aansluit bij bestaande systemen, eventueel in de cloud, volgens een ontworpen of gegenereerde architectuur met onderbouwd gebruik van frameworks

of

Opstellen en uitvoeren van regressie-, integratie- en systeemtesten en de uitkomsten ervan evalueren, verwerken en opvolgen

of

Refactoren van een bestaande applicatie m.b.v. design patterns', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Realiseren', 4, 'Bouwen en beschikbaar stellen van (zelflerende) softwaresysteem gebaseerd op een model, algoritme of data met wetenschappelijk aantoonbare correcte werking en antwoorden gevend op ethische vragen

of

Opstellen en uitvoeren van (herbruikbare) compliancytesten

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Manage & Control', 1, 'Inrichten en gebruik maken van beheersysteem ter ondersteuning van softwareontwikkeling in teamverband

', '# Links en bronnen

- [GIT gebruik playlist](https://www.youtube.com/playlist?list=PLKssF68zVDQ4jNCrTlUE2UMhOucbvzyeg)');
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Manage & Control', 2, 'Beheren en gebruiken van een ontwikkelstraat ter ondersteuning van softwareontwikkeling in teams, waardoor onder andere continuous integration (CI) tot de mogelijkheden behoort

of

Toepassen van methoden en technieken om een softwareontwikkelproces te managen en de kwaliteit ervan te borgen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Manage & Control', 3, 'Uitvoeren van configuratie-, change- en releasemanagement in afstemming met infrastructuurmanagement waardoor CI/CD tot de mogelijkheden behoort

of

Inrichten en evalueren van een ontwikkelstraat met CI/CD ondersteuning

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Software', 'Manage & Control', 4, '(Door)Ontwikkelen van methoden en technieken voor het softwareontwikkelproces met als doel de effectiviteit en kwaliteit van het softwaresysteem te verbeteren alsmede de kosten en doorlooptijd van het softwareontwikkelproces te verkleinen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Analyseren', 1, 'Verzamelen van kwaliteitseisen en acceptatiecriteria voor een systeem, bijv. een embedded of ander technisch (deel)systeem op basis van een domeinanalyse

of

Beschrijven van de (fysieke) architectuur van een systeem, bijv. een embedded of ander technisch (deel)systeem

of

Beschrijven van de toepasbaarheid van actuatoren en sensoren', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Analyseren', 2, 'Uitvoeren van een requirementsanalyse voor een (deel)systeem, inclusief hardware- en software, rekening houdend met domeinaspecten en relevante kwaliteitseigenschappen waaronder security, safety en duurzaamheid

of

Uitvoeren van een protocolanalyse

of

Opstellen van een acceptatietest voor een systeem aan de hand van de kwaliteitseisen', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Analyseren', 3, 'Uitvoeren van een requirementsanalyse in afstemming met stakeholders voor een gedistribueerd systeem rekening houdend met de kwaliteitseisen inclusief timing, resourcegebruik, performance, security (netwerken), safety en andere relevante niet functionele eisen. (bijv. met machine learning-componenten)

of

Opstellen van een acceptatietestplan en een integratietestplan aan de hand van de kwaliteitseisen


', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Analyseren', 4, 'Onderzoek doen naar emerging technologies, bijv. machine learning, voor toepassing in gedistribueerde systemen

of

Onderzoek doen naar security, safety, privacy en duurzaamheidsaspecten binnen emerging technologies

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Adviseren', 1, 'Een onderbouwd technisch advies geven voor een eenvoudig (deel van een) systeem

of

Aanbevelingen doen voor de initiële structuur en functionaliteit van een gegeven (deel)systeem, zowel op hardware- als softwaregebied

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Adviseren', 2, 'Uitbrengen van een technisch advies voor de architectuur van een (deel van een) systeem inclusief de hardware- en software-componenten, op basis van de requirementsanalyse

of

Adviseren over het koppelen van hardwaresystemen en softwarecomponenten

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Adviseren', 3, 'Uitbrengen van een technisch advies over een te realiseren gedistribueerd systeem, inclusief hardware- en softwarecomponenten en koppelingen op basis van de requirementsanalyse en in relatie tot referentiearchitecturen, innovatie en internationale ontwikkelingen en standaarden

of

Adviseren over de inrichting van een ontwikkel- en testproces

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Adviseren', 4, 'Uitbrengen van een technisch advies over de toepassing van emerging technologies om een gedistribueerd systeem te realiseren

of

Adviseren over toekomstgerichte inrichting van gedistribueerde systemen

of

Definiëren van visie op technologieroadmap en deze afstemmen met key stakeholders', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Ontwerpen', 1, 'Ontwerpen van een (deel van een) systeem, bijv. een embedded of ander technisch systeem, op basis van gegeven hardware

of

Ontwerpen van een eenvoudig protocol
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Ontwerpen', 2, 'Methodisch ontwerpen van een (deel van een) systeem op basis van requirements met zelfgekozen hardware- en softwarecomponenten

of

Integreren van een hardware interface In een applicatie (bijv. een applicatie-driver)
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Ontwerpen', 3, 'Ontwerpen van een gedistribueerd systeem inclusief bepaling van relevante hardware- en softwarecomponenten op basis van de kwaliteitseisen inclusief niet-functionele eisen zoals timing, resourcegebruik, onderhoudbaarheid, safety en security

of

Ontwerpen van een protocol voor betrouwbare productie en transmissie van grote hoeveelheden data

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Ontwerpen', 4, 'Ontwerpen van gedistribueerde systemen met gebruik van hardware synthese en/of artificial intelligence

of

Ontwerpen van een gedistribueerd systeem met gebruik van machine learning-componenten inclusief bepaling van actuatoren, sensoren, circulariteit, CO2-footprint, timing, resourcegebruik en performance

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Realiseren', 1, 'Software schrijven voor een ontworpen (deel van een) systeem, voorzien van actuatoren en sensoren

of

Testen van een gerealiseerd (deel van een) systeem
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Realiseren', 2, 'Realiseren van een ontworpen (deel van een) systeem en met behulp van software de koppelingen met hardwarecomponenten realiseren

of

Schrijven en testen van ontworpen (delen van) systemen en hardware interfaces (bijv. applicatiedriver-software)

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Realiseren', 3, 'Realiseren van een gedistribueerd systeem of deel ervan op basis van een gegeven ontwerp, inclusief de relevante kwaliteitseisen

of

Opstellen en uitvoeren van een acceptatieprocedure van een gerealiseerd systeem
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Realiseren', 4, 'Realiseren van een compleet systeem waarbij gebruik gemaakt wordt van hardware synthese (VHDL) of artificial intelligence

of

Realiseren van een compleet systeem met machine learning-elementen inclusief netwerk, hardware en systeemsoftware

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Manage & Control', 1, 'Installeren en gebruiken van een ontwikkel- en testplatform ten behoeve van hardware/software, inclusief tools
', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Manage & Control', 3, 'Opzetten en gebruikmaken van versiebeheer, releasemanagement, teamworkondersteuning en automated testing voor hard- en softwaresystemen

', NULL);
INSERT INTO "HBOIDescription" ("architectureLayerId", "activityId", level, description, sublament) VALUES ('Hardwareinterfacing', 'Manage & Control', 4, 'Aansturen van co-designteams voor het beheren van het realisatieproces van de hardware, software en synthese, inclusief de ontwikkelomgeving

', NULL);


--
-- TOC entry 3390 (class 0 OID 16398)
-- Dependencies: 218
-- Data for Name: Skill; Type: TABLE DATA; Schema: public; Owner: lef
--

INSERT INTO "Skill" (skill) VALUES ('Overzicht creëren');
INSERT INTO "Skill" (skill) VALUES ('Kritisch oordelen');
INSERT INTO "Skill" (skill) VALUES ('Juiste kennis ontwikkelen');
INSERT INTO "Skill" (skill) VALUES ('Kwalitatief product maken');
INSERT INTO "Skill" (skill) VALUES ('Plannen');
INSERT INTO "Skill" (skill) VALUES ('Boodschap delen');
INSERT INTO "Skill" (skill) VALUES ('Samenwerken');
INSERT INTO "Skill" (skill) VALUES ('Flexibel opstellen');
INSERT INTO "Skill" (skill) VALUES ('Pro-actief handelen');
INSERT INTO "Skill" (skill) VALUES ('Reflecteren');


--
-- TOC entry 3391 (class 0 OID 16405)
-- Dependencies: 219
-- Data for Name: SkillDescription; Type: TABLE DATA; Schema: public; Owner: lef
--

INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Overzicht creëren', 1, 'Student stelt vragen om te ontdekken wat er gedaan moet worden en welke manieren er zijn om dit gedaan te krijgen. Student verzamelt deze informatie, bijvoorbeeld vanuit bronnen of experimenten, en geeft deze overzichtelijk weer. Student formuleert hieruit ook acceptatie-, kwaliteits- en/of selectiecriteria. Student laat zien dat waar mogelijk rekening is gehouden met toegankelijkheid en/of gegevensbescherming.', '# Opmerkingen

Deze vaardigheid, samen met KO, vormt het oordeelvormend vermogen, oftewel onderzoekend vermogen. "Overzicht creëren" heeft te maken met een goed beeld krijgen van het probleem en mogelijk oplossingsrichtingen.

# Mogelijke bewijsstukken

Voor het opvoeren van bewijs voor deze vaardigheid kan je bijvoorbeeld denken aan de onderstaande bewijsstukken. Deze lijst is niet volledig, maar geeft je een goede indicatie wat je zou kunnen gebruiken. Het kan natuurlijk ook een combinatie van deze stukken zijn.

- Een beschrijving van de probleemstelling/oplossingsrichtingen op bijvoorbeeld een wiki van het team/project

# Links
- [Product realiseren als een expert](https://openict.fyndr.wiki/items/cl3wla81m194674sn5sc1dozn3?playlist=cl3wkj8ei167324sn57r2ad7sm)');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Overzicht creëren', 2, 'niveau 1 +

Student laat zien dat op voor het product cruciale vraagstellingen door middel van research stories/tasks methodisch informatie is verkregen en weergegeven waarbij de waardes van directe stakeholders zijn meegenomen. Student laat zien dat waar mogelijk rekening is gehouden met duurzaamheid, overdraagbaarheid, inclusiviteit en/of security. De student kiest daarbij voor toepasselijke onderzoeksmethoden en voert deze ook vakkundig uit. Student gebruikt deze informatie om criteria te formuleren. De student onderbouwt dat de mate van grondigheid passend is voor de situatie.', '# Opmerkingen

Op dit niveau begin je je onderzoekend vermogen methodisch in te steken. Er zijn vele verschillende manieren waarop dit kan gebeuren. Binnen Open-ICT wordt dit vaak gedaan door middel van Research Stories en Research Taken.

Het stuk ''Overview''/''Overzicht'' uit het DOT Framework heeft vooral met deze vaardigheid te maken.

# Mogelijke bewijsstukken

Voor het opvoeren van bewijs voor deze vaardigheid kan je bijvoorbeeld denken aan de onderstaande bewijsstukken. Deze lijst is niet volledig, maar geeft je een goede indicatie wat je zou kunnen gebruiken. Het kan natuurlijk ook een combinatie van deze stukken zijn.

- De resultaten van gekozen onderzoeksmethode
  - Transcriptie van interview
  - Video van de observatie
  - Beschrijving met bronnenlijst

# Links
- [Research stories](https://openict.fyndr.wiki/playlists/cl3wkj8ei167324sn57r2ad7sm?playlist=cl3wkj8ei167324sn57r2ad7sm)
- [Praktijkgericht onderzoek met het DOT Framework](https://openict.fyndr.wiki/playlists/cl3wkneqv176584sn5lvy28cxf)');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Overzicht creëren', 3, 'niveau 2 +

Student laat zien dat de waardes van indirect betrokkenen buiten de opleiding, SDGs en/of wetenschap zijn meegenomen. Student laat zien dat waar mogelijk rekening gehouden is met schaalbaarheid, onderhoudbaarheid en/of ethiek.', '# Voorbeeld

Stel je hebt een uitdaging/vraagstuk waar je een oplossing voor wilt vinden. Het idee van niveau 3 van Overzicht Creëren is dat je het komen tot de aanpak van het probleem/uitdaging/vraagstuk methodisch gaat doen. Hiervoor stel je in het algemeen een research story op à la "Als team willen we weten hoe we het probleem effectief kunnen aanpakken zodat wij tot een goed onderbouwde oplossing kunnen komen". Deze story kan je natuurlijk aanpassen met specifieke details, maar wat belangrijk is is dat ze vanuit de story de taken en methodes kunnen benoemen.

---

## RS: aanpak vinden voor uitdaging

**Taken:**

- Vinden van mogelijke aanpakken (Community Research, Available Product Analysis, Expert Interview, etc...)
- Criteria opstellen voor keuze van de aanpak (Brainstorm, Literature Study, Expert Interview, etc...)
- Aanpakken scoren op basis van criteria (Multi-criteria Decision Making/MCDM, etc....)
- Conclusie trekken
- Opstellen van acceptatie- en kwaliteitscriteria voor de op te leveren producten uit de gekozen aanpak

**Tussenproducten: (bijvoorbeeld als attachment aan deze research story)**

- Overzicht van de mogelijke aanpakken
- Lijst van selectie criteria
- Gevulde tabel met conclusie uit MCDM

**Deliverables:**

- Een gekozen aanpak (in stories, wiki bijdrage, paper, rapport, etc...)

---

Door jouw aanpak-research story uit te voeren laat je zien dat je op een methodische manier gekomen bent tot een aanpak. Je kan ook laten zien door acceptatie- en kwaliteitscriteria op te stellen aan welke kwaliteit dat moet voldoen. Als jouw aanpak-research story is uitgevoerd weet je wat je moet gaan doen om tot een oplossing te komen. Vaak zitten daar weer research stories of taken bij om uiteindelijk ook de onderbouwing van je deliverables/producten voor je opdrachtgever te kunnen geven.');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Overzicht creëren', 4, 'niveau 3 +

Student laat zien dat de waardes komende uit wetenschappelijke standaarden, het beroepenveld de maatschappij en gekozen bedrijfsstrategie zijn meegenomen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kritisch oordelen', 1, 'Student verwerkt verkregen informatie, bijvoorbeeld uit bronnen of experimenten, en trekt daaruit conclusies op basis van vooraf opgestelde acceptatie-, kwaliteits- en/of selectiecriteria.', '# Opmerkingen

Deze vaardigheid, samen met OC, vormt het oordeelvormend vermogen, oftewel onderzoekend vermogen. "Kritisch oordelen" heeft te maken met het maken van onderbouwde keuzes en het nemen van verantwoorde beslissingen.

# Mogelijke bewijsstukken

Voor het opvoeren van bewijs voor deze vaardigheid kan je bijvoorbeeld denken aan de onderstaande bewijsstukken. Deze lijst is niet volledig, maar geeft je een goede indicatie wat je zou kunnen gebruiken. Het kan natuurlijk ook een combinatie van deze stukken zijn.

- Een beschrijving van de probleemstelling/oplossingsrichtingen op bijvoorbeeld een wiki van het team/project

# Links
- [Product realiseren als een expert](https://openict.fyndr.wiki/items/cl3wla81m194674sn5sc1dozn3?playlist=cl3wkj8ei167324sn57r2ad7sm)');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kritisch oordelen', 2, 'niveau 1 +

Student verkrijgt en verwerkt, door middel van research stories/tasks, methodisch informatie en trekt op basis van de criteria onderbouwde conclusies. De student kiest daarbij voor toepasselijke onderzoeksmethoden en voert deze ook vakkundig uit. Student is in staat om gangbare normen, praktijken en meningen ter discussie te stellen. De student onderbouwt dat de mate van grondigheid passend is voor de situatie.', '# Opmerkingen

Op dit niveau begin je je onderzoekend vermogen methodisch in te steken. Er zijn vele verschillende manieren waarop dit kan gebeuren. Binnen Open-ICT wordt dit vaak gedaan door middel van Research Stories en Research Taken.

Het stuk ''Certainty''/''Zekerheid'' uit het DOT Framework heeft vooral met deze vaardigheid te maken.

# Mogelijke bewijsstukken

Voor het opvoeren van bewijs voor deze vaardigheid kan je bijvoorbeeld denken aan de onderstaande bewijsstukken. Deze lijst is niet volledig, maar geeft je een goede indicatie wat je zou kunnen gebruiken. Het kan natuurlijk ook een combinatie van deze stukken zijn.

- De resultaten van gekozen onderzoeksmethode
 - Testrapport

# Links
- [Research stories](https://openict.fyndr.wiki/playlists/cl3wkj8ei167324sn57r2ad7sm?playlist=cl3wkj8ei167324sn57r2ad7sm)
- [Praktijkgericht onderzoek met het DOT Framework](https://openict.fyndr.wiki/playlists/cl3wkneqv176584sn5lvy28cxf)');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kritisch oordelen', 3, 'niveau 2 +

Student laat zien kritisch te zijn richting de opdracht, de opdrachtrichting en de oplossingsruimte waarbij er aandacht is voor de vraag achter vraag. Student is in staat om  gangbare normen, praktijken en meningen uit te dagen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kritisch oordelen', 4, 'niveau 3 +

Student is in staat om bij te dragen in het vormen van nieuwe gangbare normen, praktijken en meningen. Student laat zien dat getrokken conclusies tot nieuwe inzichten voor beroepenveld leiden. Student evalueert kritisch de resultaten van het onderzoek voor andere situaties en/of de mate van methodologische grondigheid van het uitgevoerde onderzoek.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Juiste kennis ontwikkelen', 1, 'Student bepaalt welke kennis, noodzakelijk voor het maken van een product of het uitvoeren van een taak, verkregen dient/moet worden en stelt voor het leren hiervan learning stories/tasks met leerdoelen op. Student toont aan welke bronnen zijn gebruikt en welke kennis die zich eigen heeft gemaakt. Student past deze kennis toe in producten/uitvoeren van een taak en kan deze kennis overdragen aan peers.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Juiste kennis ontwikkelen', 2, 'niveau 1 +

Student kijkt naar gangbare en toepasselijke kennis en technieken in het beroepenveld, maakt deze zich eigen en deelt deze kennis aan experts. Student laat zien om te kunnen gaan met tegenstrijdigheden in beschikbare informatie (bronnen).', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Juiste kennis ontwikkelen', 3, 'niveau 2 +

Student kijkt zelfstandig naar diverse nieuwe, complexe en/of uitdagende kennis en technieken in het beroepenveld.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Juiste kennis ontwikkelen', 4, 'niveau 3 +

Student laat zien in staat te zijn om kennis eigen te maken uit bronnen die niet direct voor het leren zijn ontwikkeld.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kwalitatief product maken', 1, 'Student gebruikt voor het maken van producten opgestelde acceptatie- en kwaliteitscriteria die passend zijn voor dit niveau en toont aan hoe deze criteria zijn toegepast in/op het product. Student laat zien in staat te zijn om producten op (HBO-i) niveau 1 te kunnen opleveren.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kwalitatief product maken', 2, 'Student gebruikt voor het maken van producten opgestelde acceptatie- en kwaliteitscriteria die passend zijn voor dit niveau en toont aan hoe deze criteria zijn toegepast in/op het product. Student laat zien in staat te zijn om producten op (HBO-i) niveau 2 te kunnen opleveren. Student draagt zorg voor de samenhang tussen enkele relevante producten (HBO-i).', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kwalitatief product maken', 3, 'Student gebruikt voor het maken van producten opgestelde acceptatie- en kwaliteitscriteria die passend zijn voor dit niveau en toont aan hoe deze criteria zijn toegepast in/op het product. Student laat zien in staat te zijn om producten op (HBO-i) niveau 3 te kunnen opleveren. Student draagt zorg voor de samenhang tussen alle relevante producten.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Kwalitatief product maken', 4, 'Student gebruikt voor het maken van producten opgestelde acceptatie- en kwaliteitscriteria die passend zijn voor dit niveau en toont aan hoe deze criteria zijn toegepast in/op het product. Student laat zien in staat te zijn om producten op (HBO-i) niveau 4 te kunnen opleveren. Student draagt zorg voor de samenhang tussen alle relevante producten.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Plannen', 1, 'Student maakt met direct betrokkenen regelmatig een kortetermijnplanning. Student plant het maken van story''s met heldere acceptatie en kwaliteitscriteria. Student neemt inzicht op eigen capaciteit mee en zorgt voor voldoende eigen werk. Student definieert voor story''s eigen taken van behapbare grootte en zet ze in een logische volgorde. Student communiceert dagelijks de stand van zaken en verantwoordt verstoringen in de planning tijdig aan team. Student plant peerreview van eigen werk. ', '# Opmerkingen

Er staat in de beschrijving ''Team''. Dat is niet alleen je squad, maar denk daarbij ook vooral aan de tribe en coach. Rekening houden met hoeveel capaciteit er is in de tribe voor reviews en kennisdelingen is een belangrijk onderdeel van je planning.

Met kortetermijnplanning moet je denken aan een planning voor enkele dagen tot ongeveer een sprint.

# Mogelijke bewijsstukken

- Overzicht van je checkins/checkouts (met benoeming van de taken die je gaat doen/hebt gedaan)
- Overzicht van het sprintboard (inclusief status van de stories en taken) van het begin en het eind van de sprint.');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Plannen', 2, 'niveau 1 +

Student maakt met direct betrokkenen aan een middellangetermijnplanning door een aantal cycli vooruit te plannen. Student draagt bij aan overzichten zodat de tijdlijn en de prioriteiten in de planning helder zijn en keuzes gemaakt kunnen worden. Student neemt een inschatting van capaciteiten van het team mee in de middellangetermijnplanning. Student zet deze middellangetermijnplanning daarna om naar een eigen kortetermijnplanning. ', '# Opmerkingen

Er staat in de beschrijving ''Team''. Dat is niet alleen je squad, maar denk daarbij ook vooral aan het gilde. Rekening houden met hoeveel capaciteit er is in het gilde voor gilde reviews en kennisdelingen is een belangrijk onderdeel van je planning.

Met middellangetermijnplanning moet je denken aan een planning van meerdere weken, bijvoorbeeld een milestone of een tweetal sprints.

# Mogelijke bewijsstukken

Voor het opvoeren van bewijs voor deze vaardigheid kan je bijvoorbeeld denken aan de onderstaande bewijsstukken. Deze lijst is niet volledig, maar geeft je een goede indicatie wat je zou kunnen gebruiken. Het kan natuurlijk ook een combinatie van deze stukken zijn.

- Screenshots van planning (met verwijzingen in onderbouwing)
  - Planboard met stories en taken
  - Planboard van gilde
  - Checkins waarin taken worden genoemd
  - Afspraken met opdrachtgevers');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Plannen', 3, 'niveau 2 +

Student maakt een langetermijnplanning met één of meer doelen. Student stemt deze planning af met de betrokkenen. Student onderhoudt de planning of  past deze aan met het team.', '# Opmerkingen

Met langetermijnplanning kan je denken aan een termijn van meerdere maanden bijvoorbeeld zes tot acht sprints.');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Plannen', 4, 'niveau 3 +

Student vertaalt de gekozen bedrijfsstrategie en visie naar een langetermijnplanning (mogelijk meerjaren) en past deze planning aan waar nodig. Student houdt bij het maken van deze planning rekening met de beschikbare capaciteit van het team en de wensen van direct en indirect betrokkenen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Boodschap delen', 1, 'Student brengt boodschappen over in de vorm van bijvoorbeeld documenten, presentaties en besprekingen. Student kiest passende opbouw/structuur om het doel te bereiken. Student reageert passend op feedback en vragen. Student toont aan dat ondersteunende/gebruikte producten van passende kwaliteit zijn. ', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Boodschap delen', 2, 'niveau 1 +

Student sluit aan bij kennis en context van de doelgroep, onder andere in woordkeuze, middelen en taalgebruik. Student gaat de dialoog aan door actief te luisteren en door door te vragen om de ander te begrijpen. Student geeft opbouwende feedback en is in staat om feedback te ontvangen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Boodschap delen', 3, 'niveau 2 +

Student overtuigt betrokkenen om het draagvlak voor het doel te vergroten, met aandacht voor de context waarin de betrokkenen zich bevinden. Student maakt onderscheid tussen feiten en meningen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Boodschap delen', 4, 'niveau 3 +

Student gebruikt de meningen en feiten van anderen in de onderbouwing van de boodschap. Student is in in staat om een bredere boodschap uit te kunnen dragen die verder gaat dan de eigen ideeën waarbij de boodschap niet direct in lijn is met de eigen visie en denkbeelden.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Samenwerken', 1, 'Student zoekt de samenwerking op met andere teamleden en teambegeleiding. Student draagt bij aan het gezamenlijk resultaat en een professionele werksfeer. Student kan eigen bijdrage verwoorden,  is op de hoogte van wat teamleden doen en zorgt ervoor dat taken gelijkwaardig verdeeld zijn. Student helpt peers en maakt conflicten bespreekbaar. Student geeft regelmatig informatieve feedback aan teamleden.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Samenwerken', 2, 'niveau 1 +

Student zoekt de samenwerking op met de direct betrokkenen buiten diens team of de indirect betrokkenen binnen de opleiding. Student zorgt ervoor dat deze betrokkenen op de hoogte zijn van de eigen werkzaamheden. Student stelt zich coöperatief en respectvol op. Student geeft regelmatig informatieve feedback aan indirect betrokkenen binnen de opleiding.', '# Opmerkingen

*Let op*: communicatie is tweerichtingsverkeer. ');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Samenwerken', 3, 'niveau 2 +

Student zoekt de samenwerking op met indirecte betrokkenen buiten de opleiding. Student zorgt voor professionele communicatie met deze betrokkenen. Student zorgt ervoor dat de belangen en waarden van alle betrokkenen meegenomen worden bij de uitvoering van de opdracht. Student helpt anderen zich ook verder te ontwikkelen. ', '# Opmerking

In deze fase wordt met de direct betrokkenen ook de directe gebruikers van de toepassing/oplossing bedoeld.

Met indirect betrokkenen worden die personen/instanties bedoeld die wel met de effecten/gevolgen van de oplossing te maken krijgen, maar niet direct betrokken zijn in het ontwikkelen en gebruiken van de oplossing.

# Voorbeeld

Je maakt voor de artsen in het ziekenhuis een applicatie voor het efficiënt kunnen inrichten van hun werkplek. Dit doe je in opdracht van de afdeling Ruimtelijk Beheer. De direct betrokken zijn dan onder andere:

- Artsen
- Verpleegkundigen
- Manager van de afdeling Ruimtelijk Beheer

De indirect betrokkenen zijn in dit geval o.a.:

- Patienten
- Management van het ziekenhuis
- Afdeling Schoonmaak');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Samenwerken', 4, 'niveau 3 +

Student is in staat om strategische samenwerkingen aan te gaan/op te zetten zodat relevante partijen op de passende wijze betrokken worden, daarbij rekening houdend met hun specifieke belangen en waarden.', 'In deze fase kan je bij relevante partijen denken aan bijvoorbeeld onderzoeksinstituten, gemeentes, belangenverenigingen, incubatores en NGO''s.');
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Flexibel opstellen', 1, 'Student houdt zich vanuit diens rol binnen het team gedurende de hele looptijd aan de gekozen werkwijze binnen team en school. Student beweegt mee met deze omgeving en stelt zich constructief op bij lastige keuzes of onverwachte situaties en tegenslagen. Student doet zo wat nodig is om de rol goed neer te zetten.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Flexibel opstellen', 2, 'niveau 1 +

Student houdt zich aan de gekozen werkwijze binnen team, school en opdrachtgever. Student werkt op een constructieve manier mee aan bijeenkomsten met vakgenoten. Student is voor het invullen van zijn rol in staat om over aannames, vooroordelen, eigen gemakken en ideeën heen te stappen en/of deze ter discussie te stellen. Student gaat bij invullen van rol bewust om met dilemma''s rond duurzaamheid en diversiteit.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Flexibel opstellen', 3, 'niveau 2 +

Student is zich bewust van de gegeven bedrijfsstrategie en past de eigen werkwijze daarop aan. Student stelt zich onafhankelijk op en kan omgaan met concurrerende doelen, waaronder de eigen belangen, en dilemma''s daaromheen', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Flexibel opstellen', 4, 'niveau 3 +

Student houdt zich aan de gekozen bedrijfsstrategie en is in staat om bij te dragen aan aanpassingen van de strategie en deze bij te stellen op basis van conflicterende belangen vanuit direct en indirect betrokkenen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Pro-actief handelen', 1, 'Student pakt rol en zorgt ervoor dat eigen bijdrage voldoende uitdagend is. Student ziet zelf kansen op de korte termijn en neemt zelf het initiatief voor de ontwikkeling van zichzelf en/of het team.  Student herkent zelf problemen op korte termijn en neemt zelf het tijdig het initiatief om deze te voorkomen of op te lossen. De student laat zien dat het initiatief effect heeft gehad.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Pro-actief handelen', 2, 'Student pakt rol en zorgt in eigen bijdrage voor evenwicht tussen uitdaging en niveau van eigen vaardigheid. Student ziet zelf kansen op de middellange termijn en neemt zelf het initiatief voor ontwikkeling van zichzelf en/of vakgenoten (team, gilde). Student herkent zelf problemen op middellange termijn en neemt zelf tijdig het initiatief om deze te voorkomen of op te lossen. Student laat zien dat het initiatief effect heeft gehad.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Pro-actief handelen', 3, 'Student ziet zelf kansen op de lange termijn en neemt het initiatief voor ontwikkeling van zichzelf, vakgenoten (team, gilde) en/of de opdrachtgever  Student herkent zelf problemen op lange termijn en neemt zelf het tijdig het initiatief om deze te voorkomen of op te lossen. Student laat zien dat het initiatief effect heeft gehad.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Pro-actief handelen', 4, 'Student ziet zelf kansen op de lange termijn en neemt zelf het initiatief voor ontwikkeling van direct en indirect betrokkenen. Student herkent zelf problemen op lange termijn en neemt zelf het tijdig het initiatief om deze te voorkomen of op te lossen. De student laat zien dat het initiatief effect heeft gehad. Student activeert anderen om tijdig kansen te pakken en problemen te voorkomen of op te lossen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Reflecteren', 1, 'Student haalt met name aan de hand van reviews en retrospectives regelmatig feedback op over producten en eigen functioneren. Student kijkt terug op wat goed gaat en wat beter kan. Student stelt ontwikkeldoelen op en werkt daar naartoe door actiepunten te formuleren en uit te voeren. Student herkent hoe eigen gedrag zich verhoudt tot gangbare normen, praktijken en meningen.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Reflecteren', 2, 'niveau 1 +

Student haalt regelmatig en doelgericht feedback op bij indirect betrokkenen (meestal uit gilde). Student waardeert de verkregen feedback en geeft aan hoe deze wordt meegenomen in toekomstige activiteiten. Student kijkt terug op eerdere reflecties en gaat na wat de effectiviteit is geweest van de ondernomen acties. Student herkent hoe perspectieven en gedrag van zichzelf en anderen impact hebben op het functioneren van het team, en laat dit zien in een reflectie. ', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Reflecteren', 3, 'niveau 2 +

Student zoekt bij het reflecteren naar de diepere onderliggende oorzaken van zowel positieve als negatieve aspecten van het eigen gedrag op het functioneren van zichzelf en anderen. Student neemt bij de reflectie mee hoe de student en het team bijdragen aan de uitvoering van de bedrijfsstrategie.', NULL);
INSERT INTO "SkillDescription" ("skillId", level, description, sublament) VALUES ('Reflecteren', 4, 'niveau 3 +

Student kijkt terug op hoe er door de student en het team wordt bijgedragen aan het bepalen en uitvoeren van de bedrijfsstrategie. Student onderzoekt wat in de omgeving impact heeft op het functioneren van zichzelf en het team, brengt dit in kaart en stelt daarbij het eigen gedrag bij.', NULL);





-- Completed on 2025-02-17 17:26:08 CET

--
-- PostgreSQL database dump complete
--

