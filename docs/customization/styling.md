# Custom Styling

Advanced styling techniques and customization patterns for creating unique GlowDoc designs that match your brand and requirements.

## Architecture Overview

GlowDoc's CSS architecture is designed for maximum customization while maintaining performance and accessibility. Understanding the core structure enables powerful customizations.

### CSS Organization

The generated stylesheet follows this structure:

```css
/* 1. CSS Reset & Base Styles */
/* 2. CSS Custom Properties (Design Tokens) */
/* 3. Layout Components */
/* 4. Navigation Components */
/* 5. Content Components */
/* 6. Interactive Components */
/* 7. Responsive Media Queries */
/* 8. Theme Variations */
```

### Modification Approach

Since GlowDoc generates a single HTML file with embedded CSS, customizations should be added to the Rust source in the `generate_css()` function:

**Location:** `src/main.rs` - Look for the `generate_css()` function

## Advanced Styling Techniques

### Custom Brand Integration

#### Brand Color System

Create a comprehensive brand color palette:

```css
:root {
  /* Primary brand colors */
  --brand-primary: 220 90% 56%;
  --brand-primary-dark: 220 90% 45%;
  --brand-primary-light: 220 90% 65%;
  
  /* Secondary brand colors */
  --brand-secondary: 160 60% 45%;
  --brand-accent: 25 95% 53%;
  --brand-neutral: 220 10% 50%;
  
  /* Semantic color mappings */
  --primary: var(--brand-primary);
  --accent: var(--brand-secondary);
  
  /* Brand gradients */
  --brand-gradient: linear-gradient(135deg, 
    hsl(var(--brand-primary)), 
    hsl(var(--brand-secondary))
  );
}
```

#### Logo and Brand Assets

Integrate custom logos and brand elements:

```css
.logo {
  background-image: url('data:image/svg+xml;base64,...');
  background-size: contain;
  background-repeat: no-repeat;
  width: 120px;
  height: 40px;
  text-indent: -9999px; /* Hide text */
}

/* Alternative: Custom font logo */
.logo {
  font-family: 'Your Brand Font', sans-serif;
  font-weight: 700;
  font-size: 1.5rem;
  color: hsl(var(--brand-primary));
}
```

### Advanced Layout Customizations

#### Multi-Column Content Layout

Create complex content layouts:

```css
.content-section {
  display: grid;
  grid-template-columns: 1fr 300px;
  gap: 2rem;
  max-width: 1200px;
}

.content-main {
  min-width: 0; /* Prevent grid blowout */
}

.content-sidebar {
  background: hsl(var(--card));
  border: 1px solid hsl(var(--border));
  border-radius: 0.5rem;
  padding: 1.5rem;
  height: fit-content;
  position: sticky;
  top: 100px; /* Account for header height */
}

@media (max-width: 1024px) {
  .content-section {
    grid-template-columns: 1fr;
  }
  
  .content-sidebar {
    order: -1; /* Move sidebar above content on mobile */
  }
}
```

#### Custom Navigation Layouts

Enhanced sidebar with custom sections:

```css
.sidebar {
  display: grid;
  grid-template-rows: auto 1fr auto;
  gap: 1rem;
}

.sidebar-header {
  padding: 1rem;
  border-bottom: 1px solid hsl(var(--border));
}

.sidebar-content {
  overflow-y: auto;
  padding: 0 1rem;
}

.sidebar-footer {
  padding: 1rem;
  border-top: 1px solid hsl(var(--border));
  background: hsl(var(--muted) / 0.5);
}

/* Custom navigation grouping */
.nav-group {
  margin-bottom: 2rem;
}

.nav-group-title {
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: hsl(var(--muted-foreground));
  margin-bottom: 0.5rem;
  padding: 0 1rem;
}
```

### Typography Enhancement

#### Custom Font Integration

Professional typography with web fonts:

```css
/* Import custom fonts */
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap');

:root {
  /* Typography system */
  --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  --font-mono: 'JetBrains Mono', 'SF Mono', Consolas, monospace;
  
  /* Type scale */
  --text-xs: 0.75rem;    /* 12px */
  --text-sm: 0.875rem;   /* 14px */
  --text-base: 1rem;     /* 16px */
  --text-lg: 1.125rem;   /* 18px */
  --text-xl: 1.25rem;    /* 20px */
  --text-2xl: 1.5rem;    /* 24px */
  --text-3xl: 1.875rem;  /* 30px */
  --text-4xl: 2.25rem;   /* 36px */
  --text-5xl: 3rem;      /* 48px */
}

body {
  font-family: var(--font-sans);
}

code, pre {
  font-family: var(--font-mono);
}
```

#### Advanced Typography Styles

Rich text formatting and hierarchy:

