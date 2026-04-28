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

    // ========== Pagination state ==========
    const PAGE_SIZE = 40;
    let currentPage = 0;
    let totalEvents = 0;
    let hasMore     = false;
    let isFetching  = false;

    // ========== Current advanced filter state ==========
    let advancedFilters = {
        time_preset: 'all',
        from_time:   '',
        to_time:     '',
        live_only:   false,
        class_id:    ''
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
    function buildApiUrl(page) {
        const formData  = new FormData(filterForm);
        const target    = formData.get('target')    || '1.1';
        const tolerance = formData.get('tolerance') || '0.04';
        const sport     = formData.get('sport')     || '';

        const params = new URLSearchParams({ target, tolerance, page: String(page), page_size: String(PAGE_SIZE) });
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
                <td class="text-end fw-bold text-success">${m.decimal.toFixed(2)}</td>
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

    // ========== Reload from page 0 ==========
    function reloadEvents() {
        updateApiLink();
        fetchEvents(0, false);
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
    liveStatusRadios.forEach(r => r.addEventListener('change', onAdvancedFilterChange));

    clearFiltersBtn?.addEventListener('click', function () {
        if (leagueFilter)      leagueFilter.value  = '';
        if (timePresetSelect)  timePresetSelect.value = 'all';
        if (fromTimeInput)     fromTimeInput.value  = '';
        if (toTimeInput)       toTimeInput.value    = '';
        const allRadio = document.getElementById('status-all');
        if (allRadio) allRadio.checked = true;
        advancedFilters = { time_preset: 'all', from_time: '', to_time: '', live_only: false, class_id: '' };
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
                hdr.innerHTML = `<td colspan="3"><span class="market-group-label">${lbl}</span></td>`;
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
                matchedRow.querySelector('td:last-child')?.prepend(sticker);
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
    updateActiveFilterBadge();
    updateApiLink();
    ensureAccordionListeners();

    // Kick off initial data fetch
    fetchEvents(0, false);
});
