# Components

GlowDoc is built with a comprehensive component system that provides consistent, accessible, and responsive UI elements throughout your documentation site.

## Layout Components

### Header Component

The main header provides navigation and theme switching functionality:

**Structure:**
```html
<header class="header-content">
  <div class="container">
    <div class="logo">Site Title</div>
    <button class="theme-toggle" aria-label="Toggle theme">
      <!-- Theme toggle icon -->
    </button>
  </div>
</header>
```

**Features:**
- Sticky positioning with backdrop blur effect
- 80px fixed height
- Responsive container with max-width of 1200px
- Integrated theme toggle button

### Sidebar Navigation

The collapsible sidebar houses the main navigation and search functionality:

**Structure:**
```html
<aside class="sidebar">
  <div class="search-container">
    <input type="text" class="search-input" placeholder="Search...">
    <div class="search-results"></div>
  </div>
  
  <nav class="navigation">
    <div class="nav-section">
      <button class="nav-section-title">Section Name</button>
      <div class="nav-section-content">
        <a href="#page" class="nav-link">Page Title</a>
      </div>
    </div>
  </nav>
</aside>
```

**Features:**
- Fixed 280px width on desktop
- Collapsible sections with smooth animations
- Integrated search with live results
- Mobile-responsive with overlay behavior

### Main Content Area

The primary content container with optimal reading layout:

**Structure:**
```html
<main class="main-content">
  <div class="content-section" id="page-id">
    <h1>Page Title</h1>
    <p>Content goes here...</p>
  </div>
</main>
```

**Features:**
- Flexible layout that grows to fill available space
- 800px max-width for optimal reading
- Responsive padding (2rem desktop, 1rem mobile)
- Centered content with proper spacing

## Navigation Components

### Navigation Links

Individual navigation items with active state support:

**CSS Classes:**
```css
.nav-link {
  /* Base navigation link styles */
  display: block;
  padding: 0.5rem 1rem;
  color: hsl(var(--muted-foreground));
  text-decoration: none;
  border-radius: 0.375rem;
  transition: all 0.2s ease;
}

.nav-link:hover {
  background-color: hsl(var(--accent));
  color: hsl(var(--accent-foreground));
}

.nav-link.active {
  background-color: hsl(var(--primary));
  color: hsl(var(--primary-foreground));
  font-weight: 600;
}
```

**Usage:**
- Automatically marked as `.active` based on current page
- Left border accent on hover and active states
- Smooth color transitions
- Accessible keyboard navigation

### Section Headers

Collapsible section headers in the navigation:

**Structure:**
```html
<button class="nav-section-title" data-section="section-id">
  <span>Section Name</span>
  <svg class="chevron-icon"><!-- Chevron icon --></svg>
</button>
```

**Features:**
- Click to expand/collapse section content
- Animated chevron icon rotation
- Maintains expanded state in LocalStorage
- Proper ARIA attributes for accessibility

## Interactive Components

### Theme Toggle

The dark/light mode switcher with system preference detection:

**Structure:**
```html
<button class="theme-toggle" aria-label="Toggle theme">
  <svg class="sun-icon"><!-- Sun icon --></svg>
  <svg class="moon-icon"><!-- Moon icon --></svg>
</button>
```

**Features:**
- Automatic system preference detection on first visit
- Smooth icon transitions between light/dark states
- LocalStorage persistence for user preference
- 0.3s transition for theme switching

### Search Component

Live search functionality with instant results:

**Structure:**
```html
<div class="search-container">
  <div class="search-input-wrapper">
    <svg class="search-icon"><!-- Search icon --></svg>
    <input type="text" class="search-input" placeholder="Search documentation...">
  </div>
  <div class="search-results">
    <div class="search-result" data-target="page-id">
      <div class="search-result-title">Page Title</div>
      <div class="search-result-excerpt">Matching content...</div>
    </div>
  </div>
</div>
```

**Features:**
- Real-time search as you type (300ms debounce)
- Searches through all page titles and content
- Highlighted search terms in results
- Keyboard navigation support (arrow keys, Enter)
- Click or Enter to navigate to results

### Mobile Menu

Responsive navigation for mobile devices:

**Structure:**
```html
<button class="mobile-menu-toggle">
  <span class="hamburger-line"></span>
  <span class="hamburger-line"></span>
  <span class="hamburger-line"></span>
</button>
```

**Features:**
- Animated hamburger menu icon
- Transforms sidebar into full-screen overlay
- Smooth slide-in animation from left
- Closes on backdrop click or page navigation

## Content Components

### Code Blocks

Syntax-highlighted code blocks with proper formatting:

**Markdown Usage:**
````markdown
```javascript
function greetUser(name) {
  console.log(`Hello, ${name}!`);
}
```
````

**Generated HTML:**
```html
<pre><code class="language-javascript">
function greetUser(name) {
  console.log(`Hello, ${name}!`);
}
</code></pre>
```

**Features:**
- Automatic syntax highlighting via CSS
- Consistent background and padding
- Horizontal scrolling for long lines
- Copy-friendly formatting

### Content Sections

