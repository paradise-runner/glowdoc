# First Steps

Now that you have GlowDoc installed, let's build your first documentation site step by step.

## 1. Initialize Your Project

Start by setting up the basic structure for your documentation:

```bash
# Create a new project directory
mkdir my-docs
cd my-docs

# Download or clone GlowDoc
# Then copy the src/ and docs/ folders to your project
```

## 2. Generate Your Configuration

Use the interactive config builder to set up your site structure:

```bash
# Interactive mode - walks you through setup
cargo run init-config
```

This will:
- Scan any existing markdown files in `docs/`
- Extract page titles from H1 headers
- Generate a `docs/config.yaml` file
- Create a backup of any existing configuration

### Example Interactive Session

```
GlowDoc Configuration Builder
============================

Site title [GlowDoc]: My Project Documentation
Description [modern docs for the modern world]: Comprehensive guide for My Project

Found 3 sections in docs/:
  1. introduction (2 files)
  2. getting-started (3 files)  
  3. api (1 file)

Would you like to reorder sections? [y/N]: y
Enter section order (comma-separated): introduction,getting-started,api

Configuration saved to docs/config.yaml
```

## 3. Create Your Content

### Homepage Content

Create or edit `docs/entry.md` for your homepage:

```markdown
# My Project Documentation

Welcome to the comprehensive documentation for My Project.

## Getting Started

Follow our step-by-step guides to get up and running quickly.

## Key Features

- Feature 1: Description
- Feature 2: Description
- Feature 3: Description
```

### Add Documentation Pages

Create markdown files in organized folders:

```
docs/
├── entry.md
├── config.yaml
├── introduction/
│   ├── overview.md
│   └── installation.md
├── guides/
│   ├── quick-start.md
│   ├── configuration.md
│   └── advanced-usage.md
└── reference/
    └── api.md
```

Each markdown file should start with an H1 header:

```markdown
# Page Title

Your content here...

## Section

More content...
```

## 4. Build Your Site

Generate the documentation site:

```bash
# Build the complete site
cargo run --release
```

This creates `index.html` with your entire documentation site embedded as a single file.

## 5. Preview Your Site

Start a local server to preview your documentation:

```bash
# Serve the site locally
python3 -m http.server 8000

# Or use any other static server
# npx serve .
# php -S localhost:8000
```

Visit `http://localhost:8000` to see your documentation site.

## 6. Customize the Appearance

### Update Site Information

Edit `docs/config.yaml` to customize your site:

```yaml
title: My Project Documentation
description: Everything you need to know about My Project
theme: vibrant  # or 'default'
```

### Adjust Navigation

Reorder sections and pages by editing the navigation structure:

```yaml
navigation:
  - title: Introduction
    id: introduction
    items:
      - title: Overview
        id: overview
        file: introduction/overview.md
      - title: Installation
        id: installation
        file: introduction/installation.md
```

### Rebuild After Changes

After modifying configuration or content:

```bash
cargo run --release
```

## 7. Development Workflow

Establish an efficient workflow for ongoing documentation work:

### Quick Development Loop

```bash
# 1. Edit markdown files in docs/
# 2. Rebuild the site
cargo run --release

# 3. Refresh browser to see changes
# (No need to restart the server)
```

### Adding New Pages

1. Create the markdown file in the appropriate `docs/` subfolder
2. Run the config generator to update navigation:
   ```bash
   cargo run init-config
   ```
3. Rebuild the site:
   ```bash
   cargo run --release
   ```

### Reorganizing Content

Use CLI options for batch updates:

```bash
# Reorder sections and rename them
cargo run init-config \
  --section-order intro,guide,reference \
  --rename-section intro="Getting Started" \
  --rename-section guide="User Guide"
```

## Next Steps

Now that you have a working documentation site:

1. **Explore Customization**: Learn about theming and styling options
2. **Add More Content**: Expand your documentation with additional pages
3. **Deploy Your Site**: Set up hosting for your documentation
4. **Advanced Features**: Explore plugins and advanced configuration

## Common Tasks

### Adding a New Section

1. Create a new folder in `docs/`: `mkdir docs/new-section`
2. Add markdown files to the folder
3. Run `cargo run init-config` to detect the new section
4. Rebuild: `cargo run --release`

### Reordering Pages

```bash
cargo run init-config --page-order section=page1.md,page2.md,page3.md
```

### Excluding Draft Content

```bash
cargo run init-config --exclude-section drafts
```

### Custom Page Titles

Override auto-detected titles in `docs/config.yaml`:

```yaml
- title: Custom Navigation Title
  id: page-id
  file: section/actual-filename.md
```

## Troubleshooting

**Build errors**: Check that all files referenced in `config.yaml` exist and paths are correct.

**Missing navigation**: Ensure your markdown files have H1 headers and are included in the config.

**Styling issues**: Verify the theme setting in config.yaml and rebuild the site.

**Server not accessible**: Check that the server is running and try `http://localhost:8000` instead of `127.0.0.1`.