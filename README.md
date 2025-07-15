# GlowDoc

GlowDoc is a modern, lightweight documentation template that presents a beautiful thoughtful space for your work.  Built with HTML, CSS, and minimal JavaScript for fast loading and easy customization.

<img src="docs/diagonal_comparison.png" width=100%/>

We have a hosted version of _these very docs_ on [github pages](https://paradise-runner.github.io/glowdoc/) for you to see how markdown can be transformed _very quickly_ and _beautifully_.

## Features

- ✨ **Modern Design** - Clean, professional appearance with beautiful typography
- 🌙 **Dark Mode** - Built-in theme switching with smooth transitions
- 📱 **Responsive** - Perfect on desktop, tablet, and mobile devices
- ⚡ **Fast** - Minimal dependencies and optimized performance
- ♿ **Accessible** - Built with accessibility best practices
- 🎨 **Customizable** - Easy to modify colors, fonts, and layout
- 📝 **Markdown Support** - Write documentation in Markdown files
- 🔥 **Hot Reload** - Live development server with instant browser refresh
- 🖼️ **Rich Media** - Full support for images, videos, and static assets

## Quick Start

### Option 1: Use as Static HTML

1. Download or clone this repository
2. Open `index.html` in your browser
3. Start editing the content

### Option 2: Build from Markdown

1. Install Rust (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Set up your documentation structure:

   ```bash
   # Auto-generate config from existing docs
   cargo run init-config
   ```

3. **Development Mode (Recommended):**

   ```bash
   # Start development server with hot reload
   cargo run watch
   ```
   
   Then open http://localhost:8000 in your browser. Changes to files in `docs/` will automatically rebuild the site and refresh your browser.

4. **Production Build:**

   ```bash
   # Build the site once
   cargo run --release
   
   # Serve with any static server
   python3 -m http.server 8000
   ```

## Documentation Structure

```
docs/
├── config.yaml                 # Navigation configuration
├── entry.md                    # Homepage content
├── introduction/
│   ├── quick-start.md
│   └── what-is-glowdoc.md
├── getting-started/
│   ├── installation.md
│   ├── configuration.md
│   └── first-steps.md
├── images/                      # Static assets (images, etc.)
│   ├── logo.png
│   └── screenshots/
├── customization/
│   ├── theming.md
│   ├── components.md
│   └── styling.md
└── advanced/
    ├── plugins.md
    ├── deployment.md
    └── api.md
```

## Configuration

### Auto-Generate Configuration

GlowDoc can automatically generate `docs/config.yaml` by scanning your documentation folder:

```bash
# Interactive configuration builder
cargo run init-config

# Or generate the current structure non-interactively
cargo run init-config \
   --title "GlowDoc" \
   --description "modern docs for the modern world" \
   --section-order introduction,getting-started,customization,advanced \
   --rename-page getting-started/installation.md="Installation" \
   --page-order introduction=what-is-glowdoc.md,quick-start.md \
   --page-order getting-started=installation.md,configuration.md,first-steps.md \
   --page-order customization=theming.md,components.md,styling.md \
   --page-order advanced=plugins.md,deployment.md,api.md
```

This command generates the exact config.yaml currently used by this project.

### Manual Configuration

Edit `docs/config.yaml` to customize:

- Site title and description
- Navigation structure
- Page organization

## Development Workflow

### Adding New Pages

1. Create a new markdown file in the appropriate folder
2. Add the page to `docs/config.yaml` (or regenerate with `cargo run init-config`)
3. If using development mode (`cargo run watch`), changes are automatically applied
4. Otherwise, run `cargo run --release` to regenerate the site

### Working with Images and Assets

Place images and other static assets in the `docs/` directory:

```markdown
# In your markdown files:
![Logo](images/logo.png)
![Screenshot](screenshots/demo.jpg)
![Diagram](./diagrams/architecture.svg)

# Supports these URL patterns:
- /images/logo.png
- /docs/images/logo.png  
- ./images/logo.png (relative)
```

### Hot Reload Development

The development server (`cargo run watch`) provides:

- **Instant rebuilds** when you save files
- **Automatic browser refresh** 
- **Static asset serving** (images, CSS, fonts, etc.)
- **Debounced file watching** (prevents duplicate rebuilds)
- **Error reporting** in the console

**Development URLs:**
- Documentation: http://localhost:8000
- WebSocket (hot reload): ws://localhost:8081

**Debug Mode:**
```bash
# Enable verbose logging for development server
GLOWDOC_DEBUG=1 cargo run watch

# Enable browser console debug logging
# In browser console: localStorage.setItem('glowdoc-debug', 'true')
```

## Customization

### Colors

Modify CSS custom properties in the `:root` selector:

```css
:root {
  --primary: 222.2 47.4% 11.2%;
  --secondary: 210 40% 96%;
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
}
```

### Typography

Update the font-family in the body selector or add custom fonts.

## Deployment

GlowDoc works with any static hosting service:

- **GitHub Pages**: Push to `gh-pages` branch
- **Netlify**: Connect your repository
- **Vercel**: Deploy with zero configuration
- **Any Static Host**: Upload the built files

## Testing

GlowDoc includes unit tests to ensure reliability and consistency:

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_generated_index_matches_current

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

- **`test_generated_index_matches_current`** - Verifies that the build process generates exactly the same index.html content as the current file. This test:
  - Loads the current index.html file
  - Runs the complete build process (config loading, markdown processing, HTML generation)
  - Compares the newly generated content with the existing file
  - Ensures no regressions or unexpected changes in the build output

This test is particularly useful for:
- Validating that refactoring doesn't change the output
- Ensuring consistent builds across different environments
- Catching unintended modifications to the generation logic

## Available Commands

- `cargo run init-config` - Generate config.yaml from docs structure (interactive or CLI)
- `cargo run watch` - Start development server with hot reload (recommended for development)
- `cargo run --release` - Build the site once from markdown files
- `cargo build --release` - Compile the Rust binary without running
- `cargo test` - Run unit tests to verify build consistency
- `python3 -m http.server 8000` - Serve static files (for production builds)

## License

Apache 2.0 License - feel free to use this template for your projects!
