# Configuration

GlowDoc provides flexible configuration options to customize your documentation site. All configuration is managed through the `docs/config.yaml` file.

## config.yaml Overview

The configuration file controls your site's structure, navigation, and appearance:

```yaml
title: GlowDoc
description: modern docs for the modern world
theme: vibrant
navigation:
  - title: Introduction
    id: introduction
    items:
      - title: What is GlowDoc?
        id: what-is-glowdoc
        file: introduction/what-is-glowdoc.md
```

## Auto-Generate Configuration

The easiest way to create or update your configuration is using the built-in generator:

### Interactive Mode

```bash
cargo run init-config
```

This launches an interactive wizard that:
- Scans your `docs/` folder structure
- Detects existing markdown files
- Extracts page titles from H1 headers
- Guides you through customization options
- Backs up your existing config before generating a new one

### Command-Line Mode

For automated workflows, use CLI arguments:

```bash
cargo run init-config \
  --title "My Project" \
  --description "Comprehensive project documentation" \
  --section-order introduction,guide,api,advanced \
  --rename-section guide="User Guide" \
  --rename-page guide/setup.md="Installation Guide" \
  --page-order guide=setup.md,configuration.md,usage.md \
  --exclude-section drafts
```

### Available CLI Options

- `--title "Site Title"` - Set the site title
- `--description "Description"` - Set the site description
- `--section-order folder1,folder2` - Reorder sections by folder names
- `--rename-section old=new` - Rename section titles in navigation
- `--rename-page section/file.md="New Title"` - Override page titles
- `--page-order section=file1.md,file2.md` - Reorder pages within sections
- `--exclude-section folder` - Exclude folders from navigation
- `--help` - Show complete usage guide

## Manual Configuration

### Basic Settings

```yaml
title: Your Project Name
description: Brief description for SEO and page meta
theme: vibrant  # or 'default'
```

### Navigation Structure

Navigation follows a hierarchical structure with sections and items:

```yaml
navigation:
  - title: Section Name
    id: unique-section-id
    items:
      - title: Page Title
        id: unique-page-id
        file: folder/filename.md
```

**Key Rules:**
- Section `id` must be unique across all sections
- Page `id` must be unique across all pages
- `file` path is relative to the `docs/` folder
- Pages are displayed in the order they appear in the config

### Themes

GlowDoc includes built-in themes:

- `default` - Clean, professional appearance
- `vibrant` - Bold colors with enhanced contrast

## File Organization

### Recommended Structure

```
docs/
├── config.yaml          # Navigation configuration
├── entry.md            # Homepage content (optional)
├── introduction/
│   ├── overview.md
│   └── quick-start.md
├── guides/
│   ├── installation.md
│   └── configuration.md
└── reference/
    ├── api.md
    └── cli.md
```

### Markdown Files

Each markdown file should start with an H1 header:

```markdown
# Page Title

Content goes here...
```

The H1 title is automatically extracted during config generation and used as the default page title.

## Advanced Configuration

### Custom Page Titles

Override the auto-detected title from the markdown H1:

```yaml
- title: Custom Page Title  # Shows in navigation
  id: custom-page
  file: section/actual-filename.md  # H1 in file can be different
```

### Section Ordering

Control the order sections appear in navigation:

```bash
cargo run init-config --section-order introduction,tutorial,reference,advanced
```

### Page Ordering

Control the order pages appear within each section:

```bash
cargo run init-config --page-order tutorial=setup.md,basics.md,advanced.md
```

## Development Workflow

1. **Edit Configuration**: Modify `docs/config.yaml` or use `cargo run init-config`
2. **Update Content**: Edit markdown files in the `docs/` folder
3. **Rebuild Site**: Run `cargo run --release` to regenerate `index.html`
4. **Preview Changes**: Use `python3 -m http.server 8000` to serve locally

## Troubleshooting

### Common Issues

**Config validation errors:** Ensure all `id` fields are unique and all referenced files exist.

**Missing pages:** Check that file paths in config.yaml are correct and relative to the `docs/` folder.

**Build failures:** Verify YAML syntax in config.yaml using a YAML validator.

### Backup and Recovery

The config generator automatically creates backups:
- `docs/config.yaml.backup` - Created before generating new config
- Manual backup: `cp docs/config.yaml docs/config.yaml.manual-backup`