Organized content areas for each documentation page:

**Structure:**
```html
<div class="content-section" id="unique-page-id">
  <h1>Page Title</h1>
  <p>Introduction paragraph...</p>
  
  <h2>Section Heading</h2>
  <p>Section content...</p>
</div>
```

**Features:**
- Hidden by default (only active page shown)
- Smooth fade-in transitions when activated
- Proper heading hierarchy (H1 for page title, H2+ for sections)
- Semantic HTML structure for accessibility

### Search Result Highlighting

Dynamic highlighting of search terms in content:

**Generated Markup:**
```html
<p>This is some <mark class="search-highlight">highlighted</mark> text.</p>
```

**CSS Styling:**
```css
.search-highlight {
  background-color: hsl(var(--primary) / 0.2);
  color: hsl(var(--primary-foreground));
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
}
```

## Responsive Behavior

### Breakpoint System

GlowDoc uses a mobile-first approach with a single breakpoint:

```css
/* Mobile styles (default) */
.sidebar {
  position: fixed;
  left: -100%;
  transition: left 0.3s ease;
}

/* Desktop styles */
@media (min-width: 768px) {
  .sidebar {
    position: fixed;
    left: 0;
  }
}
```

### Mobile Adaptations

**Sidebar Behavior:**
- Becomes full-screen overlay on mobile
- Slide-in animation from left edge
- Backdrop blur and dark overlay
- Closes on outside click or page navigation

**Content Layout:**
- Single-column layout on mobile
- Reduced padding (1rem vs 2rem)
- Optimized touch targets (44px minimum)
- Larger font sizes for better readability

**Navigation:**
- Collapsible sections remain functional
- Touch-optimized interactive areas
- Simplified hover states for touch devices

## Accessibility Features

### Keyboard Navigation

**Navigation Support:**
- Tab order follows logical content flow
- Arrow keys navigate search results
- Enter key activates links and buttons
- Escape key closes mobile menu and search

**Focus Management:**
- Visible focus indicators on all interactive elements
- Focus trapped within mobile menu when open
- Focus restored when closing overlays

### Screen Reader Support

**ARIA Labels:**
```html
<button aria-label="Toggle theme" class="theme-toggle">
<input aria-label="Search documentation" class="search-input">
<nav aria-label="Main navigation" class="navigation">
```

**Semantic Structure:**
- Proper heading hierarchy (H1 → H2 → H3)
- Landmark roles for main sections
- Descriptive link text
- Form labels associated with inputs

### Color and Contrast

**WCAG AA Compliance:**
- 4.5:1 contrast ratio for normal text
- 3:1 contrast ratio for large text
- Enhanced focus states with both color and outline
- Color is not the only means of conveying information

## Performance Optimizations

### CSS Architecture

**Efficient Selectors:**
- Class-based selectors for performance
- Minimal nesting depth
- Optimized specificity

**Transition Performance:**
- GPU-accelerated transforms
- Efficient property animations (opacity, transform)
- Reasonable transition durations (0.2s-0.3s)

### JavaScript Optimization

**Search Debouncing:**
- 300ms delay to reduce excessive searches
- Efficient DOM queries using data attributes
- Minimal DOM manipulation

**Event Handling:**
- Event delegation for dynamic content
- Passive event listeners where appropriate
- Memory leak prevention

## Customization Examples

### Custom Navigation Styling

```css
.nav-link {
  /* Add custom styles */
  border-left: 3px solid transparent;
  margin-bottom: 0.25rem;
}

.nav-link:hover {
  border-left-color: hsl(var(--primary));
  background: linear-gradient(90deg, 
    hsl(var(--primary) / 0.1), 
    transparent
  );
}

.nav-link.active {
  border-left-color: hsl(var(--primary));
  background-color: hsl(var(--primary) / 0.15);
}
```

### Custom Search Styling

```css
.search-container {
  /* Custom search appearance */
  background: hsl(var(--card));
  border: 1px solid hsl(var(--border));
  border-radius: 0.75rem;
  padding: 1rem;
  margin-bottom: 1.5rem;
}

.search-input {
  /* Enhanced input styling */
  background: hsl(var(--background));
  border: 2px solid hsl(var(--border));
  border-radius: 0.5rem;
  padding: 0.75rem 1rem 0.75rem 2.5rem;
}

.search-input:focus {
  border-color: hsl(var(--primary));
  box-shadow: 0 0 0 3px hsl(var(--primary) / 0.1);
}
```

### Custom Content Styling

```css
.content-section {
  /* Enhanced content presentation */
  line-height: 1.75;
  max-width: 900px; /* Wider content area */
}

.content-section h1 {
  /* Custom heading styles */
  border-bottom: 2px solid hsl(var(--border));
  padding-bottom: 1rem;
  margin-bottom: 2rem;
}

.content-section h2 {
  /* Section heading enhancements */
  margin-top: 3rem;
  margin-bottom: 1.5rem;
  color: hsl(var(--primary));
}
```

This component system provides a solid foundation for building beautiful, functional documentation sites while maintaining consistency and accessibility across all interface elements.