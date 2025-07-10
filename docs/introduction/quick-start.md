# Quick Start

Get up and running with GlowDoc in minutes. This comprehensive guide will walk you through installation, setup, and creating your first professional documentation site.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required Software

**Rust (Latest Stable)**
```bash
# Install Rust using rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions, then reload your shell
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

**Git**
```bash
# Check if Git is installed
git --version

# If not installed:
# macOS: Install Xcode Command Line Tools
xcode-select --install

# Windows: Download from https://git-scm.com/download/win
# Linux (Ubuntu/Debian): sudo apt-get install git
```

**Python 3 (for local testing)**
```bash
# Check Python installation
python3 --version

# Python is usually pre-installed on macOS/Linux
# Windows: Download from https://python.org
```

### System Requirements

- **Operating System**: macOS, Linux, or Windows
- **RAM**: 512MB minimum, 1GB recommended
- **Disk Space**: 100MB for Rust toolchain + project files
- **Network**: Internet connection for dependencies (initial setup only)

## Installation Options

Choose the installation method that best fits your workflow:

### Option 1: Download Release (Quickest)

1. **Download the latest release:**
   ```bash
   # Download and extract (replace URL with actual release)
   curl -L https://github.com/username/glowdoc/archive/refs/heads/main.zip -o glowdoc.zip
   unzip glowdoc.zip
   cd glowdoc-main
   ```

2. **Build immediately:**
   ```bash
   cargo run --release
   ```

### Option 2: Clone Repository (Recommended for Development)

1. **Clone the repository:**
   ```bash
   git clone https://github.com/username/glowdoc.git
   cd glowdoc
   ```

2. **Build the documentation:**
   ```bash
   cargo run --release
   ```

### Option 3: Start Fresh Project

1. **Create new project directory:**
   ```bash
   mkdir my-docs
   cd my-docs
   ```

2. **Copy GlowDoc source files:**
   ```bash
   # Copy src/ and docs/ directories from GlowDoc
   # Copy Cargo.toml
   ```

3. **Initialize your documentation:**
   ```bash
   cargo run init-config
   ```

## Project Structure Overview

Understanding GlowDoc's structure helps you work effectively:

```
glowdoc/
├── Cargo.toml              # Rust project configuration
├── src/                    # Rust source code
│   ├── main.rs            # Main application logic
│   └── config_builder.rs   # Configuration builder
├── docs/                   # Your documentation source
│   ├── config.yaml        # Navigation configuration
│   ├── entry.md           # Homepage content (optional)
│   ├── introduction/      # Documentation sections
│   │   ├── quick-start.md
│   │   └── what-is-glowdoc.md
│   ├── getting-started/
│   │   ├── installation.md
│   │   ├── configuration.md
│   │   └── first-steps.md
│   └── [more-sections]/
├── index.html              # Generated documentation site
├── README.md              # Project information
└── .gitignore             # Git ignore rules
```

### Key Directories

- **`docs/`**: Your markdown documentation files
- **`src/`**: GlowDoc's Rust source code (modify for customization)
- **`index.html`**: Generated single-file documentation site

## 5-Minute Setup

Follow these steps to have a working documentation site in 5 minutes:

### Step 1: Build Your First Site (1 minute)

```bash
# After installation, build immediately
cargo run --release

# You should see output like:
# "Building documentation..."
# "Generated index.html successfully"
```

### Step 2: Preview Your Site (30 seconds)

```bash
# Start local server
python3 -m http.server 8000

# Open in browser
# Visit: http://localhost:8000
```

**Alternative server options:**
```bash
# Node.js users
npx serve .

# PHP users
php -S localhost:8000

# Python 2 users
python -m SimpleHTTPServer 8000
```

### Step 3: Verify Everything Works (30 seconds)

Check these features in your browser:

- [ ] Homepage loads correctly
- [ ] Navigation sidebar works
- [ ] Theme toggle (light/dark) functions
- [ ] Search functionality operates
- [ ] Mobile responsive design
- [ ] All documentation pages display

### Step 4: Customize Your Content (3 minutes)

1. **Update site information:**
   ```bash
   # Edit docs/config.yaml
   vim docs/config.yaml  # or your preferred editor
   ```
   
   ```yaml
   title: My Project Documentation
   description: Comprehensive guide for My Project
   theme: vibrant  # or 'default' or 'purple'
   ```

2. **Add your homepage content:**
   ```bash
   # Edit docs/entry.md
   vim docs/entry.md
   ```
   
   ```markdown
   # My Project Documentation
   
   Welcome to the comprehensive documentation for My Project.
   
   ## Getting Started
   
   Follow our guides to get up and running quickly.
   ```

3. **Add your first documentation page:**
   ```bash
   # Create a new page
   echo "# My First Page\n\nThis is my first documentation page." > docs/introduction/my-first-page.md
   ```

4. **Rebuild and see changes:**
   ```bash
   cargo run --release
   # Refresh your browser
   ```

## Configuration Walkthrough

### Automatic Configuration (Recommended)

Let GlowDoc detect and configure your documentation structure automatically:

```bash
# Interactive configuration wizard
cargo run init-config
```

**Example session:**
```
GlowDoc Configuration Builder
============================

