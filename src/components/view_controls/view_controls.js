const LEF_LEVEL_VIEW_MODES = {
    '1': ['Level1'],
    '1-2': ['Level1', 'Level2'],
    '1-2-3': ['Level1', 'Level2', 'Level3'],
    '2-3': ['Level2', 'Level3'],
    '3-4': ['Level3', 'Level4'],
};

const THEME_ORDER = ['system', 'light', 'dark'];
const THEME_LABELS = {
    system: 'Thema: systeem (klik om te wijzigen)',
    light: 'Thema: licht (klik om te wijzigen)',
    dark: 'Thema: donker (klik om te wijzigen)',
};

function applyTheme(theme) {
    if (theme === 'light' || theme === 'dark') {
        document.documentElement.setAttribute('data-theme', theme);
    } else {
        document.documentElement.removeAttribute('data-theme');
    }
    document.documentElement.setAttribute('data-theme-choice', theme);

    const button = document.querySelector('#theme-toggle');
    if (button) {
        button.setAttribute('aria-label', THEME_LABELS[theme] || THEME_LABELS.system);
    }
}

function applyLevelView(mode) {
    const allowed = LEF_LEVEL_VIEW_MODES[mode];
    document.querySelectorAll('[data-level]').forEach(el => {
        el.style.display = (!allowed || allowed.includes(el.getAttribute('data-level'))) ? '' : 'none';
    });
    document.body.setAttribute('data-level-view', mode);
}

function mountViewControls() {
    const themeButton = document.querySelector('#theme-toggle');
    const currentTheme = localStorage.getItem('lef-theme') || 'system';
    applyTheme(currentTheme);

    themeButton.addEventListener('click', () => {
        const current = document.documentElement.getAttribute('data-theme-choice') || 'system';
        const next = THEME_ORDER[(THEME_ORDER.indexOf(current) + 1) % THEME_ORDER.length];
        localStorage.setItem('lef-theme', next);
        applyTheme(next);
    });

    const levelSelect = document.querySelector('#level-view-select');
    const currentLevelView = localStorage.getItem('lef-level-view') || 'all';
    levelSelect.value = currentLevelView;
    levelSelect.addEventListener('change', () => {
        localStorage.setItem('lef-level-view', levelSelect.value);
        applyLevelView(levelSelect.value);
    });

    applyLevelView(currentLevelView);
}

document.addEventListener('DOMContentLoaded', mountViewControls);