```css
.content-section {
  /* Enhanced reading experience */
  font-size: var(--text-lg);
  line-height: 1.7;
  color: hsl(var(--foreground));
}

.content-section h1 {
  font-size: var(--text-4xl);
  font-weight: 800;
  line-height: 1.1;
  margin-bottom: 1.5rem;
  background: var(--brand-gradient);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.content-section h2 {
  font-size: var(--text-2xl);
  font-weight: 700;
  margin-top: 3rem;
  margin-bottom: 1rem;
  position: relative;
}

.content-section h2::before {
  content: '';
  position: absolute;
  left: -2rem;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 1.5rem;
  background: hsl(var(--primary));
  border-radius: 2px;
}

/* Enhanced blockquotes */
.content-section blockquote {
  border-left: 4px solid hsl(var(--primary));
  padding-left: 1.5rem;
  margin: 2rem 0;
  font-style: italic;
  font-size: var(--text-xl);
  color: hsl(var(--muted-foreground));
}

/* Improved lists */
.content-section ul {
  list-style: none;
  padding-left: 0;
}

.content-section li {
  position: relative;
  padding-left: 1.5rem;
  margin-bottom: 0.5rem;
}

.content-section li::before {
  content: '→';
  position: absolute;
  left: 0;
  color: hsl(var(--primary));
  font-weight: 600;
}
```

### Interactive Element Enhancements

#### Advanced Button Styling

Custom button system with multiple variants:

```css
/* Button base styles */
.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 0.5rem;
  font-weight: 600;
  font-size: var(--text-sm);
  text-decoration: none;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  overflow: hidden;
}

/* Primary button */
.btn-primary {
  background: hsl(var(--primary));
  color: hsl(var(--primary-foreground));
}

.btn-primary:hover {
  background: hsl(var(--primary) / 0.9);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px hsl(var(--primary) / 0.3);
}

/* Gradient button */
.btn-gradient {
  background: var(--brand-gradient);
  color: white;
  position: relative;
}

.btn-gradient::before {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--brand-gradient);
  opacity: 0;
  transition: opacity 0.2s ease;
}

.btn-gradient:hover::before {
  opacity: 0.1;
}

/* Outline button */
.btn-outline {
  background: transparent;
  border: 2px solid hsl(var(--primary));
  color: hsl(var(--primary));
}

.btn-outline:hover {
  background: hsl(var(--primary));
  color: hsl(var(--primary-foreground));
}
```

#### Enhanced Form Styling

Professional form controls:

```css
.form-group {
  margin-bottom: 1.5rem;
}

.form-label {
  display: block;
  font-weight: 600;
  font-size: var(--text-sm);
  color: hsl(var(--foreground));
  margin-bottom: 0.5rem;
}

.form-input {
  width: 100%;
  padding: 0.75rem 1rem;
  border: 2px solid hsl(var(--border));
  border-radius: 0.5rem;
  background: hsl(var(--background));
  color: hsl(var(--foreground));
  font-size: var(--text-base);
  transition: all 0.2s ease;
}

.form-input:focus {
  outline: none;
  border-color: hsl(var(--primary));
  box-shadow: 0 0 0 3px hsl(var(--primary) / 0.1);
}

.form-input::placeholder {
  color: hsl(var(--muted-foreground));
}
```

### Animation and Micro-Interactions

#### Page Transition Effects

Smooth page transitions:

```css
.content-section {
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.3s ease;
}

.content-section.active {
  opacity: 1;
  transform: translateY(0);
}

/* Staggered animation for navigation items */
.nav-link {
  opacity: 0;
  transform: translateX(-20px);
  animation: slideInLeft 0.3s ease forwards;
}

.nav-link:nth-child(1) { animation-delay: 0.1s; }
.nav-link:nth-child(2) { animation-delay: 0.15s; }
.nav-link:nth-child(3) { animation-delay: 0.2s; }
/* ... continue pattern */

@keyframes slideInLeft {
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
```

#### Hover Effects and Micro-Interactions

Engaging interactive feedback:

```css
/* Card hover effects */
.card {
  background: hsl(var(--card));
  border: 1px solid hsl(var(--border));
  border-radius: 0.75rem;
  padding: 1.5rem;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.card::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(45deg, 
    hsl(var(--primary) / 0.1), 
    hsl(var(--accent) / 0.1)
  );
  opacity: 0;
  transition: opacity 0.3s ease;
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 24px hsl(var(--foreground) / 0.1);
  border-color: hsl(var(--primary));
}

.card:hover::before {
  opacity: 1;
}

/* Ripple effect for buttons */
.btn {
  position: relative;
  overflow: hidden;
}

.btn::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.3);
  transform: translate(-50%, -50%);
  transition: width 0.6s, height 0.6s;
}

.btn:active::after {
  width: 300px;
  height: 300px;
}
```

