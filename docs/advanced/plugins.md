# Plugins and Extensions

Extend GlowDoc's functionality with custom plugins, third-party integrations, and advanced features to enhance your documentation experience.

## Plugin Architecture Overview

GlowDoc's modular architecture allows for various extension points and integration patterns. While GlowDoc doesn't have a formal plugin system, you can extend functionality through several approaches:

### Extension Methods

1. **CSS and JavaScript Extensions**: Add custom functionality through the Rust build process
2. **Third-Party Integrations**: Embed external services and tools
3. **Build Process Extensions**: Modify the Rust source for custom features
4. **External Tool Integration**: Combine GlowDoc with other documentation tools

## Syntax Highlighting Enhancements

### Advanced Code Highlighting

While GlowDoc includes basic syntax highlighting, you can enhance it with external libraries:

#### Prism.js Integration

Add advanced syntax highlighting with Prism.js:

```javascript
// Add to the JavaScript section in src/main.rs
function initializePrism() {
  // Load Prism.js dynamically
  const script = document.createElement('script');
  script.src = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js';
  script.onload = function() {
    // Load additional language support
    const languages = ['rust', 'javascript', 'python', 'bash', 'yaml'];
    languages.forEach(lang => {
      const langScript = document.createElement('script');
      langScript.src = `https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-${lang}.min.js`;
      document.head.appendChild(langScript);
    });
    
    // Load Prism CSS
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css';
    document.head.appendChild(link);
    
    // Re-highlight all code blocks
    Prism.highlightAll();
  };
  document.head.appendChild(script);
}

// Call during page initialization
initializePrism();
```

#### Custom Syntax Highlighting

Create custom highlighting for domain-specific languages:

```javascript
function customHighlighter() {
  document.querySelectorAll('pre code').forEach(block => {
    const language = block.className.match(/language-(\w+)/);
    if (language && language[1] === 'custom-dsl') {
      highlightCustomDSL(block);
    }
  });
}

function highlightCustomDSL(codeBlock) {
  let html = codeBlock.innerHTML;
  
  // Define custom syntax patterns
  const patterns = [
    { regex: /\b(function|if|else|return)\b/g, class: 'keyword' },
    { regex: /\b\d+\b/g, class: 'number' },
    { regex: /"[^"]*"/g, class: 'string' },
    { regex: /\/\/.*$/gm, class: 'comment' }
  ];
  
  patterns.forEach(pattern => {
    html = html.replace(pattern.regex, `<span class="${pattern.class}">$&</span>`);
  });
  
  codeBlock.innerHTML = html;
}
```

### Code Copy Functionality

Add copy-to-clipboard buttons for code blocks:

```javascript
function addCopyButtons() {
  document.querySelectorAll('pre').forEach(pre => {
    const button = document.createElement('button');
    button.className = 'copy-button';
    button.textContent = 'Copy';
    button.onclick = () => copyToClipboard(pre.textContent, button);
    
    pre.style.position = 'relative';
    pre.appendChild(button);
  });
}

function copyToClipboard(text, button) {
  navigator.clipboard.writeText(text).then(() => {
    const originalText = button.textContent;
    button.textContent = 'Copied!';
    button.classList.add('copied');
    
    setTimeout(() => {
      button.textContent = originalText;
      button.classList.remove('copied');
    }, 2000);
  });
}

// CSS for copy button
const copyButtonCSS = `
.copy-button {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  padding: 0.25rem 0.5rem;
  background: hsl(var(--primary));
  color: hsl(var(--primary-foreground));
  border: none;
  border-radius: 0.25rem;
  font-size: 0.75rem;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s ease;
}

pre:hover .copy-button {
  opacity: 1;
}

.copy-button.copied {
  background: hsl(var(--accent));
}
`;
```

## Search Enhancements

### Advanced Search Integration

#### Algolia DocSearch

Integrate Algolia's DocSearch for powerful search capabilities:

