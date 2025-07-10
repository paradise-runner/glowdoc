# API Reference

Comprehensive JavaScript API reference for programmatic control and customization of GlowDoc documentation sites.

## Overview

GlowDoc generates a single-page application with a rich JavaScript API for navigation, search, theming, and customization. All functionality is embedded within the generated HTML file, providing a complete client-side documentation experience.

## Core Navigation API

### `showContent(contentId, updateUrl = true)`

Displays a specific documentation page by content ID.

**Parameters:**
- `contentId` (string) - The unique identifier for the content section
- `updateUrl` (boolean, optional) - Whether to update browser URL and history (default: true)

**Returns:** `void`

**Example:**
```javascript
// Show the installation page
showContent('installation');

// Show content without updating URL (for programmatic navigation)
showContent('api-reference', false);
```

**Behavior:**
- Switches from homepage to documentation layout if needed
- Hides all content sections and displays the target section
- Updates active state in navigation sidebar
- Updates browser URL and history (unless `updateUrl` is false)
- Automatically closes mobile sidebar
- Logs content display for debugging

### `showHomepage()`

Displays the homepage content and hides documentation layout.

**Parameters:** None  
**Returns:** `void`

**Example:**
```javascript
// Return to homepage
showHomepage();
```

**Behavior:**
- Shows homepage element, hides documentation layout
- Updates browser URL to root path
- Uses HTML5 History API for navigation

### `showDocs()`

Switches the interface to documentation mode (internal function).

**Parameters:** None  
**Returns:** `void`

**Usage:** Typically called internally by `showContent()`, but available for custom implementations.

### `showContentFromSearch(contentId)`

Displays content selected from search results and clears search state.

**Parameters:**
- `contentId` (string) - The content ID to display

**Returns:** `void`

**Example:**
```javascript
// Show search result and clear search
showContentFromSearch('quick-start');
```

**Behavior:**
- Clears search input field
- Hides search results, shows navigation
- Calls `showContent()` to display the selected page

## Theme Management API

### `toggleTheme()`

Toggles between light and dark theme modes.

**Parameters:** None  
**Returns:** `void`

**Example:**
```javascript
// Toggle theme
toggleTheme();

// Programmatically check current theme
const currentTheme = document.documentElement.getAttribute('data-theme');
console.log('Current theme:', currentTheme); // 'light' or 'dark'
```

**Behavior:**
- Toggles `data-theme` attribute between 'light' and 'dark'
- Saves theme preference to localStorage
- Provides smooth transitions via CSS
- Respects system preferences on first visit

**Theme Persistence:**
```javascript
// Theme is automatically saved to localStorage
localStorage.getItem('theme'); // Returns 'light' or 'dark'
```

## Navigation and Sidebar API

### `toggleSidebar()`

Toggles sidebar visibility (primarily for mobile interfaces).

**Parameters:** None  
**Returns:** `void`

**Example:**
```javascript
// Toggle mobile sidebar
toggleSidebar();

// Check sidebar state
const sidebar = document.querySelector('.sidebar');
const isVisible = sidebar.classList.contains('visible');
```

**Behavior:**
- Toggles 'visible' class on sidebar element
- Provides slide-in animation on mobile devices
- Automatically handled for responsive breakpoints

### `toggleSection(sectionId)`

Expands or collapses navigation sections in the sidebar.

**Parameters:**
- `sectionId` (string) - The ID of the section to toggle

**Returns:** `void`

**Example:**
```javascript
// Toggle a navigation section
toggleSection('getting-started');

// Check section state
const section = document.querySelector('[data-section="getting-started"]');
const isCollapsed = section.classList.contains('collapsed');
```

**Behavior:**
- Toggles 'collapsed' class on section and its toggle icon
- Provides smooth expand/collapse animations
- State persisted for user experience

## Search API

### `performSearch()`

Performs real-time search across all documentation content.

**Parameters:** None (reads from search input element)  
**Returns:** `void`