Scanning docs/ directory...
Found 3 sections: introduction, getting-started, advanced

Site title [GlowDoc]: My Project Docs
Description [modern docs for the modern world]: Complete guide for My Project
Theme [vibrant]: default

Detected sections:
1. introduction (2 files)
2. getting-started (3 files)  
3. advanced (2 files)

Would you like to reorder sections? [y/N]: y
Enter new order (comma-separated): introduction,getting-started,advanced

Configuration saved to docs/config.yaml
Backup created: docs/config.yaml.backup
```

### Manual Configuration

For precise control, edit `docs/config.yaml` directly:

```yaml
title: My Project Documentation
description: Everything you need to know about My Project
theme: vibrant

navigation:
  - title: Introduction
    id: introduction
    items:
      - title: Overview
        id: overview
        file: introduction/overview.md
      - title: Quick Start
        id: quick-start
        file: introduction/quick-start.md
  
  - title: User Guide
    id: user-guide
    items:
      - title: Installation
        id: installation
        file: guide/installation.md
      - title: Configuration
        id: configuration
        file: guide/configuration.md
```

### Command-Line Configuration

For automated workflows, use CLI arguments:

```bash
# Complete automated setup
cargo run init-config \
  --title "My Project Docs" \
  --description "Comprehensive project documentation" \
  --section-order introduction,guide,api,advanced \
  --rename-section guide="User Guide" \
  --rename-section api="API Reference" \
  --page-order guide=installation.md,configuration.md,usage.md \
  --exclude-section drafts
```

## Content Creation Guide

### Writing Effective Documentation

#### Markdown Basics

GlowDoc supports GitHub-flavored markdown with extensions:

```markdown
# Page Title (H1 - use only once per page)

## Section Heading (H2)

### Subsection (H3)

**Bold text** and *italic text*

- Bullet points
- Another item

1. Numbered lists
2. Sequential items

`inline code` and:

```javascript
// Code blocks with syntax highlighting
function example() {
  return "Hello, World!";
}
```

> Blockquotes for important information