```javascript
function initializeAlgoliaSearch() {
  // Load Algolia DocSearch
  const script = document.createElement('script');
  script.src = 'https://cdn.jsdelivr.net/npm/@docsearch/js@3';
  script.onload = function() {
    docsearch({
      appId: 'YOUR_APP_ID',
      apiKey: 'YOUR_SEARCH_API_KEY',
      indexName: 'YOUR_INDEX_NAME',
      container: '#docsearch',
      searchParameters: {
        facetFilters: ['language:en'],
      },
    });
  };
  document.head.appendChild(script);
  
  // Replace existing search input
  const searchContainer = document.querySelector('.search-container');
  searchContainer.innerHTML = '<div id="docsearch"></div>';
}
```

#### Lunr.js Client-Side Search

Implement advanced client-side search with Lunr.js:

```javascript
function initializeLunrSearch() {
  // Build search index from content
  const documents = [];
  document.querySelectorAll('.content-section').forEach((section, idx) => {
    documents.push({
      id: idx,
      title: section.querySelector('h1')?.textContent || '',
      content: section.textContent,
      url: section.id
    });
  });
  
  // Create Lunr index
  const idx = lunr(function () {
    this.ref('id');
    this.field('title', { boost: 10 });
    this.field('content');
    
    documents.forEach((doc) => {
      this.add(doc);
    });
  });
  
  // Enhanced search function
  function performSearch(query) {
    const results = idx.search(query);
    return results.map(result => {
      const doc = documents[result.ref];
      return {
        ...doc,
        score: result.score,
        excerpt: generateExcerpt(doc.content, query)
      };
    });
  }
  
  function generateExcerpt(content, query, maxLength = 150) {
    const index = content.toLowerCase().indexOf(query.toLowerCase());
    if (index === -1) return content.substring(0, maxLength) + '...';
    
    const start = Math.max(0, index - 50);
    const end = Math.min(content.length, index + query.length + 50);
    const excerpt = content.substring(start, end);
    
    return (start > 0 ? '...' : '') + excerpt + (end < content.length ? '...' : '');
  }
}
```

### Search Analytics

Track search behavior and popular queries:

```javascript
function trackSearchAnalytics() {
  const searchInput = document.querySelector('.search-input');
  
  searchInput.addEventListener('input', debounce((e) => {
    const query = e.target.value;
    if (query.length > 2) {
      // Track search queries
      gtag('event', 'search', {
        search_term: query,
        page_title: document.title
      });
    }
  }, 1000));
  
  // Track search result clicks
  document.addEventListener('click', (e) => {
    if (e.target.classList.contains('search-result')) {
      gtag('event', 'search_result_click', {
        search_term: searchInput.value,
        result_title: e.target.textContent
      });
    }
  });
}

function debounce(func, wait) {
  let timeout;
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}
```

## Analytics and Tracking

### Google Analytics 4 Integration

Implement comprehensive analytics tracking:

```javascript
function initializeAnalytics() {
  // Load Google Analytics
  const script1 = document.createElement('script');
  script1.async = true;
  script1.src = 'https://www.googletagmanager.com/gtag/js?id=YOUR_GA_ID';
  document.head.appendChild(script1);
  
  const script2 = document.createElement('script');
  script2.innerHTML = `
    window.dataLayer = window.dataLayer || [];
    function gtag(){dataLayer.push(arguments);}
    gtag('js', new Date());
    gtag('config', 'YOUR_GA_ID', {
      page_title: document.title,
      page_location: window.location.href
    });
  `;
  document.head.appendChild(script2);
}

function trackUserBehavior() {
  // Track page navigation
  document.addEventListener('click', (e) => {
    if (e.target.classList.contains('nav-link')) {
      gtag('event', 'page_view', {
        page_title: e.target.textContent,
        page_location: window.location.href + '#' + e.target.getAttribute('href').substring(1)
      });
    }
  });
  
  // Track theme changes
  document.querySelector('.theme-toggle').addEventListener('click', () => {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    gtag('event', 'theme_change', {
      theme: currentTheme === 'dark' ? 'light' : 'dark'
    });
  });
  
  // Track scroll depth
  let maxScroll = 0;
  window.addEventListener('scroll', debounce(() => {
    const scrollPercent = Math.round((window.scrollY + window.innerHeight) / document.body.scrollHeight * 100);
    if (scrollPercent > maxScroll) {
      maxScroll = scrollPercent;
      if (maxScroll % 25 === 0) { // Track at 25%, 50%, 75%, 100%
        gtag('event', 'scroll_depth', {
          percent: maxScroll,
          page_title: document.title
        });
      }
    }
  }, 500));
}
```

