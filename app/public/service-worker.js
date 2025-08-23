// service-worker.js
const CACHE_NAME = "site-cache-v1";
const BASE_URLS = ["/", "/beroepstaken", "/beroepsproducten", "/about"];

// Alle varianten definiëren
const HOME_VARIANTS = [
	"Overzicht creëren",
	"Kritisch oordelen",
	"Juiste kennis ontwikkelen",
	"Kwalitatief product maken",
	"Plannen",
	"Boodschap delen",
	"Samenwerken",
	"Flexibel opstellen",
	"Pro-actief handelen",
	"Reflecteren",
].map(e => `/?vaardigheid=${e}`);
const Activities = [
	"Analyseren",
	"Adviseren",
	"Ontwerpen",
	"Realiseren",
	"Manage_and_Control",
];
const Layers = [
	"Gebruikersinteractie",
	"Organisatieprocessen",
	"Infrastructuur",
	"Software",
	"Hardwareinterfacing",
];
const TASK_VARIANTS = [];
const PRODUCT_VARIANTS = [];

for (let i = 0; i < 5; i++) {
	for (let j = 0; j < 5; j++) {
		TASK_VARIANTS.push(
			`/beroepstaken?architectuurlaag=${Layers[i]}&activiteit=${Activities[j]}`,
		);
		PRODUCT_VARIANTS.push(
			`/beroepsproducten?architectuurlaag=${Layers[i]}&activiteit=${Activities[j]}`,
		);
	}
	TASK_VARIANTS.push(`/beroepstaken?architectuurlaag=${Layers[i]}`);
	PRODUCT_VARIANTS.push(`/beroepsproducten?architectuurlaag=${Layers[i]}`);
	TASK_VARIANTS.push(`/beroepstaken?&activiteit=${Activities[i]}`);
	PRODUCT_VARIANTS.push(`/beroepsproducten?&activiteit=${Activities[i]}`);
}

const ALL_URLS = [
	...BASE_URLS,
	...HOME_VARIANTS,
	...TASK_VARIANTS,
	...PRODUCT_VARIANTS,
];

// Install event - preload base URLs
self.addEventListener("install", (event) => {
	event.waitUntil(
		caches.open(CACHE_NAME).then((cache) => cache.addAll(ALL_URLS)),
	);
});

// Fetch event met stale-while-revalidate
self.addEventListener("fetch", (event) => {
	event.respondWith(
		caches.match(event.request).then((response) => {
			// Return cached version immediately
			if (response) {
				// Revalidate in background
				fetch(event.request).then((fetchResponse) => {
					const responseClone = fetchResponse.clone();
					caches
						.open(CACHE_NAME)
						.then((cache) => cache.put(event.request, responseClone));
				});
				return response;
			}

			// If not cached, fetch and cache
			return fetch(event.request).then((fetchResponse) => {
				const responseClone = fetchResponse.clone();
				caches
					.open(CACHE_NAME)
					.then((cache) => cache.put(event.request, responseClone));
				return fetchResponse;
			});
		}),
	);
});