### Responsive Design Patterns

#### Advanced Responsive Typography

Fluid typography that scales smoothly:

```css
:root {
  /* Fluid typography using clamp() */
  --text-fluid-sm: clamp(0.875rem, 0.8rem + 0.375vw, 1rem);
  --text-fluid-base: clamp(1rem, 0.9rem + 0.5vw, 1.125rem);
  --text-fluid-lg: clamp(1.125rem, 1rem + 0.625vw, 1.25rem);
  --text-fluid-xl: clamp(1.25rem, 1.1rem + 0.75vw, 1.5rem);
  --text-fluid-2xl: clamp(1.5rem, 1.3rem + 1vw, 2rem);
  --text-fluid-3xl: clamp(1.875rem, 1.5rem + 1.875vw, 2.5rem);
  --text-fluid-4xl: clamp(2.25rem, 1.8rem + 2.25vw, 3rem);
}

body {
  font-size: var(--text-fluid-base);
}

h1 { font-size: var(--text-fluid-4xl); }
h2 { font-size: var(--text-fluid-3xl); }
h3 { font-size: var(--text-fluid-2xl); }
```

#### Container Queries (Future-Forward)

Modern responsive design using container queries:

```css
.content-section {
  container-type: inline-size;
}

/* Adjust layout based on container width, not viewport */
@container (min-width: 600px) {
  .content-grid {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 2rem;
  }
}

@container (min-width: 900px) {
  .content-grid {
    grid-template-columns: 1fr 2fr 1fr;
  }
}
```

### Dark Mode Advanced Customizations

#### Theme-Aware Components

Components that adapt intelligently to theme changes:

```css
/* Light theme specific styles */
[data-theme="light"] .hero-section {
  background: linear-gradient(135deg, 
    hsl(var(--background)), 
    hsl(var(--secondary))
  );
}

/* Dark theme specific styles */
[data-theme="dark"] .hero-section {
  background: linear-gradient(135deg, 
    hsl(var(--background)), 
    hsl(var(--card))
  );
}

/* Theme-aware shadows */
.elevated-card {
  box-shadow: 
    0 4px 6px hsl(var(--foreground) / 0.1),
    0 1px 3px hsl(var(--foreground) / 0.05);
}

[data-theme="dark"] .elevated-card {
  box-shadow: 
    0 4px 6px rgba(0, 0, 0, 0.3),
    0 1px 3px rgba(0, 0, 0, 0.2);
}
```

### Performance Optimization

#### Efficient CSS Architecture

Optimized styles for better performance:

```css
/* Use CSS custom properties for frequently changing values */
:root {
  --animation-speed: 0.2s;
  --animation-easing: cubic-bezier(0.4, 0, 0.2, 1);
}

/* Optimize animations for 60fps */
.animated-element {
  will-change: transform, opacity;
  transform: translateZ(0); /* Force hardware acceleration */
  transition: transform var(--animation-speed) var(--animation-easing);
}

/* Efficient selectors */
.nav-link { /* Good: class selector */ }
nav > ul > li > a { /* Avoid: deep nesting */ }
* { /* Avoid: universal selector */ }
```

#### Critical CSS Patterns

Inline critical styles for immediate rendering:

```css
/* Critical above-the-fold styles */
.layout,
.header-content,
.sidebar,
.main-content {
  /* Essential layout properties only */
  display: flex;
  position: relative;
}

/* Non-critical styles can be loaded later */
.fancy-animations,
.decorative-elements {
  /* Complex animations and decorative styles */
}
```

## Custom Styling Workflow

### 1. Planning Your Customizations

Before modifying styles:

1. **Audit existing styles**: Understand the current CSS architecture
2. **Define your design system**: Colors, typography, spacing, components
3. **Plan responsive behavior**: Mobile-first approach
4. **Consider accessibility**: Maintain contrast ratios and focus states

### 2. Implementation Strategy

**Recommended approach:**
1. Start with CSS custom property overrides
2. Add new component styles
3. Implement responsive variations
4. Test across themes (light/dark)
5. Validate accessibility compliance

### 3. Testing Checklist

- [ ] All themes (default, purple, vibrant)
- [ ] Light and dark modes
- [ ] Mobile and desktop layouts
- [ ] Keyboard navigation
- [ ] Screen reader compatibility
- [ ] Performance impact

## Troubleshooting Custom Styles

**Styles not applying**: Check CSS specificity and ensure your styles come after the base styles in the build order.

**Theme conflicts**: Verify that custom styles work with both light and dark modes.

**Performance issues**: Minimize complex selectors and excessive animations.

**Responsive problems**: Test on actual devices, not just browser dev tools.

**Accessibility concerns**: Use tools like axe-core to validate accessibility compliance.