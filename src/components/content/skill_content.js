document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll("button[lef-modal]").forEach(
        (element) => element.onclick = () => {
            let modal = document.querySelector(`dialog[id="${element.attributes['lef-modal'].value}"]`);
            modal.showModal()
            modal.open = true
        }
    )
});



const tips = [
    '🐛 "It\'s not a bug, it\'s a feature!" - Elke developer ooit',
    '☕ Koffie + Code = Succes (wetenschappelijk bewezen*) *niet echt',
    '🔄 Git commit vaak, push regelmatig, slaap rustig',
    '📝 Comments schrijven = cadeau voor toekomstige jezelf',
    '🎯 Clean code > Clever code (altijd!)',
    '🤝 Peer reviews = gratis code improvement + knowledge sharing',
    '🔒 Security by design, niet security by accident'
];

const tip = tips[Math.floor(Math.random() * tips.length)];


const facts = [
    '🏢 Open-ICT heeft ~400 studenten verdeeld over meerdere tribes',
    '☕ Gemiddelde student drinkt 3.7 koppen koffie per dag (geschat)',
    '🎯 85% van de studenten vindt een ICT-baan binnen 6 maanden na afstuderen',
    '🚀 Elke 2 weken nieuwe sprint = 26 kansen per jaar om te groeien',
    '🤝 Squad-based learning = je bent nooit alleen in je development journey',
    '🛠️ 8 verschillende gildes = 8 verschillende superpower specialisaties',
    '📱 Van apps tot AI, van games tot cybersecurity - we doen het allemaal!',
    '🌍 Opdrachtgevers van startups tot multinationals werken met onze studenten'
];

const randomFact = facts[Math.floor(Math.random() * facts.length)];

console.log(`
%c
     ██████  ██████  ███████ ███    ██       ██  ██████ ████████ 
    ██    ██ ██   ██ ██      ████   ██       ██ ██         ██    
    ██    ██ ██████  █████   ██ ██  ██ █████ ██ ██         ██    
    ██    ██ ██      ██      ██  ██ ██       ██ ██         ██    
     ██████  ██      ███████ ██   ████       ██  ██████    ██    
     
%c    ╭─────────────────────────────────────╮
    │  🌟 OPEN-ICT DEVELOPER DISCOVERED     │  
    ╰─────────────────────────────────────╯

    %c⚡ System Stats:
    %c   • Squad Power Level: %cOVER 9000! %c⚡
    %c   • Coffee Consumed: %c${Math.floor(Math.random() * 999) + 1} cups %c☕
    %c   • Bug Fixes Today: %c${Math.floor(Math.random() * 42) + 1} %c🐛
    %c   • Sprint Progress: %c${Math.floor(Math.random() * 100) + 1}% %c📈

    %c💡 Open-ICT Fun Fact:
    %c   • ${randomFact}
    
    %c💡 Open-ICT Developer Wisdom:
    %c   • ${tip}
    
`,
    'color: #00ff88; font-family: monospace; font-size: 10px; text-shadow: 0 0 10px #00ff88;',
    'color: #ff6b9d; font-weight: bold; text-shadow: 1px 1px 2px rgba(0,0,0,0.3);',
    'color: #87CEEB; font-weight: bold;',
    'color: #666;', 'color: #ff6b6b; font-weight: bold; text-shadow: 0 0 10px #ff6b6b;', 'color: #666;',
    'color: #666;', 'color: #FFD700; font-weight: bold;', 'color: #666;',
    'color: #666;', 'color: #90EE90; font-weight: bold;', 'color: #666;',
    'color: #666;', 'color: #DDA0DD; font-weight: bold;', 'color: #666;',
    'color: #87CEEB; font-weight: bold;',
    'color: #DDA0DD;',
    'color: #87CEEB; font-weight: bold;',
    'color: #DDA0DD;',

);