// Henze DS Web - Event handling
document.addEventListener('DOMContentLoaded', function() {
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
            <button class="toast-close" aria-label="Close">
                <i class="bi bi-x"></i>
            </button>
            <div class="toast-progress" style="animation-duration: ${duration}ms"></div>
        `;
        
        toastContainer.appendChild(toast);
        
        // Trigger animation
        requestAnimationFrame(() => {
            toast.classList.add('show');
        });
        
        // Close button handler
        const closeBtn = toast.querySelector('.toast-close');
        closeBtn.addEventListener('click', () => dismissToast(toast));
        
        // Auto dismiss
        setTimeout(() => dismissToast(toast), duration);
        
        return toast;
    }
    
    function dismissToast(toast) {
        if (!toast || toast.classList.contains('hiding')) return;
        toast.classList.remove('show');
        toast.classList.add('hiding');
        setTimeout(() => toast.remove(), 400);
    }
    
    // Show results toast on page load (if we have results)
    const eventCount = document.querySelectorAll('.accordion-item').length;
    const marketCount = document.querySelectorAll('tbody tr[data-event-id]').length;
    const liveCount = document.querySelectorAll('.live-event').length;
    
    if (eventCount > 0) {
        let message = `${marketCount} market${marketCount !== 1 ? 's' : ''} across ${eventCount} event${eventCount !== 1 ? 's' : ''}`;
        if (liveCount > 0) {
            message += ` (${liveCount} live)`;
        }
        showToast('Results loaded', message, 'success');
    }

    // Loading overlay helper functions
    const loadingOverlay = document.getElementById('loading-overlay');
    
    function showLoading() {
        if (loadingOverlay) {
            loadingOverlay.classList.add('active');
        }
    }
    
    function hideLoading() {
        if (loadingOverlay) {
            loadingOverlay.classList.remove('active');
        }
    }

    // Show loading animation when form is submitted (Find Bets button)
    const searchForm = document.getElementById('filter-form');
    if (searchForm) {
        searchForm.addEventListener('submit', function() {
            showLoading();
        });
    }

    // ========== Advanced Filters System (100% Client-Side) ==========
    
    // Elements
    const advancedFiltersToggle = document.getElementById('advanced-filters-toggle');
    const advancedFiltersCollapse = document.getElementById('advancedFilters');
    const activeFilterCountBadge = document.getElementById('active-filter-count');
    const leagueFilter = document.getElementById('league-filter');
    const liveStatusRadios = document.querySelectorAll('input[name="live_status"]');
    const timePresetSelect = document.getElementById('time-preset-select');
    const customTimeRange = document.getElementById('custom-time-range');
    const fromTimeInput = document.getElementById('from-time-input');
    const toTimeInput = document.getElementById('to-time-input');
    const clearFiltersBtn = document.getElementById('clear-filters');
    
    // Get all event accordion items
    const allEventItems = document.querySelectorAll('.accordion-item[data-event-id]');
    
    // Count visible events and markets
    function countVisibleResults() {
        let visibleEvents = 0;
        let visibleMarkets = 0;
        let visibleLive = 0;
        
        allEventItems.forEach(item => {
            if (!item.classList.contains('filter-hidden')) {
                visibleEvents++;
                const marketRows = item.querySelectorAll('tbody tr[data-event-id]');
                visibleMarkets += marketRows.length;
                if (item.dataset.isLive === 'true') {
                    visibleLive++;
                }
            }
        });
        
        return { events: visibleEvents, markets: visibleMarkets, live: visibleLive };
    }
    
    // Update results count badge
    function updateResultsCount() {
        const counts = countVisibleResults();
        const badge = document.querySelector('.card-header .badge.bg-secondary');
        if (badge) {
            badge.textContent = `${counts.events} event${counts.events !== 1 ? 's' : ''} (${counts.markets} market${counts.markets !== 1 ? 's' : ''})`;
        }
        // Also update the info text in the league filter section
        const infoText = document.querySelector('.card-filter .form-text');
        if (infoText) {
            infoText.innerHTML = `<i class="bi bi-info-circle me-1"></i> Showing ${counts.events} events`;
        }
    }
    
    // Count active filters
    function countActiveFilters() {
        let count = 0;
        
        // Time filter
        const timePreset = timePresetSelect?.value || 'all';
        if (timePreset !== 'all') count++;
        
        // League filter
        if (leagueFilter && leagueFilter.value) count++;
        
        // Live status filter
        const liveStatus = document.querySelector('input[name="live_status"]:checked')?.value || 'all';
        if (liveStatus !== 'all') count++;
        
        return count;
    }
    
    // Update active filter count badge
    function updateFilterCountBadge() {
        if (!activeFilterCountBadge) return;
        
        const count = countActiveFilters();
        if (count > 0) {
            activeFilterCountBadge.textContent = count;
            activeFilterCountBadge.style.display = 'inline';
        } else {
            activeFilterCountBadge.style.display = 'none';
        }
    }
    
    // Get time range based on preset
    function getTimeRange(preset) {
        const now = new Date();
        const todayStart = new Date(now.getFullYear(), now.getMonth(), now.getDate());
        const tomorrowStart = new Date(todayStart.getTime() + 24 * 60 * 60 * 1000);
        
        switch (preset) {
            case 'next2h':
                return { from: now, to: new Date(now.getTime() + 2 * 60 * 60 * 1000) };
            case 'today':
                return { from: todayStart, to: tomorrowStart };
            case 'tomorrow':
                return { from: tomorrowStart, to: new Date(tomorrowStart.getTime() + 24 * 60 * 60 * 1000) };
            case 'week':
                return { from: now, to: new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000) };
            case 'custom':
                const fromVal = fromTimeInput?.value;
                const toVal = toTimeInput?.value;
                return {
                    from: fromVal ? new Date(fromVal) : null,
                    to: toVal ? new Date(toVal) : null
                };
            default: // 'all'
                return { from: null, to: null };
        }
    }
    
    // Apply all client-side filters
    function applyAllFilters() {
        const selectedLeague = leagueFilter?.value || '';
        const selectedLiveStatus = document.querySelector('input[name="live_status"]:checked')?.value || 'all';
        const timePreset = timePresetSelect?.value || 'all';
        const timeRange = getTimeRange(timePreset);
        
        let visibleCount = 0;
        let hiddenCount = 0;
        
        allEventItems.forEach(item => {
            let shouldShow = true;
            
            // League filter (using class_id)
            if (selectedLeague && item.dataset.classId !== selectedLeague) {
                shouldShow = false;
            }
            
            // Live status filter
            const isLive = item.dataset.isLive === 'true';
            if (selectedLiveStatus === 'live' && !isLive) {
                shouldShow = false;
            } else if (selectedLiveStatus === 'upcoming' && isLive) {
                shouldShow = false;
            }
            
            // Time range filter
            if (shouldShow && (timeRange.from || timeRange.to)) {
                const eventTimeStr = item.dataset.eventTimeUtc;
                if (eventTimeStr) {
                    const eventTime = new Date(eventTimeStr);
                    if (timeRange.from && eventTime < timeRange.from) {
                        shouldShow = false;
                    }
                    if (timeRange.to && eventTime > timeRange.to) {
                        shouldShow = false;
                    }
                }
            }
            
            // Apply visibility
            if (shouldShow) {
                item.classList.remove('filter-hidden');
                visibleCount++;
            } else {
                item.classList.add('filter-hidden');
                hiddenCount++;
            }
        });
        
        // Update counts
        updateResultsCount();
        updateFilterCountBadge();
        
        // Show toast only when filters are actively changed (not on load)
        if (document.activeElement && hiddenCount > 0 && visibleCount > 0) {
            showToast('Filters applied', `Showing ${visibleCount} event${visibleCount !== 1 ? 's' : ''}`, 'info', 2000);
        } else if (document.activeElement && visibleCount === 0 && hiddenCount > 0) {
            showToast('No matches', 'No events match the selected filters', 'warning');
        }
        
        // Save filter state to localStorage
        saveFilterState();
    }
    
    // Save filter state to localStorage
    function saveFilterState() {
        const state = {
            leagueId: leagueFilter?.value || '',
            liveStatus: document.querySelector('input[name="live_status"]:checked')?.value || 'all',
            timePreset: timePresetSelect?.value || 'all',
            fromTime: fromTimeInput?.value || '',
            toTime: toTimeInput?.value || ''
        };
        try {
            localStorage.setItem('henzeFilters', JSON.stringify(state));
        } catch (e) {
            console.warn('Could not save filter state', e);
        }
    }
    
    // Load filter state from localStorage
    function loadFilterState() {
        try {
            const saved = localStorage.getItem('henzeFilters');
            if (saved) {
                const state = JSON.parse(saved);
                
                // Restore league filter
                if (leagueFilter && state.leagueId) {
                    const option = leagueFilter.querySelector(`option[value="${state.leagueId}"]`);
                    if (option) leagueFilter.value = state.leagueId;
                }
                
                // Restore live status
                if (state.liveStatus) {
                    const radio = document.getElementById(`status-${state.liveStatus}`);
                    if (radio) radio.checked = true;
                }
                
                // Restore time preset
                if (state.timePreset && timePresetSelect) {
                    timePresetSelect.value = state.timePreset;
                }
                
                // Restore custom time values
                if (fromTimeInput && state.fromTime) fromTimeInput.value = state.fromTime;
                if (toTimeInput && state.toTime) toTimeInput.value = state.toTime;
                
                // Apply the restored filters (without showing toast)
                toggleCustomTimeRange();
                
                // Apply filters silently (don't trigger change events)
                const selectedLeague = leagueFilter?.value || '';
                const selectedLiveStatus = document.querySelector('input[name="live_status"]:checked')?.value || 'all';
                const timePreset = timePresetSelect?.value || 'all';
                const timeRange = getTimeRange(timePreset);
                
                allEventItems.forEach(item => {
                    let shouldShow = true;
                    
                    if (selectedLeague && item.dataset.classId !== selectedLeague) {
                        shouldShow = false;
                    }
                    
                    const isLive = item.dataset.isLive === 'true';
                    if (selectedLiveStatus === 'live' && !isLive) {
                        shouldShow = false;
                    } else if (selectedLiveStatus === 'upcoming' && isLive) {
                        shouldShow = false;
                    }
                    
                    if (shouldShow && (timeRange.from || timeRange.to)) {
                        const eventTimeStr = item.dataset.eventTimeUtc;
                        if (eventTimeStr) {
                            const eventTime = new Date(eventTimeStr);
                            if (timeRange.from && eventTime < timeRange.from) shouldShow = false;
                            if (timeRange.to && eventTime > timeRange.to) shouldShow = false;
                        }
                    }
                    
                    if (shouldShow) {
                        item.classList.remove('filter-hidden');
                    } else {
                        item.classList.add('filter-hidden');
                    }
                });
                
                updateResultsCount();
                updateFilterCountBadge();
            }
        } catch (e) {
            console.warn('Could not load filter state', e);
        }
    }
    
    // Toggle custom time range visibility
    function toggleCustomTimeRange() {
        const selectedPreset = timePresetSelect?.value || 'all';
        if (customTimeRange) {
            customTimeRange.style.display = selectedPreset === 'custom' ? 'block' : 'none';
        }
    }
    
    // Event listener for time preset dropdown
    if (timePresetSelect) {
        timePresetSelect.addEventListener('change', function() {
            toggleCustomTimeRange();
            applyAllFilters();
        });
    }
    
    // Custom time inputs
    if (fromTimeInput) {
        fromTimeInput.addEventListener('change', applyAllFilters);
    }
    if (toTimeInput) {
        toTimeInput.addEventListener('change', applyAllFilters);
    }
    
    // League filter
    if (leagueFilter) {
        leagueFilter.addEventListener('change', applyAllFilters);
    }
    
    // Live status radios
    liveStatusRadios.forEach(radio => {
        radio.addEventListener('change', applyAllFilters);
    });
    
    // Clear filters button
    if (clearFiltersBtn) {
        clearFiltersBtn.addEventListener('click', function() {
            // Reset all filters
            if (leagueFilter) leagueFilter.value = '';
            
            const statusAllRadio = document.getElementById('status-all');
            if (statusAllRadio) statusAllRadio.checked = true;
            
            if (timePresetSelect) timePresetSelect.value = 'all';
            
            if (fromTimeInput) fromTimeInput.value = '';
            if (toTimeInput) toTimeInput.value = '';
            
            toggleCustomTimeRange();
            
            // Clear localStorage
            try {
                localStorage.removeItem('henzeFilters');
            } catch (e) {}
            
            // Show all events
            allEventItems.forEach(item => {
                item.classList.remove('filter-hidden');
            });
            
            updateResultsCount();
            updateFilterCountBadge();
            
            showToast('Filters cleared', 'Showing all events', 'info', 2000);
        });
    }
    
    // Toggle button icon rotation when advanced filters collapse/expand
    if (advancedFiltersCollapse) {
        advancedFiltersCollapse.addEventListener('shown.bs.collapse', function() {
            const icon = advancedFiltersToggle?.querySelector('.toggle-icon');
            if (icon) icon.classList.add('rotated');
        });
        advancedFiltersCollapse.addEventListener('hidden.bs.collapse', function() {
            const icon = advancedFiltersToggle?.querySelector('.toggle-icon');
            if (icon) icon.classList.remove('rotated');
        });
    }
    
    // Initialize
    toggleCustomTimeRange();
    updateFilterCountBadge();
    loadFilterState();

    // Auto-expand live events on page load
    const liveEvents = document.querySelectorAll('.live-event:not(.filter-hidden) .accordion-button');
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

    // ========== Market Table Sorting & Grouping ==========

    const TABLE_PREFS_KEY = 'henzeTablePrefs';

    // Normalize market type/subType values — treat "-", "--", or empty as null (no group)
    function normalizeMarketType(value) {
        const v = (value || '').trim();
        return (v === '' || v === '-' || v === '--') ? null : v;
    }

    // Build a group label from type + subType
    function groupLabel(type, subType) {
        const t = normalizeMarketType(type);
        const s = normalizeMarketType(subType);
        if (!t) return 'Other';
        if (!s || s === t) return t;
        return `${t} – ${s}`;
    }

    // Load persisted table prefs
    function loadTablePrefs() {
        try {
            const raw = localStorage.getItem(TABLE_PREFS_KEY);
            return raw ? JSON.parse(raw) : {};
        } catch (e) {
            return {};
        }
    }

    // Save table prefs
    function saveTablePrefs(prefs) {
        try {
            localStorage.setItem(TABLE_PREFS_KEY, JSON.stringify(prefs));
        } catch (e) {}
    }

    // Per-table sort/group state: { sortCol: 'market'|'outcome'|'odds'|null, sortDir: 'asc'|'desc', grouped: bool }
    const tablePrefs = loadTablePrefs();
    // Default state when key is absent
    function defaultPrefs() { return { sortCol: null, sortDir: 'asc', grouped: true }; }

    // Guard against registering duplicate listeners when initMarketTables is called multiple times
    const initializedTables = new WeakSet();

    // Apply sort and optional grouping to a single market table
    function renderTable(table, prefs) {
        const tbody = table.querySelector('tbody');
        if (!tbody) return;

        // Collect raw data rows (exclude any group header rows we may have injected)
        const dataRows = Array.from(tbody.querySelectorAll('tr[data-decimal]'));
        if (dataRows.length === 0) return;

        // Sort the rows
        const { sortCol, sortDir, grouped } = prefs;

        const sorted = [...dataRows].sort((a, b) => {
            if (!sortCol) return 0; // preserve original order when no sort active
            let va, vb;
            if (sortCol === 'market') {
                va = (a.dataset.marketName || '').toLowerCase();
                vb = (b.dataset.marketName || '').toLowerCase();
            } else if (sortCol === 'outcome') {
                va = (a.dataset.outcome || '').toLowerCase();
                vb = (b.dataset.outcome || '').toLowerCase();
            } else if (sortCol === 'odds') {
                va = parseFloat(a.dataset.decimal) || 0;
                vb = parseFloat(b.dataset.decimal) || 0;
                return sortDir === 'asc' ? va - vb : vb - va;
            }
            const cmp = va < vb ? -1 : va > vb ? 1 : 0;
            return sortDir === 'asc' ? cmp : -cmp;
        });

        // Clear tbody (only data rows; avoid touching DOM outside tbody)
        tbody.innerHTML = '';

        if (!grouped) {
            sorted.forEach(r => tbody.appendChild(r));
        } else {
            // Group by market type/subType
            const groups = new Map(); // label -> []
            sorted.forEach(row => {
                const lbl = groupLabel(row.dataset.marketType, row.dataset.marketSubType);
                if (!groups.has(lbl)) groups.set(lbl, []);
                groups.get(lbl).push(row);
            });

            // Sort group labels (but keep "Other" last)
            const labels = [...groups.keys()].sort((a, b) => {
                if (a === 'Other') return 1;
                if (b === 'Other') return -1;
                return a.localeCompare(b);
            });

            labels.forEach(lbl => {
                // Group header row
                const hdr = document.createElement('tr');
                hdr.className = 'market-group-header';
                hdr.innerHTML = `<td colspan="3"><span class="market-group-label">${lbl}</span></td>`;
                tbody.appendChild(hdr);
                groups.get(lbl).forEach(r => tbody.appendChild(r));
            });
        }

        // Update header sort indicators
        table.querySelectorAll('.sort-header').forEach(th => {
            const col = th.dataset.sortCol;
            const icon = th.querySelector('.sort-icon');
            if (col === sortCol) {
                th.setAttribute('aria-sort', sortDir === 'asc' ? 'ascending' : 'descending');
                th.classList.add('sort-active');
                if (icon) {
                    icon.className = `bi ${sortDir === 'asc' ? 'bi-arrow-up' : 'bi-arrow-down'} sort-icon ms-1`;
                }
            } else {
                th.removeAttribute('aria-sort');
                th.classList.remove('sort-active');
                if (icon) icon.className = 'bi bi-arrow-down-up sort-icon ms-1';
            }
        });

        // Update grouping toggle button state
        const toggleBtn = table.closest('.accordion-body')?.querySelector('.market-group-toggle');
        if (toggleBtn) {
            toggleBtn.setAttribute('aria-pressed', grouped ? 'true' : 'false');
            toggleBtn.classList.toggle('active', grouped);
        }
    }

    // Initialize all market tables
    function initMarketTables() {
        document.querySelectorAll('.market-table').forEach(table => {
            const eventId = table.querySelector('tbody tr[data-event-id]')?.dataset.eventId;
            if (!eventId) return;
            const key = `event-${eventId}`;
            if (!tablePrefs[key]) tablePrefs[key] = defaultPrefs();

            // Only register listeners once per table element
            if (!initializedTables.has(table)) {
                initializedTables.add(table);

                // Sort header clicks
                table.querySelectorAll('.sort-header').forEach(th => {
                    const handler = (e) => {
                        e.stopPropagation(); // prevent accordion toggle
                        const col = th.dataset.sortCol;
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

                // Grouping toggle
                const toggleBtn = table.closest('.accordion-body')?.querySelector('.market-group-toggle');
                if (toggleBtn) {
                    toggleBtn.addEventListener('click', (e) => {
                        e.stopPropagation();
                        const prefs = tablePrefs[key];
                        prefs.grouped = !prefs.grouped;
                        renderTable(table, prefs);
                        saveTablePrefs(tablePrefs);
                    });
                }
            }

            // Always re-render with current prefs (e.g. when accordion re-opens)
            renderTable(table, tablePrefs[key]);
        });
    }

    // Re-init tables when an accordion item is shown (lazy: rows exist in DOM from initial render)
    document.querySelectorAll('.accordion-collapse').forEach(collapse => {
        collapse.addEventListener('shown.bs.collapse', () => initMarketTables());
    });

    initMarketTables();

    // Add keyboard navigation for accessibility
    document.querySelectorAll('.accordion-button').forEach(button => {
        button.addEventListener('keypress', function(e) {
            if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                button.click();
            }
        });
    });

    // Feeling Lucky button - choose from currently visible (filtered) frontend rows
    const luckyBtn = document.getElementById('feeling-lucky');
    if (luckyBtn) {
        luckyBtn.addEventListener('click', function() {
            // Clear previous lucky highlights
            document.querySelectorAll('tr.ds-lucky-row').forEach(r => {
                r.classList.remove('ds-lucky-row', 'ds-lucky-highlight');
                const badge = r.querySelector('.ds-lucky-ribbon-inline');
                if (badge) badge.remove();
            });

            // Only consider rows from events that are currently visible after frontend filters.
            const visibleEventRows = Array.from(
                document.querySelectorAll('.accordion-item[data-event-id]:not(.filter-hidden) tr[data-event-id][data-decimal]')
            );

            if (visibleEventRows.length === 0) {
                showToast('No visible bets', 'Adjust filters to show at least one event', 'warning');
                return;
            }

            const matchedRow = visibleEventRows[Math.floor(Math.random() * visibleEventRows.length)];
            if (!matchedRow) return;

            const eventId = matchedRow.dataset.eventId;
            const marketName = matchedRow.dataset.marketName || 'Market';
            const outcome = matchedRow.dataset.outcome || 'Outcome';
            const decimal = parseFloat(matchedRow.dataset.decimal || '0');

            const placeSticker = () => {
                // Scroll after collapse is shown so row geometry is stable.
                matchedRow.scrollIntoView({ behavior: 'smooth', block: 'center' });
                matchedRow.classList.add('ds-lucky-row', 'ds-lucky-highlight');

                // Add inline sticker in odds cell so it sits left of the odds value.
                const rowSticker = document.createElement('span');
                rowSticker.className = 'ds-lucky-ribbon-inline';
                rowSticker.textContent = 'Lucky Bet';
                const oddsCell = matchedRow.querySelector('td:last-child');
                if (oddsCell) {
                    oddsCell.prepend(rowSticker);
                }

                showToast('Lucky pick!', `${marketName}: ${outcome} @ ${decimal.toFixed(2)}`, 'success');
            };

            // Expand the accordion for the selected event and place sticker after expansion.
            const collapseEl = document.querySelector(`#event-${eventId}`);
            if (collapseEl) {
                const bsCollapse = bootstrap.Collapse.getOrCreateInstance(collapseEl);
                if (!collapseEl.classList.contains('show')) {
                    collapseEl.addEventListener('shown.bs.collapse', placeSticker, { once: true });
                    bsCollapse.show();
                } else {
                    placeSticker();
                }
            } else {
                placeSticker();
            }
        });
    }
});