### Hotjar Integration

Add user behavior tracking with Hotjar:

```javascript
function initializeHotjar() {
  const script = document.createElement('script');
  script.innerHTML = `
    (function(h,o,t,j,a,r){
      h.hj=h.hj||function(){(h.hj.q=h.hj.q||[]).push(arguments)};
      h._hjSettings={hjid:YOUR_HOTJAR_ID,hjsv:6};
      a=o.getElementsByTagName('head')[0];
      r=o.createElement('script');r.async=1;
      r.src=t+h._hjSettings.hjid+j+h._hjSettings.hjsv;
      a.appendChild(r);
    })(window,document,'https://static.hotjar.com/c/hotjar-','.js?sv=');
  `;
  document.head.appendChild(script);
}
```

## Accessibility Enhancements

### Screen Reader Improvements

Enhance accessibility with ARIA attributes and screen reader support:

```javascript
function enhanceAccessibility() {
  // Add skip navigation
  const skipLink = document.createElement('a');
  skipLink.href = '#main-content';
  skipLink.textContent = 'Skip to main content';
  skipLink.className = 'skip-link';
  document.body.insertBefore(skipLink, document.body.firstChild);
  
  // Improve navigation ARIA labels
  document.querySelectorAll('.nav-section-title').forEach(title => {
    const sectionId = title.getAttribute('data-section');
    title.setAttribute('aria-expanded', 'false');
    title.setAttribute('aria-controls', sectionId + '-content');
    
    const content = title.nextElementSibling;
    if (content) {
      content.setAttribute('id', sectionId + '-content');
      content.setAttribute('aria-labelledby', sectionId + '-title');
    }
  });
  
  // Announce page changes to screen readers
  const announcer = document.createElement('div');
  announcer.setAttribute('aria-live', 'polite');
  announcer.setAttribute('aria-atomic', 'true');
  announcer.className = 'sr-only';
  document.body.appendChild(announcer);
  
  // Announce when content changes
  function announcePageChange(title) {
    announcer.textContent = `Navigated to ${title}`;
    setTimeout(() => announcer.textContent = '', 1000);
  }
  
  return { announcePageChange };
}

// CSS for accessibility
const accessibilityCSS = `
.skip-link {
  position: absolute;
  top: -40px;
  left: 6px;
  background: hsl(var(--primary));
  color: hsl(var(--primary-foreground));
  padding: 8px;
  text-decoration: none;
  border-radius: 0 0 4px 4px;
  z-index: 1000;
}