**Example:**
```javascript
// Trigger search programmatically
document.querySelector('.search-input').value = 'installation';
performSearch();

// Search is automatically triggered on input
```

**Search Features:**
- **Real-time Results**: Updates as user types
- **Content Indexing**: Searches titles, sections, and full content
- **Result Ranking**: Title matches rank higher than content matches
- **Snippet Generation**: Shows relevant content excerpts
- **Keyword Highlighting**: Highlights matching terms in results

**Search Index Structure:**
```javascript
// Global searchIndex object
const searchIndex = {
  "page-id": {
    "title": "Page Title",
    "section": "Section Name",
    "content": "Full searchable content..."
  }
  // ... more pages
};
```

### Custom Search Integration

```javascript
// Access search index for custom functionality
function customSearch(query) {
  const results = [];
  for (const [id, data] of Object.entries(searchIndex)) {
    if (data.title.toLowerCase().includes(query.toLowerCase())) {
      results.push({ id, ...data });
    }
  }
  return results;
}

// Example: Find all pages in a specific section
function findBySection(sectionName) {
  return Object.entries(searchIndex)
    .filter(([id, data]) => data.section === sectionName)
    .map(([id, data]) => ({ id, ...data }));
}
```

## URL and History Management

### `loadFromUrl()`

Loads appropriate content based on current URL hash.

**Parameters:** None  
**Returns:** `void`

**Example:**
```javascript
// Load content based on URL
loadFromUrl();

// Handle URL changes
window.addEventListener('hashchange', loadFromUrl);
```

**URL Format:**
- Homepage: `#` or no hash
- Content pages: `#page-id`
- Automatically handles invalid page IDs

**History Management:**
```javascript
// Navigation automatically updates browser history
// Back/forward buttons work seamlessly
window.addEventListener('popstate', (event) => {
  if (event.state?.contentId) {
    showContent(event.state.contentId, false);
  } else if (event.state?.page === 'home') {
    showHomepage();
  }
});
```

## Event System

### Built-in Event Listeners

GlowDoc automatically registers several event listeners:

```javascript
// Navigation clicks
document.addEventListener('click', (e) => {
  if (e.target.hasAttribute('data-content-id')) {
    e.preventDefault();
    showContent(e.target.getAttribute('data-content-id'));
  }
});

// Browser navigation
window.addEventListener('popstate', (event) => {
  // Handle back/forward navigation
});

// Initial load
document.addEventListener('DOMContentLoaded', () => {
  loadFromUrl();
});

// Mobile sidebar - outside clicks
document.addEventListener('click', (e) => {
  // Close sidebar when clicking outside on mobile
});
```

### Custom Event Handling

```javascript
// Listen for content changes
function onContentChange(contentId) {
  console.log('Content changed to:', contentId);
  // Custom logic here
}

// Override or extend existing functions
const originalShowContent = showContent;
showContent = function(contentId, updateUrl = true) {
  onContentChange(contentId);
  return originalShowContent(contentId, updateUrl);
};
```

## Configuration and Customization

### Global Configuration

```javascript
// Access current state
const getCurrentContent = () => {
  const activeSection = document.querySelector('.content-section:not([style*="display: none"])');
  return activeSection?.id;
};

const getCurrentTheme = () => {
  return document.documentElement.getAttribute('data-theme');
};

// Get navigation state
const getNavigationState = () => {
  const collapsedSections = Array.from(document.querySelectorAll('.nav-section.collapsed'))
    .map(section => section.dataset.section);
  return { collapsedSections };
};
```

### DOM Element Access

**Required Elements:**
```javascript
// Core layout elements
const homepage = document.getElementById('homepage');
const docsLayout = document.getElementById('docs-layout');
const sidebar = document.querySelector('.sidebar');

// Search elements
const searchInput = document.querySelector('.search-input');
const searchResults = document.querySelector('.search-results');
const searchResultsList = document.querySelector('.search-results-list');

// Navigation elements
const navigationContainer = document.querySelector('.navigation-container');
const contentSections = document.querySelectorAll('.content-section');
const navLinks = document.querySelectorAll('.nav-link');
```

