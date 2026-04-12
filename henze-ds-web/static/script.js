// Table sorting functionality
document.addEventListener('DOMContentLoaded', function() {
    const table = document.getElementById('bets-table');
    if (!table) return;

    const headers = table.querySelectorAll('th.sortable');
    const tbody = table.querySelector('tbody');
    let currentSort = { column: null, direction: 'asc' };

    headers.forEach(header => {
        header.addEventListener('click', () => {
            const sortKey = header.dataset.sort;
            
            // Determine sort direction
            if (currentSort.column === sortKey) {
                currentSort.direction = currentSort.direction === 'asc' ? 'desc' : 'asc';
            } else {
                currentSort.column = sortKey;
                currentSort.direction = 'asc';
            }

            // Update header classes
            headers.forEach(h => {
                h.classList.remove('sorted-asc', 'sorted-desc');
            });
            header.classList.add(`sorted-${currentSort.direction}`);

            // Sort the table
            sortTable(sortKey, currentSort.direction);
        });
    });

    function sortTable(sortKey, direction) {
        const rows = Array.from(tbody.querySelectorAll('tr'));
        const columnIndex = getColumnIndex(sortKey);

        rows.sort((a, b) => {
            const aValue = getCellValue(a, columnIndex, sortKey);
            const bValue = getCellValue(b, columnIndex, sortKey);

            if (sortKey === 'odds') {
                // Numeric sort for odds
                return direction === 'asc' 
                    ? aValue - bValue 
                    : bValue - aValue;
            } else if (sortKey === 'time') {
                // Date sort for time
                const aDate = new Date(aValue);
                const bDate = new Date(bValue);
                return direction === 'asc' 
                    ? aDate - bDate 
                    : bDate - aDate;
            } else {
                // String sort for other columns
                return direction === 'asc'
                    ? aValue.localeCompare(bValue)
                    : bValue.localeCompare(aValue);
            }
        });

        // Re-append rows in sorted order
        rows.forEach(row => tbody.appendChild(row));
    }

    function getColumnIndex(sortKey) {
        const map = {
            'event': 0,
            'time': 1,
            'market': 2,
            'outcome': 3,
            'odds': 4
        };
        return map[sortKey] ?? 0;
    }

    function getCellValue(row, columnIndex, sortKey) {
        const cell = row.cells[columnIndex];
        const text = cell.textContent.trim();
        
        if (sortKey === 'odds') {
            return parseFloat(text) || 0;
        }
        return text;
    }

    // Add keyboard navigation for accessibility
    headers.forEach(header => {
        header.setAttribute('tabindex', '0');
        header.setAttribute('role', 'button');
        header.addEventListener('keypress', (e) => {
            if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                header.click();
            }
        });
    });
});
