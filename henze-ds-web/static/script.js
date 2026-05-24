// Henze DS Web — API-backed paginated event loading
document.addEventListener('DOMContentLoaded', function () {

    // ========== Toast Notification System ==========
    const toastContainer = document.getElementById('toast-container');

    function showToast(title, message, type = 'info', duration = 4000) {
        if (!toastContainer) return;
        const icons = {
            success: 'bi-check-circle-fill',
            info: 'bi-info-circle-fill',
            warning: 'bi-exclamation-triangle-fill',
            error: 'bi-x-circle-fill'
        };
        const toast = document.createElement('div');
        toast.className = 'toast';
        toast.innerHTML = `
            <i class="bi ${icons[type] || icons.info} toast-icon ${type}"></i>
            <div class="toast-content">
                <div class="toast-title">${title}</div>
                ${message ? `<div class="toast-message">${message}</div>` : ''}
            </div>
            <button class="toast-close" aria-label="Close"><i class="bi bi-x"></i></button>
            <div class="toast-progress" style="animation-duration: ${duration}ms"></div>
        `;
        toastContainer.appendChild(toast);
        requestAnimationFrame(() => toast.classList.add('show'));
        toast.querySelector('.toast-close').addEventListener('click', () => dismissToast(toast));
        setTimeout(() => dismissToast(toast), duration);
        return toast;
    }

    function dismissToast(toast) {
        if (!toast || toast.classList.contains('hiding')) return;
        toast.classList.remove('show');
        toast.classList.add('hiding');
        setTimeout(() => toast.remove(), 400);
    }

    // ========== Loading Overlay ==========
    const loadingOverlay = document.getElementById('loading-overlay');
    function showLoading() { loadingOverlay?.classList.add('active'); }
    function hideLoading() { loadingOverlay?.classList.remove('active'); }

    // ========== UI element references ==========
    const accordion         = document.getElementById('eventsAccordion');
    const resultsBadge      = document.getElementById('results-badge');
    const emptyState        = document.getElementById('empty-state');
    const loadMoreSentinel = document.getElementById('load-more-sentinel');
    const loadMoreStatus   = document.getElementById('load-more-status');
    const eventsInfoText    = document.getElementById('events-info-text');
    const leagueFilter      = document.getElementById('league-filter');
    const timePresetSelect  = document.getElementById('time-preset-select');
    const customTimeRange   = document.getElementById('custom-time-range');
    const fromTimeInput     = document.getElementById('from-time-input');
    const toTimeInput       = document.getElementById('to-time-input');
    const liveStatusRadios  = document.querySelectorAll('input[name="live_status"]');
    const advFilterToggle   = document.getElementById('advanced-filters-toggle');
    const advFilterCollapse = document.getElementById('advancedFilters');
    const activeFilterBadge = document.getElementById('active-filter-count');
    const clearFiltersBtn   = document.getElementById('clear-filters');
    const filterForm        = document.getElementById('filter-form');
    const apiLink           = document.getElementById('api-link');
    const combosTbody       = document.getElementById('combinations-tbody');
    const combosBadge       = document.getElementById('combinations-badge');
    const combosInfo        = document.getElementById('combinations-info');
    const combosEmptyState  = document.getElementById('combinations-empty-state');
    const combosTableWrap   = document.getElementById('combinations-table-wrap');
    const comboClearBtn     = document.getElementById('combo-clear-btn');
    const comboAddLegBtn    = document.getElementById('combo-add-leg-btn');
    const omitLiveCombosInput = document.getElementById('omit-live-combos');
    const COMBO_TARGET_ODDS = 5.00;
    const MAX_COMBO_BETS = 25;

    // ========== Pagination state ==========
    const PAGE_SIZE = 40;
    let currentPage = 0;
    let totalEvents = 0;
    let hasMore     = false;
    let isFetching  = false;

    let comboPage = -1;
    let comboMatches = [];
    let comboIsSearching = false;
    let comboCandidatePool = [];
    let comboCandidateSignature = '';

    // ========== Current advanced filter state ==========
    let advancedFilters = {
        time_preset: 'all',
        from_time:   '',
        to_time:     '',
        live_only:   false,
        class_id:    '',
        omit_live_events: false,
    };

    // ========== HTML escape ==========
    function esc(str) {
        return String(str ?? '')
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#039;');
    }

    // ========== Build /api/events URL ==========
    function buildApiUrl(page, pageSize = PAGE_SIZE) {
        const formData  = new FormData(filterForm);
        const target    = formData.get('target')    || '1.1';
        const tolerance = formData.get('tolerance') || '0.04';
        const sport     = formData.get('sport')     || '';

        const params = new URLSearchParams({ target, tolerance, page: String(page), page_size: String(pageSize) });
        if (sport)                                  params.set('sport',       sport);
        if (advancedFilters.time_preset !== 'all')  params.set('time_preset', advancedFilters.time_preset);
        if (advancedFilters.from_time)              params.set('from_time',   advancedFilters.from_time);
        if (advancedFilters.to_time)                params.set('to_time',     advancedFilters.to_time);
        if (advancedFilters.live_only)              params.set('live_only',   'true');
        if (advancedFilters.class_id)               params.set('class_id',    advancedFilters.class_id);
        return `/api/events?${params.toString()}`;
    }

    // ========== Update the API footer link ==========
    function updateApiLink() {
        if (!apiLink) return;
        const formData  = new FormData(filterForm);
        const target    = formData.get('target')    || '1.1';
        const tolerance = formData.get('tolerance') || '0.04';
        const sport     = formData.get('sport')     || '';
        const p = new URLSearchParams({ target, tolerance });
        if (sport) p.set('sport', sport);
        apiLink.href = `/api/bets?${p.toString()}`;
    }

    // ========== Render a single event accordion item ==========
    function renderEventCard(event) {
        const mc = event.markets.length;
        const liveBadge = event.is_live
            ? `<span class="live-badge me-2"><i class="bi bi-circle-fill pulse me-1"></i>LIVE${
                event.match_minute != null
                    ? `<span class="match-minute">${event.match_minute}'</span>` : ''
              }</span>`
            : '';

        const rows = event.markets.map(m => `
            <tr data-event-id="${esc(event.event_id)}"
                data-market-name="${esc(m.market_name)}"
                data-market-type="${esc(m.market_type)}"
                data-market-sub-type="${esc(m.market_sub_type)}"
                data-outcome="${esc(m.outcome)}"
                data-decimal="${m.decimal}">
                <td>${esc(m.market_name)}</td>
                <td>${esc(m.outcome)}</td>
                <td class="text-end fw-bold text-success odds-cell">${m.decimal.toFixed(2)}</td>
                <td class="text-end">
                    <button type="button" class="btn btn-sm btn-outline-primary add-to-combo-btn">Add Bet</button>
                </td>
            </tr>`).join('');

        const div = document.createElement('div');
        div.className = `accordion-item${event.is_live ? ' live-event' : ''}`;
        div.dataset.eventId      = event.event_id;
        div.dataset.categoryId   = event.category_id;
        div.dataset.categoryName = event.category_name;
        div.dataset.classId      = event.class_id;
        div.dataset.className2   = event.class_name;
        div.dataset.isLive       = String(event.is_live);
        div.dataset.eventTimeUtc = event.event_time_utc;

        div.innerHTML = `
            <h2 class="accordion-header">
                <button class="accordion-button collapsed${event.is_live ? ' live-header' : ''}"
                        type="button" data-bs-toggle="collapse"
                        data-bs-target="#event-${esc(event.event_id)}" aria-expanded="false">
                    <div class="event-header-content">
                        <div class="event-info">
                            ${liveBadge}
                            <span class="sport-badge me-2">${esc(event.category_name)}</span>
                            <span class="league-badge me-2">${esc(event.class_name)}</span>
                            <a href="${esc(event.event_url)}" target="_blank" class="event-link">
                                ${esc(event.event_name)} <i class="bi bi-box-arrow-up-right small"></i>
                            </a>
                        </div>
                        <div class="event-meta">
                            <span class="event-time"><i class="bi bi-clock me-1"></i>${esc(event.event_time)}</span>
                            <span class="market-count badge bg-secondary ms-2">${mc} market${mc !== 1 ? 's' : ''}</span>
                        </div>
                    </div>
                </button>
            </h2>
            <div id="event-${esc(event.event_id)}" class="accordion-collapse collapse"
                 data-bs-parent="#eventsAccordion">
                <div class="accordion-body p-0">
                    <div class="market-table-controls d-flex align-items-center gap-2 px-3 py-2 border-bottom">
                        <button type="button" class="btn btn-sm btn-outline-secondary market-group-toggle" aria-pressed="false">
                            <i class="bi bi-collection me-1"></i>Group by type
                        </button>
                    </div>
                    <table class="table table-sm table-striped mb-0 market-table">
                        <thead class="table-light">
                            <tr>
                                <th class="sort-header" data-sort-col="market" role="button" tabindex="0" aria-sort="none">
                                    Market <i class="bi bi-arrow-down-up sort-icon ms-1"></i>
                                </th>
                                <th class="sort-header" data-sort-col="outcome" role="button" tabindex="0" aria-sort="none">
                                    Outcome <i class="bi bi-arrow-down-up sort-icon ms-1"></i>
                                </th>
                                <th class="sort-header text-end" data-sort-col="odds" role="button" tabindex="0" aria-sort="none">
                                    Odds <i class="bi bi-arrow-down-up sort-icon ms-1"></i>
                                </th>
                                <th class="text-end">Builder</th>
                            </tr>
                        </thead>
                        <tbody>${rows}</tbody>
                    </table>
                </div>
            </div>`;
        return div;
    }

    // ========== Insert events into accordion ==========
    function insertEvents(events, append) {
        if (!append) accordion.innerHTML = '';
        const frag = document.createDocumentFragment();
        events.forEach(e => frag.appendChild(renderEventCard(e)));
        accordion.appendChild(frag);
        ensureAccordionListeners();
        initMarketTables(events.map(e => `event-${e.event_id}`));

        if (!append) {
            const liveButtons = accordion.querySelectorAll('.live-event .accordion-button');
            if (liveButtons.length > 0 && liveButtons.length <= 3) {
                liveButtons.forEach(btn => {
                    const targetEl = document.querySelector(btn.getAttribute('data-bs-target'));
                    if (targetEl) {
                        bootstrap.Collapse.getOrCreateInstance(targetEl, { toggle: false }).show();
                        btn.classList.remove('collapsed');
                        btn.setAttribute('aria-expanded', 'true');
                    }
                });
            }
        }
    }

    // ========== Update UI counters ==========
    function updateCounters() {
        const renderedEvents  = accordion.querySelectorAll('.accordion-item[data-event-id]').length;
        const renderedMarkets = accordion.querySelectorAll('tbody tr[data-decimal]').length;
        if (resultsBadge) {
            resultsBadge.textContent = `${renderedEvents} event${renderedEvents !== 1 ? 's' : ''} (${renderedMarkets} market${renderedMarkets !== 1 ? 's' : ''})`;
        }
        if (eventsInfoText) {
            eventsInfoText.innerHTML = `<i class="bi bi-info-circle me-1"></i> Showing ${renderedEvents} of ${totalEvents} events`;
        }
        if (emptyState)      emptyState.classList.toggle('d-none', renderedEvents > 0 || hasMore);
        if (loadMoreSentinel) loadMoreSentinel.classList.toggle('d-none', !hasMore);
        if (loadMoreStatus && hasMore) {
            loadMoreStatus.textContent = `Showing ${renderedEvents} of ${totalEvents} events`;
        }
    }

    // ========== Populate class (league) dropdown ==========
    function populateClassFilter(classes) {
        if (!leagueFilter || !classes?.length) return;
        const selected = advancedFilters.class_id;
        leagueFilter.innerHTML = `<option value="">All Leagues (${classes.length})</option>`;
        classes.forEach(cls => {
            const opt = document.createElement('option');
            opt.value       = cls.id;
            opt.textContent = `${cls.name} (${cls.count})`;
            if (cls.id === selected) opt.selected = true;
            leagueFilter.appendChild(opt);
        });
    }

    // ========== Fetch events from /api/events ==========
    async function fetchEvents(page, append) {
        if (isFetching) return;
        isFetching = true;
        if (!append) showLoading();

        try {
            const url  = buildApiUrl(page);
            const resp = await fetch(url);
            if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
            const data = await resp.json();

            currentPage = data.page;
            totalEvents = data.total_events;
            hasMore     = data.has_more;

            insertEvents(data.events, append);

            if (page === 0 && data.classes?.length > 0) {
                populateClassFilter(data.classes);
            }

            updateCounters();

            if (!append && page === 0) {
                const mc = accordion.querySelectorAll('tbody tr[data-decimal]').length;
                if (mc > 0) {
                    const lc  = accordion.querySelectorAll('.live-event').length;
                    let msg   = `${mc} market${mc !== 1 ? 's' : ''} across ${totalEvents} event${totalEvents !== 1 ? 's' : ''}`;
                    if (lc > 0) msg += ` (${lc} live)`;
                    showToast('Results loaded', msg, 'success');
                }
            }
        } catch (err) {
            showToast('Error', `Failed to load events: ${err.message}`, 'error');
            if (!append && accordion.children.length === 0) {
                emptyState?.classList.remove('d-none');
                if (resultsBadge) resultsBadge.textContent = 'Error';
            }
        } finally {
            isFetching = false;
            if (!append) hideLoading();
        }
    }

    function renderCombinations(combinations) {
        if (!combosTbody) return;
        combosTbody.innerHTML = '';

        const frag = document.createDocumentFragment();
        combinations.forEach((combo) => {
            const tr = document.createElement('tr');
            const legsHtml = combo.legs
                .map((leg, legIndex) => {
                    const market = esc(leg.market_name || 'Market');
                    const outcome = esc(leg.outcome || 'Outcome');
                    const eventName = esc(leg.event_name || 'Event');
                    const eventUrl = leg.event_url ? esc(leg.event_url) : '';
                    const eventLabel = eventUrl
                        ? `<a href="${eventUrl}" target="_blank" rel="noopener noreferrer" class="event-link">${eventName} <i class="bi bi-box-arrow-up-right small"></i></a>`
                        : eventName;
                    return `<li class="d-flex align-items-center justify-content-between gap-2">
                        <span>
                            <span class="fw-semibold">${market}</span>: ${outcome}
                            <span class="text-muted">(${eventLabel})</span>
                            <span class="text-success">@ ${Number(leg.decimal || 0).toFixed(2)}</span>
                        </span>
                        <span class="d-flex gap-1">
                            <button type="button" class="btn btn-outline-secondary btn-sm combo-replace-leg" data-leg-index="${legIndex}">
                                Replace
                            </button>
                            <button type="button" class="btn btn-outline-danger btn-sm combo-remove-leg" data-leg-index="${legIndex}">
                                Remove
                            </button>
                        </span>
                    </li>`;
                })
                .join('');

            tr.innerHTML = `
                <td>
                    <ol class="mb-0 combo-legs-list">${legsHtml}</ol>
                </td>
                <td class="text-end fw-bold text-success">${Number(combo.combined_odds || 0).toFixed(3)}</td>
                <td class="text-end text-muted">${Number(combo.delta || 0).toFixed(3)}</td>
            `;
            frag.appendChild(tr);
        });
        combosTbody.appendChild(frag);
    }

    function recomputeComboMetrics(combo) {
        combo.combined_odds = combo.legs.reduce((acc, leg) => acc * Number(leg.decimal || 1), 1);
        combo.delta = Math.abs(combo.combined_odds - COMBO_TARGET_ODDS);
    }

    function randomSimilarReplacementPool(combo, legIndex) {
        const originalLeg = combo.legs[legIndex];
        if (!originalLeg) return [];

        const protectedEvents = new Set(
            combo.legs
                .filter((_, idx) => idx !== legIndex)
                .map((leg) => leg.event_id)
        );
        const originalKey = legKey(originalLeg);
        const allCandidates = comboCandidatePool.length > 0
            ? comboCandidatePool
            : collectVisibleLegCandidates();
        const originalOdds = Number(originalLeg.decimal || 1);

        const bands = [0.03, 0.06, 0.10];
        for (const band of bands) {
            const pool = allCandidates.filter((candidate) => {
                const odds = Number(candidate.decimal || 0);
                if (!Number.isFinite(odds) || odds <= 1.0) return false;
                if (protectedEvents.has(candidate.event_id)) return false;
                if (legKey(candidate) === originalKey) return false;
                return Math.abs(odds - originalOdds) <= band;
            });

            if (pool.length > 0) {
                return pool;
            }
        }

        return [];
    }

    async function replaceLegInCurrentCombination(legIndex) {
        try {
            await fetchAllLegCandidatesForCurrentFilters();
        } catch (err) {
            showToast('Error', `Failed to load matching events: ${err.message}`, 'error');
            return;
        }
        const current = currentComboPageMatches()[0];
        if (!current) {
            showToast('Builder is empty', 'Use Add Bet to start building.', 'warning');
            return;
        }

        const pool = randomSimilarReplacementPool(current, legIndex);
        if (pool.length === 0) {
            showToast('No similar replacement found', 'Try loading more events or adjust filters.', 'warning');
            return;
        }

        const replacement = pool[Math.floor(Math.random() * pool.length)];
        current.legs[legIndex] = replacement;
        recomputeComboMetrics(current);

        renderCurrentComboPage();
        updateCombinationUiState();
        showToast(
            'Leg replaced',
            `${replacement.market_name}: ${replacement.outcome} @ ${Number(replacement.decimal).toFixed(2)}`,
            'success',
            2500
        );
    }

    function removeLegFromCurrentCombination(legIndex) {
        const current = currentComboPageMatches()[0];
        if (!current) {
            showToast('Builder is empty', 'Use Add Bet to start building.', 'warning');
            return;
        }
        if (!Array.isArray(current.legs) || current.legs.length <= 1) {
            showToast('Cannot remove', 'A combination must contain at least 1 bet.', 'warning');
            return;
        }

        const removed = current.legs.splice(legIndex, 1)[0];
        if (!removed) return;
        recomputeComboMetrics(current);

        renderCurrentComboPage();
        updateCombinationUiState();
        showToast('Leg removed', `${removed.market_name}: ${removed.outcome}`, 'info', 2200);
    }

    function randomCandidateForAdd(combo) {
        const protectedEvents = new Set(combo.legs.map((leg) => leg.event_id));
        const sourcePool = comboCandidatePool.length > 0 ? comboCandidatePool : collectVisibleLegCandidates();
        const avgOdds = combo.legs.length > 0
            ? combo.legs.reduce((acc, leg) => acc + Number(leg.decimal || 0), 0) / combo.legs.length
            : COMBO_TARGET_ODDS;

        const bands = [0.04, 0.08, 0.15];
        for (const band of bands) {
            const pool = sourcePool.filter((candidate) => {
                const odds = Number(candidate.decimal || 0);
                if (!Number.isFinite(odds) || odds <= 1.0) return false;
                if (protectedEvents.has(candidate.event_id)) return false;
                return Math.abs(odds - avgOdds) <= band;
            });
            if (pool.length > 0) {
                return pool[Math.floor(Math.random() * pool.length)];
            }
        }

        const fallback = sourcePool.filter((candidate) => !protectedEvents.has(candidate.event_id));
        if (fallback.length === 0) return null;
        return fallback[Math.floor(Math.random() * fallback.length)];
    }

    async function addRandomLegToCurrentCombination() {
        try {
            await fetchAllLegCandidatesForCurrentFilters();
        } catch (err) {
            showToast('Error', `Failed to load matching events: ${err.message}`, 'error');
            return;
        }

        if (comboMatches.length === 0) {
            comboMatches = [{ legs: [], combined_odds: 1, delta: 0 }];
            comboPage = 0;
        }
        const current = currentComboPageMatches()[0];

        if (current.legs.length >= MAX_COMBO_BETS) {
            showToast('Max bets reached', `This combination already has ${MAX_COMBO_BETS} bets.`, 'warning');
            return;
        }

        const candidate = randomCandidateForAdd(current);
        if (!candidate) {
            showToast('No random bet available', 'No compatible event left to add.', 'warning');
            return;
        }

        current.legs.push(candidate);
        recomputeComboMetrics(current);

        renderCurrentComboPage();
        updateCombinationUiState();
        showToast(
            'Bet added',
            `${candidate.market_name}: ${candidate.outcome} @ ${Number(candidate.decimal).toFixed(2)}`,
            'success',
            2500
        );
    }

    function legCandidateFromMarketRow(row) {
        const decimal = Number.parseFloat(row.dataset.decimal || '0');
        if (!Number.isFinite(decimal) || decimal <= 1.0) {
            return null;
        }
        const eventNode = row.closest('.accordion-item[data-event-id]');
        if (!eventNode) {
            return null;
        }
        return {
            event_id: row.dataset.eventId || '',
            event_name: eventNode.querySelector('.event-link')?.textContent?.trim() || 'Event',
            event_url: eventNode.querySelector('.event-link')?.getAttribute('href') || '',
            market_name: row.dataset.marketName || 'Market',
            outcome: row.dataset.outcome || 'Outcome',
            decimal,
        };
    }

    function addSpecificLegToCurrentCombination(candidate) {
        if (!candidate) return;

        if (comboMatches.length === 0) {
            comboMatches = [{ legs: [], combined_odds: 1, delta: 0 }];
            comboPage = 0;
        }
        const current = currentComboPageMatches()[0];
        if (!current) return;

        if (current.legs.length >= MAX_COMBO_BETS) {
            showToast('Max bets reached', `This combination already has ${MAX_COMBO_BETS} bets.`, 'warning');
            return;
        }

        if (current.legs.some((leg) => leg.event_id === candidate.event_id)) {
            showToast('Event already included', 'Only one bet per event is allowed in the builder.', 'warning');
            return;
        }

        current.legs.push(candidate);
        recomputeComboMetrics(current);
        renderCurrentComboPage();
        updateCombinationUiState();
        showToast(
            'Bet added',
            `${candidate.market_name}: ${candidate.outcome} @ ${Number(candidate.decimal).toFixed(2)}`,
            'success',
            2500
        );
    }

    function currentComboPageMatches() {
        if (comboMatches.length === 0) {
            return [];
        }
        return [comboMatches[0]];
    }

    function renderCurrentComboPage() {
        renderCombinations(currentComboPageMatches());
    }

    function clearCombinationResults(message) {
        comboPage = -1;
        comboMatches = [];
        renderCurrentComboPage();
        if (combosInfo && message) {
            combosInfo.textContent = message;
        }
        updateCombinationUiState();
    }

    function updateCombinationUiState() {
        const hasBuilderCombo = comboMatches.length > 0;
        const comboSize = hasBuilderCombo ? (comboMatches[0].legs?.length || 0) : 0;
        if (combosBadge) {
            combosBadge.textContent = hasBuilderCombo ? `${comboSize} bet${comboSize === 1 ? '' : 's'}` : 'Empty';
        }
        if (combosInfo) {
            combosInfo.textContent = hasBuilderCombo
                ? `Builder active with ${comboSize} bet${comboSize === 1 ? '' : 's'}.`
                : 'Builder is empty. Use Add Bet to start.';
        }
        if (combosEmptyState) {
            combosEmptyState.classList.toggle('d-none', hasBuilderCombo);
        }
        if (combosTableWrap) {
            combosTableWrap.classList.remove('d-none');
        }
        if (comboClearBtn) comboClearBtn.disabled = comboIsSearching || !hasBuilderCombo;
        if (comboAddLegBtn) comboAddLegBtn.disabled = comboIsSearching;
    }

    function collectVisibleLegCandidates() {
        const rows = Array.from(
            accordion.querySelectorAll('.accordion-item[data-event-id] tr[data-event-id][data-decimal]')
        );

        return rows
            .map((row) => {
                const decimal = Number.parseFloat(row.dataset.decimal || '0');
                if (!Number.isFinite(decimal) || decimal <= 1.0) {
                    return null;
                }

                const eventNode = row.closest('.accordion-item[data-event-id]');
                if (!eventNode) {
                    return null;
                }

                const isLive = eventNode.dataset.isLive === 'true';
                if (advancedFilters.omit_live_events && isLive) {
                    return null;
                }

                return {
                    event_id: row.dataset.eventId || '',
                    event_name: eventNode.querySelector('.event-link')?.textContent?.trim() || 'Event',
                    event_url: eventNode.querySelector('.event-link')?.getAttribute('href') || '',
                    market_name: row.dataset.marketName || 'Market',
                    outcome: row.dataset.outcome || 'Outcome',
                    decimal,
                };
            })
            .filter(Boolean);
    }

    function candidatePoolSignature() {
        const formData  = new FormData(filterForm);
        return JSON.stringify({
            target: formData.get('target') || '1.1',
            tolerance: formData.get('tolerance') || '0.04',
            sport: formData.get('sport') || '',
            time_preset: advancedFilters.time_preset,
            from_time: advancedFilters.from_time,
            to_time: advancedFilters.to_time,
            live_only: advancedFilters.live_only,
            class_id: advancedFilters.class_id,
            omit_live_events: advancedFilters.omit_live_events,
        });
    }

    async function fetchAllLegCandidatesForCurrentFilters() {
        const signature = candidatePoolSignature();
        if (comboCandidatePool.length > 0 && comboCandidateSignature === signature) {
            return comboCandidatePool;
        }

        const pageSize = 200;
        const maxPages = 250;
        let page = 0;
        let hasMorePages = true;
        const all = [];

        while (hasMorePages && page < maxPages) {
            const url = buildApiUrl(page, pageSize);
            const resp = await fetch(url);
            if (!resp.ok) {
                throw new Error(`HTTP ${resp.status}`);
            }

            const data = await resp.json();
            (data.events || []).forEach((event) => {
                const isLive = !!event.is_live;
                if (advancedFilters.omit_live_events && isLive) {
                    return;
                }

                (event.markets || []).forEach((market) => {
                    const decimal = Number.parseFloat(String(market.decimal ?? 0));
                    if (!Number.isFinite(decimal) || decimal <= 1.0) {
                        return;
                    }

                    all.push({
                        event_id: event.event_id || '',
                        event_name: event.event_name || 'Event',
                        event_url: event.event_url || '',
                        market_name: market.market_name || 'Market',
                        outcome: market.outcome || 'Outcome',
                        decimal,
                    });
                });
            });

            hasMorePages = !!data.has_more;
            page += 1;
            if (combosInfo && hasMorePages) {
                combosInfo.textContent = `Loading all matching events for combinations... page ${page}`;
            }
        }

        comboCandidatePool = all;
        comboCandidateSignature = signature;
        return comboCandidatePool;
    }

    function oddsEntropy(legs) {
        if (!legs?.length) return 0;
        const buckets = new Map();
        legs.forEach((leg) => {
            const bucket = (Math.round((leg.decimal || 0) * 20) / 20).toFixed(2);
            buckets.set(bucket, (buckets.get(bucket) || 0) + 1);
        });

        const total = legs.length;
        let entropy = 0;
        buckets.forEach((count) => {
            const p = count / total;
            entropy -= p * Math.log2(p);
        });

        const maxEntropy = Math.log2(total);
        return maxEntropy > 0 ? entropy / maxEntropy : 0;
    }

    function legKey(leg) {
        return `${leg.event_id}|${leg.market_name}|${leg.outcome}`;
    }

    function jaccardSimilarity(legsA, legsB) {
        const setA = new Set(legsA.map(legKey));
        const setB = new Set(legsB.map(legKey));
        return jaccardSimilaritySets(setA, setB);
    }

    function jaccardSimilaritySets(setA, setB) {
        let intersection = 0;
        setA.forEach((key) => {
            if (setB.has(key)) intersection += 1;
        });
        const union = setA.size + setB.size - intersection;
        return union > 0 ? intersection / union : 0;
    }

    function filterSimilarCombinationsByEntropy(sortedMatches) {
        const accepted = [];
        const similarityThreshold = 0.85;

        sortedMatches.forEach((candidate) => {
            let duplicateIndex = -1;
            for (let i = 0; i < accepted.length; i += 1) {
                const existing = accepted[i];
                if (jaccardSimilarity(candidate.legs, existing.legs) >= similarityThreshold) {
                    duplicateIndex = i;
                    break;
                }
            }

            if (duplicateIndex === -1) {
                accepted.push(candidate);
                return;
            }

            if (candidate.entropy > accepted[duplicateIndex].entropy) {
                accepted[duplicateIndex] = candidate;
            }
        });

        return accepted;
    }

    // Re-rank candidates to explicitly balance quality vs novelty.
    // Higher minVariety increases novelty pressure and yields visibly different combos.
    function diversifyMatches(sortedMatches, minVariety, prioritizeFewerBets) {
        if (sortedMatches.length <= 1) {
            return sortedMatches.map((m) => ({ ...m, variety: 1 }));
        }

        const poolLimit = 600;
        const pool = sortedMatches
            .slice(0, Math.min(poolLimit, sortedMatches.length))
            .map((match) => ({ ...match, legSet: new Set(match.legs.map(legKey)) }));
        const chosen = [];
        const used = new Set();
        const legUseCount = new Map();
        const maxKeep = 300;

        const safeDenominator = Math.max(1, pool.length - 1);
        const baseVarietyWeight = Math.min(0.95, 0.40 + (minVariety * 0.60) + (prioritizeFewerBets ? 0.06 : 0));
        const reusePenaltyWeight = 0.10 + (minVariety * 0.35);

        function averageLegReuse(candidateLegSet) {
            if (chosen.length === 0 || candidateLegSet.size === 0) return 0;
            let total = 0;
            candidateLegSet.forEach((k) => {
                total += (legUseCount.get(k) || 0) / chosen.length;
            });
            return total / candidateLegSet.size;
        }

        function pickNext(relaxFactor = 1) {
            const requiredVariety = Math.max(0, Math.min(1, minVariety * relaxFactor));
            let bestIdx = -1;
            let bestScore = Number.NEGATIVE_INFINITY;
            let bestVariety = 1;

            for (let i = 0; i < pool.length; i += 1) {
                if (used.has(i)) continue;

                const candidate = pool[i];
                const maxSimilarity = chosen.length === 0
                    ? 0
                    : chosen.reduce(
                        (acc, existing) => Math.max(acc, jaccardSimilaritySets(candidate.legSet, existing.legSet)),
                        0
                    );

                const variety = 1 - maxSimilarity;
                if (chosen.length > 0 && variety < requiredVariety) {
                    continue;
                }

                const quality = 1 - (i / safeDenominator);
                const reusePenalty = averageLegReuse(candidate.legSet);
                const score = (quality * (1 - baseVarietyWeight))
                    + (variety * baseVarietyWeight)
                    - (reusePenalty * reusePenaltyWeight);

                if (score > bestScore) {
                    bestScore = score;
                    bestIdx = i;
                    bestVariety = variety;
                }
            }

            return { bestIdx, bestVariety };
        }

        while (chosen.length < pool.length && chosen.length < maxKeep) {
            // First pass honors requested variety. If no candidate survives, relax progressively.
            let { bestIdx, bestVariety } = pickNext(1);
            if (bestIdx === -1) ({ bestIdx, bestVariety } = pickNext(0.85));
            if (bestIdx === -1) ({ bestIdx, bestVariety } = pickNext(0.65));
            if (bestIdx === -1) ({ bestIdx, bestVariety } = pickNext(0));

            if (bestIdx === -1) break;

            used.add(bestIdx);
            const picked = { ...pool[bestIdx], variety: bestVariety };
            picked.legSet.forEach((k) => {
                legUseCount.set(k, (legUseCount.get(k) || 0) + 1);
            });
            chosen.push(picked);
        }

        return chosen.map(({ legSet, ...rest }) => rest);
    }

    // ========== Reload from page 0 ==========
    function reloadEvents() {
        updateApiLink();
        fetchEvents(0, false);
        comboCandidatePool = [];
        comboCandidateSignature = '';
    }

    // ========== Form submit (Find Bets) ==========
    filterForm?.addEventListener('submit', function (e) {
        e.preventDefault();
        advancedFilters.class_id = '';
        if (leagueFilter) leagueFilter.value = '';
        reloadEvents();
    });

    // ========== Infinite scroll via IntersectionObserver ==========
    if (loadMoreSentinel && 'IntersectionObserver' in window) {
        const observer = new IntersectionObserver((entries) => {
            if (entries[0].isIntersecting && hasMore && !isFetching) {
                fetchEvents(currentPage + 1, true);
            }
        }, { rootMargin: '200px' });
        observer.observe(loadMoreSentinel);
    }

    // ========== Advanced filter changes ==========
    function syncAdvancedFilters() {
        advancedFilters.time_preset = timePresetSelect?.value ?? 'all';
        advancedFilters.from_time   = fromTimeInput?.value  ?? '';
        advancedFilters.to_time     = toTimeInput?.value    ?? '';
        advancedFilters.live_only   = document.querySelector('input[name="live_status"]:checked')?.value === 'live';
        advancedFilters.class_id    = leagueFilter?.value   ?? '';
        advancedFilters.omit_live_events = !!omitLiveCombosInput?.checked;
        if (omitLiveCombosInput) omitLiveCombosInput.checked = !!advancedFilters.omit_live_events;
    }

    function onAdvancedFilterChange() {
        syncAdvancedFilters();
        toggleCustomTimeRange();
        updateActiveFilterBadge();
        saveFilterState();
        reloadEvents();
    }

    timePresetSelect?.addEventListener('change', onAdvancedFilterChange);
    fromTimeInput?.addEventListener('change',    onAdvancedFilterChange);
    toTimeInput?.addEventListener('change',      onAdvancedFilterChange);
    leagueFilter?.addEventListener('change',     onAdvancedFilterChange);
    omitLiveCombosInput?.addEventListener('change', () => { syncAdvancedFilters(); saveFilterState(); });
    liveStatusRadios.forEach(r => r.addEventListener('change', onAdvancedFilterChange));

    clearFiltersBtn?.addEventListener('click', function () {
        if (leagueFilter)      leagueFilter.value  = '';
        if (timePresetSelect)  timePresetSelect.value = 'all';
        if (fromTimeInput)     fromTimeInput.value  = '';
        if (toTimeInput)       toTimeInput.value    = '';
        if (omitLiveCombosInput) omitLiveCombosInput.checked = false;
        const allRadio = document.getElementById('status-all');
        if (allRadio) allRadio.checked = true;
        advancedFilters = {
            time_preset: 'all',
            from_time: '',
            to_time: '',
            live_only: false,
            class_id: '',
            omit_live_events: false,
        };
        toggleCustomTimeRange();
        updateActiveFilterBadge();
        try { localStorage.removeItem('henzeFilters'); } catch (_) {}
        showToast('Filters cleared', 'Showing all events', 'info', 2000);
        reloadEvents();
    });

    // ========== Advanced filters collapse toggle icon ==========
    advFilterCollapse?.addEventListener('shown.bs.collapse', () => {
        advFilterToggle?.querySelector('.toggle-icon')?.classList.add('rotated');
    });
    advFilterCollapse?.addEventListener('hidden.bs.collapse', () => {
        advFilterToggle?.querySelector('.toggle-icon')?.classList.remove('rotated');
    });

    // ========== Custom time range toggle ==========
    function toggleCustomTimeRange() {
        if (customTimeRange) {
            customTimeRange.style.display = (timePresetSelect?.value === 'custom') ? 'block' : 'none';
        }
    }

    // ========== Active filter badge ==========
    function updateActiveFilterBadge() {
        if (!activeFilterBadge) return;
        let count = 0;
        if (advancedFilters.time_preset !== 'all') count++;
        if (advancedFilters.class_id)              count++;
        if (advancedFilters.live_only)             count++;
        activeFilterBadge.textContent    = count;
        activeFilterBadge.style.display  = count > 0 ? 'inline' : 'none';
    }

    // ========== Persist / restore filter state ==========
    function saveFilterState() {
        try { localStorage.setItem('henzeFilters', JSON.stringify(advancedFilters)); } catch (_) {}
    }

    function loadFilterState() {
        try {
            const raw = localStorage.getItem('henzeFilters');
            if (!raw) return;
            const state = JSON.parse(raw);
            advancedFilters = { ...advancedFilters, ...state };
            if (timePresetSelect)  timePresetSelect.value = advancedFilters.time_preset || 'all';
            if (fromTimeInput)     fromTimeInput.value    = advancedFilters.from_time   || '';
            if (toTimeInput)       toTimeInput.value      = advancedFilters.to_time     || '';
            if (leagueFilter)      leagueFilter.value     = advancedFilters.class_id    || '';
            if (omitLiveCombosInput) omitLiveCombosInput.checked = !!advancedFilters.omit_live_events;
            const liveVal = advancedFilters.live_only ? 'live' : 'all';
            const radio   = document.getElementById(`status-${liveVal}`);
            if (radio) radio.checked = true;
        } catch (_) {}
    }

    // ========== Event delegation for accordion (applied once) ==========
    let accordionListenersAttached = false;
    function ensureAccordionListeners() {
        if (accordionListenersAttached || !accordion) return;
        accordionListenersAttached = true;

        accordion.addEventListener('shown.bs.collapse', (e) => {
            const id = e.target.id;
            if (id) initMarketTables([id]);
        });
        accordion.addEventListener('click', (e) => {
            if (e.target.closest('.event-link')) e.stopPropagation();

            const addBtn = e.target.closest('.add-to-combo-btn');
            if (addBtn) {
                e.stopPropagation();
                const row = addBtn.closest('tr[data-event-id][data-decimal]');
                if (!row) return;
                const candidate = legCandidateFromMarketRow(row);
                if (!candidate) {
                    showToast('Cannot add bet', 'Selected row has invalid odds data.', 'warning');
                    return;
                }
                addSpecificLegToCurrentCombination(candidate);
            }
        });
        accordion.addEventListener('keypress', (e) => {
            if (e.target.matches('.accordion-button') && (e.key === 'Enter' || e.key === ' ')) {
                e.preventDefault();
                e.target.click();
            }
        });
    }

    // ========== Market Table Sorting & Grouping ==========
    const TABLE_PREFS_KEY = 'henzeTablePrefs';

    function normalizeMarketType(value) {
        const v = (value || '').trim();
        return (v === '' || v === '-' || v === '--') ? null : v;
    }

    function groupLabel(type, subType) {
        const t = normalizeMarketType(type);
        const s = normalizeMarketType(subType);
        if (!t) return 'Other';
        if (!s || s === t) return t;
        return `${t} \u2013 ${s}`;
    }

    function loadTablePrefs() {
        try { return JSON.parse(localStorage.getItem(TABLE_PREFS_KEY)) ?? {}; } catch (_) { return {}; }
    }

    function saveTablePrefs(prefs) {
        try { localStorage.setItem(TABLE_PREFS_KEY, JSON.stringify(prefs)); } catch (_) {}
    }

    const tablePrefs       = loadTablePrefs();
    function defaultPrefs() { return { sortCol: null, sortDir: 'asc', grouped: true }; }
    const initializedTables = new WeakSet();

    function renderTable(table, prefs) {
        const tbody = table.querySelector('tbody');
        if (!tbody) return;
        const dataRows = Array.from(tbody.querySelectorAll('tr[data-decimal]'));
        if (dataRows.length === 0) return;

        const { sortCol, sortDir, grouped } = prefs;
        const sorted = [...dataRows].sort((a, b) => {
            if (!sortCol) return 0;
            if (sortCol === 'odds') {
                const va = parseFloat(a.dataset.decimal) || 0;
                const vb = parseFloat(b.dataset.decimal) || 0;
                return sortDir === 'asc' ? va - vb : vb - va;
            }
            const va = (sortCol === 'market' ? a.dataset.marketName : a.dataset.outcome || '').toLowerCase();
            const vb = (sortCol === 'market' ? b.dataset.marketName : b.dataset.outcome || '').toLowerCase();
            const cmp = va < vb ? -1 : va > vb ? 1 : 0;
            return sortDir === 'asc' ? cmp : -cmp;
        });

        tbody.innerHTML = '';

        if (!grouped) {
            sorted.forEach(r => tbody.appendChild(r));
        } else {
            const groups = new Map();
            sorted.forEach(row => {
                const lbl = groupLabel(row.dataset.marketType, row.dataset.marketSubType);
                if (!groups.has(lbl)) groups.set(lbl, []);
                groups.get(lbl).push(row);
            });
            const labels = [...groups.keys()].sort((a, b) => {
                if (a === 'Other') return 1;
                if (b === 'Other') return -1;
                return a.localeCompare(b);
            });
            labels.forEach(lbl => {
                const hdr = document.createElement('tr');
                hdr.className = 'market-group-header';
                hdr.innerHTML = `<td colspan="4"><span class="market-group-label">${lbl}</span></td>`;
                tbody.appendChild(hdr);
                groups.get(lbl).forEach(r => tbody.appendChild(r));
            });
        }

        table.querySelectorAll('.sort-header').forEach(th => {
            const col  = th.dataset.sortCol;
            const icon = th.querySelector('.sort-icon');
            if (col === sortCol) {
                th.setAttribute('aria-sort', sortDir === 'asc' ? 'ascending' : 'descending');
                th.classList.add('sort-active');
                if (icon) icon.className = `bi ${sortDir === 'asc' ? 'bi-arrow-up' : 'bi-arrow-down'} sort-icon ms-1`;
            } else {
                th.removeAttribute('aria-sort');
                th.classList.remove('sort-active');
                if (icon) icon.className = 'bi bi-arrow-down-up sort-icon ms-1';
            }
        });

        const toggleBtn = table.closest('.accordion-body')?.querySelector('.market-group-toggle');
        if (toggleBtn) {
            toggleBtn.setAttribute('aria-pressed', grouped ? 'true' : 'false');
            toggleBtn.classList.toggle('active', grouped);
        }
    }

    // collapseIds: optional array of collapse-element IDs to scope init to new items only
    function initMarketTables(collapseIds) {
        let tables;
        if (collapseIds?.length) {
            tables = collapseIds.flatMap(id => {
                const el = document.getElementById(id);
                return el ? Array.from(el.querySelectorAll('.market-table')) : [];
            });
        } else {
            tables = Array.from(document.querySelectorAll('.market-table'));
        }

        tables.forEach(table => {
            const eventId = table.querySelector('tbody tr[data-event-id]')?.dataset.eventId;
            if (!eventId) return;
            const key = `event-${eventId}`;
            if (!tablePrefs[key]) tablePrefs[key] = defaultPrefs();

            if (!initializedTables.has(table)) {
                initializedTables.add(table);

                table.querySelectorAll('.sort-header').forEach(th => {
                    const handler = (e) => {
                        e.stopPropagation();
                        const col   = th.dataset.sortCol;
                        const prefs = tablePrefs[key];
                        if (prefs.sortCol === col) {
                            prefs.sortDir = prefs.sortDir === 'asc' ? 'desc' : 'asc';
                        } else {
                            prefs.sortCol = col;
                            prefs.sortDir = 'asc';
                        }
                        renderTable(table, prefs);
                        saveTablePrefs(tablePrefs);
                    };
                    th.addEventListener('click', handler);
                    th.addEventListener('keydown', (e) => {
                        if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handler(e); }
                    });
                });

                const toggleBtn = table.closest('.accordion-body')?.querySelector('.market-group-toggle');
                if (toggleBtn) {
                    toggleBtn.addEventListener('click', (e) => {
                        e.stopPropagation();
                        const prefs   = tablePrefs[key];
                        prefs.grouped = !prefs.grouped;
                        renderTable(table, prefs);
                        saveTablePrefs(tablePrefs);
                    });
                }
            }

            renderTable(table, tablePrefs[key]);
        });
    }

    // ========== Feeling Lucky ==========
    const luckyBtn = document.getElementById('feeling-lucky');
    if (luckyBtn) {
        luckyBtn.addEventListener('click', function () {
            document.querySelectorAll('tr.ds-lucky-row').forEach(r => {
                r.classList.remove('ds-lucky-row', 'ds-lucky-highlight');
                r.querySelector('.ds-lucky-ribbon-inline')?.remove();
            });

            const visibleRows = Array.from(
                accordion.querySelectorAll('.accordion-item[data-event-id] tr[data-event-id][data-decimal]')
            );
            if (visibleRows.length === 0) {
                showToast('No visible bets', 'Load at least one event first', 'warning');
                return;
            }

            const matchedRow  = visibleRows[Math.floor(Math.random() * visibleRows.length)];
            const eventId     = matchedRow.dataset.eventId;
            const marketName  = matchedRow.dataset.marketName || 'Market';
            const outcome     = matchedRow.dataset.outcome    || 'Outcome';
            const decimal     = parseFloat(matchedRow.dataset.decimal || '0');

            const placeSticker = () => {
                matchedRow.scrollIntoView({ behavior: 'smooth', block: 'center' });
                matchedRow.classList.add('ds-lucky-row', 'ds-lucky-highlight');
                const sticker = document.createElement('span');
                sticker.className   = 'ds-lucky-ribbon-inline';
                sticker.textContent = 'Lucky Bet';
                matchedRow.querySelector('td.odds-cell')?.prepend(sticker);
                showToast('Lucky pick!', `${marketName}: ${outcome} @ ${decimal.toFixed(2)}`, 'success');
            };

            const collapseEl = document.querySelector(`#event-${eventId}`);
            if (collapseEl) {
                const bsc = bootstrap.Collapse.getOrCreateInstance(collapseEl);
                if (!collapseEl.classList.contains('show')) {
                    collapseEl.addEventListener('shown.bs.collapse', placeSticker, { once: true });
                    bsc.show();
                } else {
                    placeSticker();
                }
            } else {
                placeSticker();
            }
        });
    }

    // ========== Initialise ==========
    toggleCustomTimeRange();
    loadFilterState();
    syncAdvancedFilters();
    updateActiveFilterBadge();
    updateApiLink();
    ensureAccordionListeners();

    comboClearBtn?.addEventListener('click', () => {
        clearCombinationResults('Builder is empty. Use Add Bet to start.');
    });
    comboAddLegBtn?.addEventListener('click', addRandomLegToCurrentCombination);
    combosTbody?.addEventListener('click', (e) => {
        const replaceBtn = e.target.closest('.combo-replace-leg');
        const removeBtn = e.target.closest('.combo-remove-leg');
        const button = replaceBtn || removeBtn;
        if (!button) return;
        const legIndex = Number.parseInt(button.dataset.legIndex || '', 10);
        if (Number.isNaN(legIndex)) return;
        if (replaceBtn) {
            replaceLegInCurrentCombination(legIndex);
        } else {
            removeLegFromCurrentCombination(legIndex);
        }
    });

    // Kick off initial data fetch
    fetchEvents(0, false);
    clearCombinationResults('Builder is empty. Use Add Bet to start.');
});
