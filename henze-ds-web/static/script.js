// Henze DS Web - Event handling
document.addEventListener('DOMContentLoaded', function() {
    // Auto-expand live events on page load
    const liveEvents = document.querySelectorAll('.live-event .accordion-button');
    if (liveEvents.length > 0 && liveEvents.length <= 3) {
        // Auto-expand all live events if there are 3 or fewer
        liveEvents.forEach(button => {
            const targetId = button.getAttribute('data-bs-target');
            const target = document.querySelector(targetId);
            if (target) {
                new bootstrap.Collapse(target, { toggle: false }).show();
                button.classList.remove('collapsed');
                button.setAttribute('aria-expanded', 'true');
            }
        });
    }

    // Prevent accordion toggle when clicking on event links
    document.querySelectorAll('.event-link').forEach(link => {
        link.addEventListener('click', function(e) {
            e.stopPropagation();
        });
    });

    // Add keyboard navigation for accessibility
    document.querySelectorAll('.accordion-button').forEach(button => {
        button.addEventListener('keypress', function(e) {
            if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                button.click();
            }
        });
    });

    // Feeling Lucky button - fetch current API results and highlight a random market in the table
    const luckyBtn = document.getElementById('feeling-lucky');
    if (luckyBtn) {
        luckyBtn.addEventListener('click', async function() {
            const target = document.getElementById('target')?.value || '';
            const tolerance = document.getElementById('tolerance')?.value || '';
            const sport = document.getElementById('sport')?.value || '';
            const params = new URLSearchParams();
            if (target) params.set('target', target);
            if (tolerance) params.set('tolerance', tolerance);
            if (sport) params.set('sport', sport);
            try {
                const resp = await fetch(`/api/bets?${params.toString()}`);
                if (!resp.ok) return;
                const bets = await resp.json();
                if (!bets || bets.length === 0) return;
                const choice = bets[Math.floor(Math.random() * bets.length)];
                if (!choice) return;

                // Clear previous lucky highlights
                document.querySelectorAll('tr.ds-lucky-row').forEach(r => {
                    r.classList.remove('ds-lucky-row', 'ds-lucky-highlight');
                    const sticker = r.querySelector('.ds-lucky-ribbon');
                    if (sticker) sticker.remove();
                });

                const eventId = String(choice.event_id);
                const marketName = String(choice.market_name);
                const outcome = String(choice.outcome);
                const decimal = String(choice.decimal);

                // Find matching row in DOM
                const selector = `tr[data-event-id="${eventId}"]`;
                const candidates = Array.from(document.querySelectorAll(selector));

                let matchedRow = null;
                for (const row of candidates) {
                    const mname = row.getAttribute('data-market-name') || '';
                    const out = row.getAttribute('data-outcome') || '';
                    const dec = row.getAttribute('data-decimal') || '';
                    if (mname === marketName && out === outcome) {
                        matchedRow = row;
                        break;
                    }
                    // fallback: match by decimal if names differ due to formatting
                    if (!matchedRow && dec === decimal) matchedRow = row;
                }

                if (matchedRow) {
                    // Expand the accordion for the event
                    const collapseId = `#event-${eventId}`;
                    const collapseEl = document.querySelector(collapseId);
                    if (collapseEl) {
                        const bsCollapse = bootstrap.Collapse.getOrCreateInstance(collapseEl);
                        bsCollapse.show();
                    }

                    // Scroll into view and highlight
                    matchedRow.scrollIntoView({ behavior: 'smooth', block: 'center' });
                    matchedRow.classList.add('ds-lucky-row', 'ds-lucky-highlight');

                    // Add floating, tilted sticker overlayed across the row
                    // Remove previous floating stickers first
                    document.querySelectorAll('.ds-lucky-ribbon-floating').forEach(s => s.remove());

                    const sticker = document.createElement('div');
                    sticker.className = 'ds-lucky-ribbon-floating';
                    sticker.textContent = 'Lucky Bet';

                    // Add to body so it overlays across table layout
                    document.body.appendChild(sticker);

                    // Position sticker centered over the matched row (account for scrolling)
                    const rowRect = matchedRow.getBoundingClientRect();
                    const scrollTop = window.scrollY || window.pageYOffset;
                    const centerX = rowRect.left + rowRect.width * 0.5;
                    const centerY = rowRect.top + scrollTop + rowRect.height * 0.5;

                    // Use translate(-50%, -50%) to center the sticker at (left, top)
                    sticker.style.left = `${centerX}px`;
                    sticker.style.top = `${centerY}px`;
                    sticker.style.transform = 'rotate(-12deg) translate(-50%, -50%) scale(0.85)';
                    sticker.style.opacity = '0';

                    // Force layout then animate in
                    requestAnimationFrame(() => {
                        sticker.style.transition = 'transform 220ms ease, opacity 220ms ease';
                        sticker.style.transform = 'rotate(-12deg) translate(-50%, -50%) scale(1)';
                        sticker.style.opacity = '1';
                    });

                    // Auto-remove sticker after 5 seconds
                    setTimeout(() => {
                        sticker.style.opacity = '0';
                        sticker.style.transform = 'rotate(-12deg) translate(-50%, -50%) scale(0.9)';
                        setTimeout(() => sticker.remove(), 300);
                    }, 5000);
                } else {
                    console.warn('Lucky choice not found in DOM', choice);
                }
            } catch (e) {
                console.error('Feeling Lucky failed', e);
            }
        });
    }
});