.skip-link:focus {
  top: 0;
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
`;
```

### Keyboard Navigation Enhancement

Improve keyboard navigation throughout the documentation:

```javascript
function enhanceKeyboardNavigation() {
  let focusedSearchResult = -1;
  
  document.addEventListener('keydown', (e) => {
    // Search result navigation
    if (e.target.classList.contains('search-input')) {
      const results = document.querySelectorAll('.search-result');
      
      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          focusedSearchResult = Math.min(focusedSearchResult + 1, results.length - 1);
          updateSearchResultFocus(results);
          break;
          
        case 'ArrowUp':
          e.preventDefault();
          focusedSearchResult = Math.max(focusedSearchResult - 1, -1);
          updateSearchResultFocus(results);
          break;
          
        case 'Enter':
          if (focusedSearchResult >= 0 && results[focusedSearchResult]) {
            e.preventDefault();
            results[focusedSearchResult].click();
          }
          break;
          
        case 'Escape':
          e.target.blur();
          document.querySelector('.search-results').style.display = 'none';
          break;
      }
    }
    
    // Global shortcuts
    if (e.ctrlKey || e.metaKey) {
      switch (e.key) {
        case 'k':
          e.preventDefault();
          document.querySelector('.search-input').focus();
          break;
          
        case '/':
          e.preventDefault();
          document.querySelector('.search-input').focus();
          break;
      }
    }
  });
  
  function updateSearchResultFocus(results) {
    results.forEach((result, index) => {
      result.classList.toggle('focused', index === focusedSearchResult);
    });
    
    if (focusedSearchResult >= 0 && results[focusedSearchResult]) {
      results[focusedSearchResult].scrollIntoView({ block: 'nearest' });
    }
  }
}
```

## Performance Enhancements

### Lazy Loading and Code Splitting

Implement performance optimizations:

```javascript
function implementLazyLoading() {
  // Lazy load heavy components
  const observerOptions = {
    root: null,
    rootMargin: '100px',
    threshold: 0.1
  };
  
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const section = entry.target;
        loadSectionAssets(section);
        observer.unobserve(section);
      }
    });
  }, observerOptions);
  
  // Observe content sections for lazy loading
  document.querySelectorAll('.content-section').forEach(section => {
    observer.observe(section);
  });
  
  function loadSectionAssets(section) {
    // Load section-specific assets
    const codeBlocks = section.querySelectorAll('pre code');
    if (codeBlocks.length > 0) {
      loadSyntaxHighlighting(codeBlocks);
    }
    
    const diagrams = section.querySelectorAll('.mermaid');
    if (diagrams.length > 0) {
      loadMermaidDiagrams(diagrams);
    }
  }
}

function implementServiceWorker() {
  if ('serviceWorker' in navigator) {
    const swContent = `
      const CACHE_NAME = 'glowdoc-v1';
      const urlsToCache = [
        '/',
        '/index.html'
      ];
      
      self.addEventListener('install', (event) => {
        event.waitUntil(
          caches.open(CACHE_NAME)
            .then((cache) => cache.addAll(urlsToCache))
        );
      });
      
      self.addEventListener('fetch', (event) => {
        event.respondWith(
          caches.match(event.request)
            .then((response) => {
              return response || fetch(event.request);
            })
        );
      });
    `;
    
    const blob = new Blob([swContent], { type: 'application/javascript' });
    const swURL = URL.createObjectURL(blob);
    
    navigator.serviceWorker.register(swURL)
      .then((registration) => {
        console.log('ServiceWorker registered:', registration);
      })
      .catch((error) => {
        console.log('ServiceWorker registration failed:', error);
      });
  }
}
```

## Diagram and Visualization Support

### Mermaid Diagrams

Add support for Mermaid diagrams:

```javascript
function initializeMermaid() {
  const script = document.createElement('script');
  script.src = 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js';
  script.onload = function() {
    mermaid.initialize({
      startOnLoad: false,
      theme: document.documentElement.getAttribute('data-theme') === 'dark' ? 'dark' : 'default',
      securityLevel: 'loose'
    });
    
    // Process Mermaid diagrams
    document.querySelectorAll('.language-mermaid').forEach(async (element, index) => {
      const graphDefinition = element.textContent;
      const graphId = `mermaid-graph-${index}`;
      
      try {
        const { svg } = await mermaid.render(graphId, graphDefinition);
        const wrapper = document.createElement('div');
        wrapper.className = 'mermaid-wrapper';
        wrapper.innerHTML = svg;
        element.closest('pre').replaceWith(wrapper);
      } catch (error) {
        console.error('Mermaid rendering error:', error);
      }
    });
  };
  document.head.appendChild(script);
}

// Update Mermaid theme when user changes theme
function updateMermaidTheme() {
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.attributeName === 'data-theme') {
        const newTheme = document.documentElement.getAttribute('data-theme') === 'dark' ? 'dark' : 'default';
        if (window.mermaid) {
          mermaid.initialize({ theme: newTheme });
          // Re-render existing diagrams
          document.querySelectorAll('.mermaid-wrapper').forEach(async (wrapper, index) => {
            // Re-render logic here
          });
        }
      }
    });
  });
  
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-theme']
  });
}
```

### Chart.js Integration

Add interactive charts and graphs:

```javascript
function initializeCharts() {
  const script = document.createElement('script');
  script.src = 'https://cdn.jsdelivr.net/npm/chart.js';
  script.onload = function() {
    document.querySelectorAll('.chart-container').forEach(container => {
      const config = JSON.parse(container.dataset.config);
      const canvas = document.createElement('canvas');
      container.appendChild(canvas);
      
      new Chart(canvas, {
        ...config,
        options: {
          ...config.options,
          responsive: true,
          plugins: {
            ...config.options?.plugins,
            legend: {
              ...config.options?.plugins?.legend,
              labels: {
                color: getComputedStyle(document.documentElement)
                  .getPropertyValue('--foreground')
              }
            }
          }
        }
      });
    });
  };
  document.head.appendChild(script);
}
```

## Plugin Development Guide

### Creating Custom Extensions

Structure for building your own GlowDoc extensions:

```rust
// In src/main.rs, add to the generate_javascript() function

pub fn generate_custom_plugin_javascript() -> String {
    r#"
    // Custom Plugin Framework
    class GlowDocPlugin {
        constructor(name, options = {}) {
            this.name = name;
            this.options = options;
            this.hooks = {};
            this.initialized = false;
        }
        
        // Register event hooks
        on(event, callback) {
            if (!this.hooks[event]) {
                this.hooks[event] = [];
            }
            this.hooks[event].push(callback);
            return this;
        }
        
        // Trigger event hooks
        trigger(event, data) {
            if (this.hooks[event]) {
                this.hooks[event].forEach(callback => callback(data));
            }
        }
        
        // Initialize plugin
        init() {
            if (this.initialized) return;
            this.trigger('beforeInit', this.options);
            this.setup();
            this.initialized = true;
            this.trigger('afterInit', this.options);
        }
        
        // Override in subclasses
        setup() {
            throw new Error('Plugin must implement setup() method');
        }
    }
    
    // Plugin registry
    window.GlowDocPlugins = {
        plugins: new Map(),
        
        register(plugin) {
            this.plugins.set(plugin.name, plugin);
            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', () => plugin.init());
            } else {
                plugin.init();
            }
        },
        
        get(name) {
            return this.plugins.get(name);
        }
    };
    
    // Example plugin
    class ExamplePlugin extends GlowDocPlugin {
        setup() {
            this.on('beforeInit', (options) => {
                console.log('Example plugin initializing with options:', options);
            });
            
            // Add custom functionality
            this.addCustomButton();
        }
        
        addCustomButton() {
            const button = document.createElement('button');
            button.textContent = 'Custom Action';
            button.onclick = () => this.trigger('customAction', 'Hello from plugin!');
            document.querySelector('.header-content').appendChild(button);
        }
    }
    
    // Auto-register plugins
    document.addEventListener('DOMContentLoaded', () => {
        // Register built-in plugins
        const examplePlugin = new ExamplePlugin('example', { debug: true });
        GlowDocPlugins.register(examplePlugin);
    });
    "#.to_string()
}
```

### Plugin Configuration

Allow users to configure plugins through config.yaml:

```rust
// Add to Config struct in src/main.rs
#[derive(Debug, Deserialize, Serialize)]
struct PluginConfig {
    name: String,
    enabled: bool,
    options: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    title: String,
    description: String,
    navigation: Vec<NavigationSection>,
    theme: String,
    plugins: Option<Vec<PluginConfig>>,
}
```

## Best Practices for Extensions

### Performance Considerations

1. **Lazy Loading**: Only load plugins when needed
2. **Code Splitting**: Separate plugin code from core functionality
3. **Memory Management**: Clean up event listeners and resources
4. **Minimal Dependencies**: Keep external dependencies lightweight

### Security Guidelines

1. **Input Validation**: Sanitize all user inputs
2. **Content Security Policy**: Implement CSP headers
3. **External Resources**: Use integrity hashes for CDN resources
4. **User Data**: Handle user data securely

### Accessibility Standards

1. **ARIA Support**: Provide proper ARIA attributes
2. **Keyboard Navigation**: Ensure all functionality is keyboard accessible
3. **Screen Reader Compatibility**: Test with screen readers
4. **Color Contrast**: Maintain adequate contrast ratios

## Troubleshooting Plugins

**Plugin not loading**: Check browser console for JavaScript errors and verify script URLs.

**Conflicts between plugins**: Implement proper namespacing and avoid global variable conflicts.

**Performance issues**: Profile plugin performance and optimize heavy operations.

**Theme compatibility**: Ensure plugins work with all themes and dark mode.

**Mobile responsiveness**: Test plugin functionality on mobile devices.