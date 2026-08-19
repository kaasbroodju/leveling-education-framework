const LEF_LEVEL_VIEW_MODES = {
    '1': ['Level1'],
    '1-2': ['Level1', 'Level2'],
    '1-2-3': ['Level1', 'Level2', 'Level3'],
    '2-3': ['Level2', 'Level3'],
    '3-4': ['Level3', 'Level4'],
};

function applyTheme(theme) {
    if (theme === 'light' || theme === 'dark') {
        document.documentElement.setAttribute('data-theme', theme);
    } else {
        document.documentElement.removeAttribute('data-theme');
    }
}

function applyLevelView(mode) {
    const allowed = LEF_LEVEL_VIEW_MODES[mode];
    document.querySelectorAll('[data-level]').forEach(el => {
        el.style.display = (!allowed || allowed.includes(el.getAttribute('data-level'))) ? '' : 'none';
    });
    document.body.setAttribute('data-level-view', mode);
}

function mountSettings() {
    const dialog = document.querySelector('#settings-modal');
    const button = document.querySelector('#settings-button');
    button.onclick = () => dialog.showModal();

    const currentTheme = localStorage.getItem('lef-theme') || 'system';
    const currentLevelView = localStorage.getItem('lef-level-view') || 'all';

    document.querySelectorAll('input[name="lef-theme"]').forEach(input => {
        input.checked = input.value === currentTheme;
        input.addEventListener('change', () => {
            localStorage.setItem('lef-theme', input.value);
            applyTheme(input.value);
        });
    });

    document.querySelectorAll('input[name="lef-level-view"]').forEach(input => {
        input.checked = input.value === currentLevelView;
        input.addEventListener('change', () => {
            localStorage.setItem('lef-level-view', input.value);
            applyLevelView(input.value);
        });
    });

    applyLevelView(currentLevelView);
}

document.addEventListener('DOMContentLoaded', mountSettings);