**Data Attributes:**
- `data-content-id`: Links navigation items to content sections
- `data-section`: Identifies collapsible navigation sections
- `data-theme`: Current theme state on document element

## Advanced Customization

### Custom Navigation

```javascript
// Add custom navigation item
function addCustomNavItem(sectionId, title, contentId) {
  const navSection = document.querySelector(`[data-section="${sectionId}"] .nav-section-content`);
  if (navSection) {
    const link = document.createElement('a');
    link.href = `#${contentId}`;
    link.className = 'nav-link';
    link.setAttribute('data-content-id', contentId);
    link.textContent = title;
    navSection.appendChild(link);
  }
}

// Custom content injection
function addCustomContent(contentId, title, htmlContent) {
  const contentSection = document.createElement('div');
  contentSection.className = 'content-section';
  contentSection.id = contentId;
  contentSection.style.display = 'none';
  contentSection.innerHTML = `<h1>${title}</h1>${htmlContent}`;
  
  document.querySelector('.main-content').appendChild(contentSection);
  
  // Add to search index
  searchIndex[contentId] = {
    title: title,
    section: 'Custom',
    content: contentSection.textContent
  };
}
```

### Theme Customization

```javascript
// Custom theme switching
function setCustomTheme(themeName) {
  document.documentElement.setAttribute('data-theme', themeName);
  localStorage.setItem('theme', themeName);
}

// Theme change detection
const observer = new MutationObserver((mutations) => {
  mutations.forEach((mutation) => {
    if (mutation.attributeName === 'data-theme') {
      const newTheme = document.documentElement.getAttribute('data-theme');
      console.log('Theme changed to:', newTheme);
      // Custom theme change logic
    }
  });
});

observer.observe(document.documentElement, {
  attributes: true,
  attributeFilter: ['data-theme']
});
```

### Search Customization

```javascript
// Custom search implementation
function customPerformSearch() {
  const query = document.querySelector('.search-input').value.toLowerCase().trim();
  const resultsContainer = document.querySelector('.search-results-list');
  
  if (!query) {
    // Hide search results
    document.querySelector('.search-results').style.display = 'none';
    document.querySelector('.navigation-container').style.display = 'block';
    return;
  }
  
  const results = [];
  
  // Custom search logic
  for (const [id, data] of Object.entries(searchIndex)) {
    let score = 0;
    
    // Title match (highest priority)
    if (data.title.toLowerCase().includes(query)) score += 10;
    
    // Section match (medium priority)
    if (data.section.toLowerCase().includes(query)) score += 5;
    
    // Content match (lower priority)
    if (data.content.toLowerCase().includes(query)) score += 1;
    
    if (score > 0) {
      results.push({ id, ...data, score });
    }
  }
  
  // Sort by score (descending)
  results.sort((a, b) => b.score - a.score);
  
  // Display results
  displaySearchResults(results, query);
}

function displaySearchResults(results, query) {
  const resultsContainer = document.querySelector('.search-results-list');
  
  if (results.length === 0) {
    resultsContainer.innerHTML = '<div class="no-results">No results found</div>';
  } else {
    resultsContainer.innerHTML = results.map(result => {
      const snippet = generateSnippet(result.content, query);
      return `
        <div class="search-result" onclick="showContentFromSearch('${result.id}')">
          <div class="search-result-title">${highlightText(result.title, query)}</div>
          <div class="search-result-section">${result.section}</div>
          <div class="search-result-snippet">${snippet}</div>
        </div>
      `;
    }).join('');
  }
  
  // Show search results
  document.querySelector('.search-results').style.display = 'block';
  document.querySelector('.navigation-container').style.display = 'none';
}

