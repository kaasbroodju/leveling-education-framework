function mountBeroepstakenFilter() {
	const architectuurlaagButtons = document.querySelectorAll('[data-filter-architectuurlaag]');
	const activiteitButtons = document.querySelectorAll('[data-filter-activiteit]');
	const cards = document.querySelectorAll('[data-architectuurlaag]');

	const params = new URLSearchParams(window.location.search);
	let activeArchitectuurlaag = params.get('architectuurlaag');
	let activeActiviteit = params.get('activiteit');

	function updateUrl() {
		const url = new URL(window.location.href);
		if (activeArchitectuurlaag) {
			url.searchParams.set('architectuurlaag', activeArchitectuurlaag);
		} else {
			url.searchParams.delete('architectuurlaag');
		}
		if (activeActiviteit) {
			url.searchParams.set('activiteit', activeActiviteit);
		} else {
			url.searchParams.delete('activiteit');
		}
		history.replaceState(null, '', url.toString());
	}

	function applyFilter() {
		cards.forEach(card => {
			const cardLaag = card.getAttribute('data-architectuurlaag');
			const cardActiviteit = card.getAttribute('data-activiteit');

			const laagMatch = !activeArchitectuurlaag || cardLaag === activeArchitectuurlaag;
			const activiteitMatch = !activeActiviteit || cardActiviteit === activeActiviteit;

			card.style.display = (laagMatch && activiteitMatch) ? '' : 'none';
		});

		architectuurlaagButtons.forEach(b => {
			b.toggleAttribute('lef-link-active', b.getAttribute('data-filter-architectuurlaag') === activeArchitectuurlaag);
		});
		activiteitButtons.forEach(b => {
			b.toggleAttribute('lef-link-active', b.getAttribute('data-filter-activiteit') === activeActiviteit);
		});
	}

	architectuurlaagButtons.forEach(btn => {
		btn.addEventListener('click', () => {
			const value = btn.getAttribute('data-filter-architectuurlaag');
			activeArchitectuurlaag = activeArchitectuurlaag === value ? null : value;
			updateUrl();
			applyFilter();
		});
	});

	activiteitButtons.forEach(btn => {
		btn.addEventListener('click', () => {
			const value = btn.getAttribute('data-filter-activiteit');
			activeActiviteit = activeActiviteit === value ? null : value;
			updateUrl();
			applyFilter();
		});
	});

	applyFilter();
}

document.addEventListener('DOMContentLoaded', mountBeroepstakenFilter);
