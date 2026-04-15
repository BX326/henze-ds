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
            
            // Show loading animation
            showLoading();
            
            try {
                const resp = await fetch(`/api/bets?${params.toString()}`);
                if (!resp.ok) {
                    hideLoading();
                    return;
                }
                const bets = await resp.json();
                
                // Hide loading after receiving response
                hideLoading();
                
                if (!bets || bets.length === 0) {
                    showToast('No bets found', 'Try adjusting your criteria', 'warning');
                    return;
                }
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
                    
                    // Show toast with bet info
                    showToast('Lucky pick!', `${choice.outcome} @ ${parseFloat(choice.decimal).toFixed(2)}`, 'success');

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
                    showToast('Bet not displayed', 'Try searching first', 'warning');
                }
            } catch (e) {
                hideLoading();
                showToast('Error', 'Failed to fetch bets', 'error');
                console.error('Feeling Lucky failed', e);
            }
        });
    }
});
