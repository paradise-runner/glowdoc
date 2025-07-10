# Theming

GlowDoc features a sophisticated theming system built on CSS custom properties, supporting multiple color schemes and seamless dark mode switching.

## Built-in Themes

GlowDoc includes three professionally designed themes:

### Default Theme
Clean, neutral design perfect for professional documentation:
```yaml
# In docs/config.yaml
theme: default
```

### Purple Theme
Purple-accented design with elegant color tones:
```yaml
theme: purple
```

### Vibrant Theme
Colorful, energetic design with bold accents:
```yaml
theme: vibrant
```

## Color System Architecture

GlowDoc uses a semantic color system with HSL values for precise color control and smooth transitions.

### Core Color Properties

All themes use the same CSS custom property structure:

```css
:root {
  /* Background colors */
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  
  /* Component backgrounds */
  --card: 0 0% 100%;
  --card-foreground: 222.2 84% 4.9%;
  --popover: 0 0% 100%;
  --popover-foreground: 222.2 84% 4.9%;
  
  /* Semantic colors */
  --primary: 222.2 47.4% 11.2%;
  --primary-foreground: 210 40% 98%;
  --secondary: 210 40% 96%;
  --secondary-foreground: 222.2 84% 4.9%;
  --muted: 210 40% 96%;
  --muted-foreground: 215.4 16.3% 46.9%;
  
  /* Interactive elements */
  --accent: 210 40% 96%;
  --accent-foreground: 222.2 84% 4.9%;
  --destructive: 0 84.2% 60.2%;
  --destructive-foreground: 210 40% 98%;
  
  /* UI elements */
  --border: 214.3 31.8% 91.4%;
  --input: 214.3 31.8% 91.4%;
  --ring: 222.2 84% 4.9%;
  --radius: 0.5rem;
}
```

### Dark Mode Support

Each theme automatically includes dark mode variants:

```css
[data-theme="dark"] {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;
  /* ... other dark mode overrides */
}
```

**Dark Mode Features:**
- Automatic system preference detection
- Manual toggle with LocalStorage persistence
- Smooth 0.3s transitions between themes
- Optimized contrast ratios for readability

## Typography System

### Font Stack

GlowDoc uses a carefully selected system font stack for optimal performance and cross-platform consistency:

```css
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", 
             Roboto, "Helvetica Neue", Arial, sans-serif;
```

### Typography Scale

**Homepage Typography:**
- **Main Heading (H1):** 3rem (48px), weight 800, gradient text effect
- **Section Headings (H2):** 1.75rem (28px), weight 600
- **Body Text:** 1.125rem (18px) for enhanced readability

**Content Typography:**
- **Page Titles (H1):** 2.5rem (40px), weight 700
- **Section Headings (H2):** 1.75rem (28px), weight 600
- **Body Text:** 1rem (16px) for optimal reading
- **Navigation:** 0.875rem (14px), weight 500-600
- **Code:** 0.875rem (14px) monospace

### Special Typography Effects

**Gradient Text (Homepage):**
```css
background: linear-gradient(135deg, 
  hsl(var(--primary)), 
  hsl(var(--accent))
);
-webkit-background-clip: text;
color: transparent;
```

## Spacing System

GlowDoc uses a consistent spacing scale based on rem units:

```css
/* Spacing scale */
--space-1: 0.25rem;  /* 4px */
--space-2: 0.5rem;   /* 8px */
--space-3: 0.75rem;  /* 12px */
--space-4: 1rem;     /* 16px */
--space-6: 1.5rem;   /* 24px */
--space-8: 2rem;     /* 32px */
--space-16: 4rem;    /* 64px */
```

**Common Usage:**
- Small margins: `0.25rem` (4px)
- Button padding: `0.5rem` (8px)
- Standard spacing: `1rem` (16px)
- Section gaps: `1.5rem` (24px)
- Content padding: `2rem` (32px)
- Large sections: `4rem` (64px)

## Custom Theme Creation

### 1. Modify Existing Theme

To customize an existing theme, you'll need to modify the CSS generation in the Rust source:

**Location:** `src/main.rs` in the `generate_css()` function

### 2. Color Customization

Update the HSL values for any theme:

```css
:root {
  /* Change primary brand color */
  --primary: 220 70% 50%;  /* Blue instead of dark gray */
  --primary-foreground: 0 0% 100%;
  
  /* Adjust accent color */
  --accent: 160 60% 45%;   /* Teal accent */
  --accent-foreground: 0 0% 100%;
}
```

### 3. Typography Customization

**Custom Font Integration:**
```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap');

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
}
```

**Custom Font Sizes:**
```css
/* Larger base font size */
.main-content {
  font-size: 1.125rem;
  line-height: 1.75;
}

/* Custom heading sizes */
h1 { font-size: 3rem; }
h2 { font-size: 2rem; }
h3 { font-size: 1.5rem; }
```

## Advanced Theming

### Custom CSS Properties

Add your own custom properties for consistent theming:

```css
:root {
  /* Custom brand colors */
  --brand-blue: 220 90% 56%;
  --brand-green: 142 71% 45%;
  --brand-orange: 25 95% 53%;
  
  /* Custom spacing */
  --content-width: 900px;
  --sidebar-width: 320px;
  
  /* Custom shadows */
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1);
}
```

### Responsive Design Variables

```css
:root {
  --mobile-breakpoint: 768px;
  --sidebar-width: 280px;
  --mobile-padding: 1rem;
  --desktop-padding: 2rem;
}

@media (max-width: 768px) {
  .main-content {
    padding: var(--mobile-padding);
  }
}
```

### Animation Customization

```css
:root {
  /* Transition speeds */
  --transition-fast: 0.15s;
  --transition-normal: 0.2s;
  --transition-slow: 0.3s;
  
  /* Easing functions */
  --ease-out: cubic-bezier(0.0, 0.0, 0.2, 1);
  --ease-in-out: cubic-bezier(0.4, 0.0, 0.2, 1);
}

/* Apply to interactive elements */
.nav-link {
  transition: all var(--transition-normal) var(--ease-out);
}
```

## Theme Implementation Details

### Theme Switching Mechanism

GlowDoc implements theme switching through:

1. **Data attribute:** `data-theme="light|dark"` on the `<html>` element
2. **JavaScript toggle:** Smooth transitions between light/dark modes
3. **LocalStorage:** Persistent user preference storage
4. **System detection:** Automatic theme based on user's OS preference

### Color Accessibility

All themes maintain WCAG AA contrast ratios:
- Normal text: 4.5:1 contrast ratio
- Large text: 3:1 contrast ratio
- Interactive elements: Enhanced focus states

### Performance Considerations

- CSS custom properties enable instant theme switching
- No additional HTTP requests for theme assets
- Optimized for both light and dark viewing conditions
- Smooth transitions without layout shifts

## Troubleshooting

**Theme not applying:** Ensure the theme name in `config.yaml` matches exactly: `default`, `purple`, or `vibrant`.

**Dark mode not working:** Check that JavaScript is enabled and the browser supports CSS custom properties.

**Custom colors not showing:** Verify HSL values are properly formatted: `220 70% 50%` (without `hsl()` wrapper).

**Typography issues:** Ensure font declarations come after the base stylesheet in the build process.