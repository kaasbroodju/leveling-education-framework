-- CreateTable
CREATE TABLE "HBOIExample" (
    "id" UUID NOT NULL,
    "architectureLayerId" TEXT NOT NULL,
    "activityId" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "guild" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "sublament" TEXT,

    CONSTRAINT "HBOIExample_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "HBOIExample" ADD CONSTRAINT "HBOIExample_architectureLayerId_fkey" FOREIGN KEY ("architectureLayerId") REFERENCES "ArchitectureLayer"("architectureLayer") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HBOIExample" ADD CONSTRAINT "HBOIExample_activityId_fkey" FOREIGN KEY ("activityId") REFERENCES "Activity"("activity") ON DELETE RESTRICT ON UPDATE CASCADE;

--
-- PostgreSQL database dump
--

-- Dumped from database version 17.3 (Debian 17.3-1.pgdg120+1)
-- Dumped by pg_dump version 17.0


--
-- TOC entry 3375 (class 0 OID 18789)
-- Dependencies: 223
-- Data for Name: HBOIExample; Type: TABLE DATA; Schema: public; Owner: lef
--

INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('038b2842-44b6-4bfa-8244-123107cbb8ce', 'Organisatieprocessen', 'Ontwerpen', 1, 'BE', 'Processen modelleren (1)', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('81f20477-484f-4ba6-8b9b-21efcdccecc2', 'Infrastructuur', 'Ontwerpen', 1, 'BE', 'Ontwerpen welke code waar draait', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('ea65da61-392e-40ed-b7df-c1b8f98c5623', 'Software', 'Analyseren', 1, 'BE', 'Huidige component architectuur', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('d335716b-acc9-4bf1-bb54-aecd52543df7', 'Software', 'Analyseren', 1, 'BE', 'Test coverage', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('91e9818c-80d3-4e97-b5aa-788a6a74c2ab', 'Software', 'Analyseren', 1, 'BE', 'Code smells / statische analyse', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('1fc3f195-fefa-472f-90c3-bba126460156', 'Software', 'Analyseren', 1, 'BE', 'Code reviews', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('8161ff56-1d76-42bb-95ab-4a669bdb1edd', 'Software', 'Analyseren', 2, 'BE', 'Perfromance analyse', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('1ccb1916-8aa9-47de-a39f-f71bdcb183b4', 'Software', 'Adviseren', 1, 'BE', 'Advies over stack, frameworks,  databases, etc. gebaseerd op analyse', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('b45f79c6-917e-40a4-b6e9-6d89a6b28e7a', 'Software', 'Adviseren', 2, 'BE', 'Advies over stack, frameworks,  databases, etc. gebaseerd op MCD', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('dffe7b75-ff22-4bb1-ac79-1f60954760e5', 'Software', 'Ontwerpen', 1, 'BE', 'Domeinmodel (1 UML)', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('c0b8a329-0565-4e86-b07c-fc35cf394861', 'Software', 'Ontwerpen', 1, 'BE', 'ERD (1 Databasemodel)', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('bf4e0762-d656-40a9-be3b-a20ffb4d81c6', 'Software', 'Ontwerpen', 1, 'BE', 'Teststrategie', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('e2beabfc-de28-4e2a-b73e-520a9eb58754', 'Software', 'Ontwerpen', 1, 'BE', 'UML diagram', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('1f32316f-940f-4b6c-886d-26ac28248091', 'Software', 'Ontwerpen', 2, 'BE', 'Architectuur Patronen/bestaande componenten (2,3 Design Patterns, Design Principes, Message Brokers)', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('7ab17026-b4d7-4c43-bd15-e8e2c894fbf6', 'Software', 'Ontwerpen', 2, 'BE', 'C4 - Architectuur', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('b8587714-837c-497b-a382-69b90f7a0a51', 'Software', 'Ontwerpen', 2, 'BE', 'Teststrategie', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('d7f78fd0-b107-4113-bf68-adc6d13bf238', 'Software', 'Ontwerpen', 3, 'BE', 'Architectuur Patronen/bestaande componenten (2,3 Design Patterns, Design Principes, Message Brokers)', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('472ee6a9-df99-48b0-8286-e6dd48a88339', 'Software', 'Ontwerpen', 3, 'BE', 'Teststrategie', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('abd109a4-3b72-4687-955f-eeaed89e6417', 'Software', 'Realiseren', 1, 'BE', 'Controllerlaag met unittests', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('46d72f7a-4452-49f2-a66f-aff21dcf0b27', 'Software', 'Realiseren', 1, 'BE', 'OOP, imperative, functioneel', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('da43780a-6469-473f-b2e4-6ec0c896a4aa', 'Software', 'Realiseren', 1, 'BE', 'Security by design (OWASP)', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('01499b03-acc5-4aca-b16c-2e1b0ba0347a', 'Software', 'Realiseren', 1, 'BE', 'Prototypen', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('4e6df8f5-3bf7-447f-ab7f-603e790c7af4', 'Software', 'Realiseren', 2, 'BE', 'Services/API''s restfull principes', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('dd1df801-579c-4c12-b7eb-618f7ac53a87', 'Software', 'Manage & Control', 1, 'BE', 'Versiebeheer (1)', '# VCS
Je werkt met een VCS (version control system), zoals GIT (standaard) of Mercurial (Facebook).
## Workflow & DTAP
Je hebt beschreven op de wiki hoe men omgaat qua branches, commit messages en peer reviews. Je hebt je (proces)documentatie uitgewerkt tot en met de D van DTAP.');
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('61109fd9-3869-417b-9f20-ba420a42f1d7', 'Software', 'Manage & Control', 1, 'BE', 'Conventies/Documentatie(1)', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('e8f01612-9e78-46e2-b49c-220b3846f663', 'Software', 'Manage & Control', 1, 'BE', 'Open API / swagger', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('c18b2fb7-1c66-4002-b8f6-1a05a613cfff', 'Software', 'Manage & Control', 2, 'BE', 'CI', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('a2322787-0aef-4957-82b6-dfdce3294ba3', 'Software', 'Manage & Control', 2, 'BE', 'Docker container/images', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('6a96fc22-c317-4b68-8d94-ba901e0a5b96', 'Software', 'Manage & Control', 3, 'BE', 'CI + CD', NULL);
INSERT INTO "HBOIExample" (id, "architectureLayerId", "activityId", level, guild, title, sublament) VALUES ('bc70bb78-45c9-4e8e-9e1a-8cc7a196bf68', 'Software', 'Manage & Control', 3, 'BE', 'Testplan', NULL);


-- Completed on 2025-02-17 19:21:20 CET

--
-- PostgreSQL database dump complete
--