[Links to other pages](other-page.md)
[External links](https://example.com)

| Tables | Are | Supported |
|--------|-----|-----------|
| Cell 1 | Cell 2 | Cell 3 |
```

#### Page Structure Best Practices

```markdown
# Clear, Descriptive Page Title

Brief introduction paragraph explaining what this page covers.

## Main Concepts

Explain the core concepts first.

### Detailed Subsection

Break down complex topics into digestible sections.

## Examples

Provide practical examples:

```bash
# Command examples
cargo run --release
```

## Next Steps

Guide readers to related pages or next actions.
```

### Organizing Your Content

#### Recommended Structure

```
docs/
├── config.yaml
├── entry.md              # Homepage content
├── introduction/         # High-level overview
│   ├── overview.md       # What is your project?
│   ├── quick-start.md    # This page
│   └── concepts.md       # Core concepts
├── guides/               # Step-by-step instructions
│   ├── installation.md
│   ├── configuration.md
│   ├── first-project.md
│   └── troubleshooting.md
├── reference/            # Detailed technical info
│   ├── api.md
│   ├── cli.md
│   └── configuration-reference.md
└── advanced/             # Advanced topics
    ├── customization.md
    ├── plugins.md
    └── deployment.md
```

#### Content Guidelines

1. **Start with user goals**: What does the reader want to accomplish?
2. **Use progressive disclosure**: Basic info first, details later
3. **Include examples**: Show, don't just tell
4. **Test your instructions**: Verify steps work as documented
5. **Update regularly**: Keep content current with your project

### Adding New Pages

1. **Create the markdown file:**
   ```bash
   # Create in appropriate section
   touch docs/guides/new-feature.md
   ```

2. **Add content with H1 title:**
   ```markdown
   # New Feature Guide
   
   This guide explains how to use the new feature.
   ```

3. **Update configuration:**
   ```bash
   # Auto-detect and add to navigation
   cargo run init-config
   
   # Or manually edit docs/config.yaml
   ```

4. **Rebuild documentation:**
   ```bash
   cargo run --release
   ```

## Theme Customization

### Built-in Themes

GlowDoc includes three professionally designed themes:

```yaml
# In docs/config.yaml
theme: default  # Clean, professional
theme: purple   # Purple accents
theme: vibrant  # Colorful, energetic
```

### Quick Theme Preview

```bash
# Try different themes quickly
sed -i 's/theme: .*/theme: purple/' docs/config.yaml && cargo run --release
sed -i 's/theme: .*/theme: vibrant/' docs/config.yaml && cargo run --release
sed -i 's/theme: .*/theme: default/' docs/config.yaml && cargo run --release
```

### Dark Mode

All themes include automatic dark mode:
- **System preference detection**: Respects user's OS setting
- **Manual toggle**: Click the theme button in header
- **Persistent choice**: Remembers user preference

## Development Workflow

### Efficient Development Loop

```bash
# 1. Edit your markdown files
vim docs/guides/new-page.md

# 2. Rebuild (takes ~1-3 seconds)
cargo run --release

# 3. Refresh browser (server keeps running)
# No need to restart python server
```

### File Watching (Optional)

For automatic rebuilds on file changes:

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch for changes and rebuild
cargo watch -x "run --release"
```

### Version Control Integration

```bash
# Initialize git repository
git init

# Add files (excluding generated content)
git add .
git commit -m "Initial documentation setup"

# .gitignore should include:
echo "target/" >> .gitignore
echo "index.html" >> .gitignore  # Generated file
```

## Testing Your Documentation

### Pre-Deployment Checklist

```bash
# 1. Build successfully
cargo run --release

# 2. Check file size (should be reasonable)
ls -lh index.html

# 3. Validate HTML (optional)
# Install html-validate: npm install -g html-validate
html-validate index.html

# 4. Test locally
python3 -m http.server 8000
```

### Manual Testing

Visit `http://localhost:8000` and verify:

- [ ] **Navigation**: All links work correctly
- [ ] **Search**: Finds content in titles and text
- [ ] **Themes**: Light/dark mode toggle works
- [ ] **Mobile**: Sidebar collapses, navigation works
- [ ] **Content**: All pages display properly
- [ ] **Links**: Internal and external links function
- [ ] **Performance**: Pages load quickly

### Accessibility Testing

```bash
# Install axe-core CLI (optional)
npm install -g @axe-core/cli

# Test accessibility
axe http://localhost:8000
```

## Common Issues and Solutions

### Build Problems

**"cargo: command not found"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**"No such file or directory: docs/config.yaml"**
```bash
# Generate configuration
cargo run init-config
```

**Build errors after editing config.yaml**
```bash
# Validate YAML syntax
python3 -c "import yaml; yaml.safe_load(open('docs/config.yaml'))"

# Check file references exist
ls docs/introduction/quick-start.md
```

### Server Issues

**"Address already in use"**
```bash
# Find and kill process using port 8000
lsof -ti:8000 | xargs kill

# Or use different port
python3 -m http.server 8080
```

**Browser shows "No such file or directory"**
```bash
# Ensure you're in the correct directory
ls index.html

# Rebuild if missing
cargo run --release
```

### Content Problems

**Page not appearing in navigation**
```bash
# Ensure file is referenced in config.yaml
grep -r "filename.md" docs/config.yaml

# Or regenerate config
cargo run init-config
```

**Search not finding content**
```bash
# Rebuild to update search index
cargo run --release

# Check file has H1 heading
head -5 docs/section/page.md
```

## Next Steps

Congratulations! You now have a working GlowDoc site. Here's what to explore next:

### Immediate Actions

1. **Add Your Content**
   - Replace sample content with your documentation
   - Update `docs/entry.md` with your project information
   - Add pages for your specific use cases

2. **Customize Appearance**
   - Try different themes in `config.yaml`
   - Explore advanced styling options
   - Add your logo or branding

3. **Test Thoroughly**
   - Verify all navigation works
   - Test search functionality
   - Check mobile responsiveness

### Advanced Features

1. **Learn Configuration Management**
   - Read the [Configuration Guide](../getting-started/configuration.md)
   - Understand navigation structure options
   - Explore CLI automation features

2. **Explore Customization**
   - Check out [Custom Styling](../customization/styling.md)
   - Learn about [Components](../customization/components.md)
   - Discover [Theming](../customization/theming.md) options

3. **Plan Deployment**
   - Review [Deployment Options](../advanced/deployment.md)
   - Set up automated builds
   - Choose your hosting platform

4. **Extend Functionality**
   - Explore [Plugins](../advanced/plugins.md)
   - Check the [API Reference](../advanced/api.md)
   - Consider custom integrations

### Community and Support

- **Documentation**: Continue with [First Steps](../getting-started/first-steps.md)
- **Examples**: Browse sample configurations and setups
- **Issues**: Report problems or request features
- **Contributions**: Help improve GlowDoc

### Pro Tips

1. **Keep It Simple**: Start with basic setup, add complexity gradually
2. **Test Early**: Preview changes frequently during development
3. **Version Control**: Commit documentation changes regularly
4. **User Focus**: Write for your audience, not yourself
5. **Iterate**: Improve documentation based on user feedback

You're now ready to create professional, beautiful documentation with GlowDoc. Happy documenting!