function generateSnippet(content, query, maxLength = 150) {
  const queryIndex = content.toLowerCase().indexOf(query.toLowerCase());
  if (queryIndex === -1) {
    return content.substring(0, maxLength) + (content.length > maxLength ? '...' : '');
  }
  
  const start = Math.max(0, queryIndex - 50);
  const end = Math.min(content.length, queryIndex + query.length + 50);
  const snippet = content.substring(start, end);
  
  return (start > 0 ? '...' : '') + 
         highlightText(snippet, query) + 
         (end < content.length ? '...' : '');
}

function highlightText(text, query) {
  const regex = new RegExp(`(${query})`, 'gi');
  return text.replace(regex, '<mark class="search-highlight">$1</mark>');
}
```

## Performance and Optimization

### Debounced Search

```javascript
// Implement search debouncing
let searchTimeout;
function debouncedSearch() {
  clearTimeout(searchTimeout);
  searchTimeout = setTimeout(performSearch, 300);
}

// Replace default search input handler
document.querySelector('.search-input').addEventListener('input', debouncedSearch);
```

### Lazy Loading

```javascript
// Lazy load content sections
const observerOptions = {
  root: null,
  rootMargin: '100px',
  threshold: 0.1
};

const contentObserver = new IntersectionObserver((entries) => {
  entries.forEach(entry => {
    if (entry.isIntersecting) {
      // Load heavy content when section becomes visible
      loadSectionAssets(entry.target);
    }
  });
}, observerOptions);

// Observe all content sections
document.querySelectorAll('.content-section').forEach(section => {
  contentObserver.observe(section);
});
```

## Error Handling and Debugging

### Debug Mode

```javascript
// Enable debug mode
window.GLOWDOC_DEBUG = true;

// Enhanced showContent with debugging
function debugShowContent(contentId, updateUrl = true) {
  if (window.GLOWDOC_DEBUG) {
    console.log('Showing content:', contentId);
    console.log('Available content IDs:', Object.keys(searchIndex));
    console.log('Update URL:', updateUrl);
  }
  
  const contentElement = document.getElementById(contentId);
  if (!contentElement) {
    console.error(`Content element with ID '${contentId}' not found`);
    return;
  }
  
  return showContent(contentId, updateUrl);
}
```

### Error Recovery

```javascript
// Handle missing content gracefully
function safeShowContent(contentId, fallbackId = 'introduction') {
  const contentElement = document.getElementById(contentId);
  if (!contentElement) {
    console.warn(`Content '${contentId}' not found, showing fallback`);
    return showContent(fallbackId);
  }
  return showContent(contentId);
}

// Validate navigation state
function validateNavigation() {
  const issues = [];
  
  // Check for orphaned navigation links
  document.querySelectorAll('[data-content-id]').forEach(link => {
    const contentId = link.getAttribute('data-content-id');
    if (!document.getElementById(contentId)) {
      issues.push(`Navigation link points to missing content: ${contentId}`);
    }
  });
  
  // Check for content without navigation
  document.querySelectorAll('.content-section').forEach(section => {
    const contentId = section.id;
    const navLink = document.querySelector(`[data-content-id="${contentId}"]`);
    if (!navLink) {
      issues.push(`Content section has no navigation link: ${contentId}`);
    }
  });
  
  return issues;
}
```

## Browser Compatibility

**Supported Features:**
- ES6+ JavaScript (const, let, arrow functions, template literals)
- HTML5 History API
- CSS Custom Properties
- LocalStorage
- Modern DOM APIs

**Minimum Browser Versions:**
- Chrome 49+
- Firefox 44+
- Safari 10+
- Edge 12+

**Graceful Degradation:**
```javascript
// Feature detection
if (!window.history?.pushState) {
  console.warn('History API not supported, using hash navigation');
  // Fallback to hash-based navigation
}

if (!window.localStorage) {
  console.warn('LocalStorage not supported, theme preference will not persist');
  // Use session-based theme storage
}
```

This comprehensive API reference provides complete control over GlowDoc's functionality, enabling deep customization while maintaining the system's performance and user experience benefits.