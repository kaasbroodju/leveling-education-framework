function mountSkillFilter() {
	const buttons = document.querySelectorAll('[data-filter-vaardigheid]');
	const cards = document.querySelectorAll('[data-vaardigheid]');

	const params = new URLSearchParams(window.location.search);
	let activeVaardigheid = params.get('vaardigheid');

	function updateUrl() {
		const url = new URL(window.location.href);
		if (activeVaardigheid) {
			url.searchParams.set('vaardigheid', activeVaardigheid);
		} else {
			url.searchParams.delete('vaardigheid');
		}
		history.replaceState(null, '', url.toString());
	}

	function applyFilter() {
		cards.forEach(card => {
			const cardVaardigheid = card.getAttribute('data-vaardigheid');
			card.style.display = (!activeVaardigheid || cardVaardigheid === activeVaardigheid) ? '' : 'none';
		});

		buttons.forEach(b => {
			b.toggleAttribute('lef-link-active', b.getAttribute('data-filter-vaardigheid') === activeVaardigheid);
		});
	}

	buttons.forEach(btn => {
		btn.addEventListener('click', () => {
			const value = btn.getAttribute('data-filter-vaardigheid');
			activeVaardigheid = activeVaardigheid === value ? null : value;
			updateUrl();
			applyFilter();
		});
	});

	applyFilter();
}

document.addEventListener('DOMContentLoaded', mountSkillFilter);
