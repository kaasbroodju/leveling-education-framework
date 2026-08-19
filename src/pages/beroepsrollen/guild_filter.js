function mountGuildFilter() {
	const buttons = document.querySelectorAll('[data-filter-guild]');
	const cards = document.querySelectorAll('[data-guild]');

	const params = new URLSearchParams(window.location.search);
	let activeGuild = params.get('guild');

	function updateUrl() {
		const url = new URL(window.location.href);
		if (activeGuild) {
			url.searchParams.set('guild', activeGuild);
		} else {
			url.searchParams.delete('guild');
		}
		history.replaceState(null, '', url.toString());
	}

	function applyFilter() {
		cards.forEach(card => {
			const cardGuild = card.getAttribute('data-guild');
			card.style.display = (!activeGuild || cardGuild === activeGuild) ? '' : 'none';
		});

		buttons.forEach(b => {
			b.toggleAttribute('lef-link-active', b.getAttribute('data-filter-guild') === activeGuild);
		});
	}

	buttons.forEach(btn => {
		btn.addEventListener('click', () => {
			const value = btn.getAttribute('data-filter-guild');
			activeGuild = activeGuild === value ? null : value;
			updateUrl();
			applyFilter();
		});
	});

	applyFilter();
}

document.addEventListener('DOMContentLoaded', mountGuildFilter);
