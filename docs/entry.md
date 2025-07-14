# ✨ Welcome to GlowDoc! ✨

Create stunning documentation sites with modern design principles. GlowDoc is a powerful, yet simple static site generator built with Rust that transforms your markdown files into beautiful, responsive documentation websites.

## Why Choose GlowDoc?

**✨ Beautiful by Default** - Clean, professional design with carefully crafted typography, spacing, and visual hierarchy that makes your content shine

**🌙 Smart Dark Mode** - Automatic dark mode detection with smooth transitions and persistent user preferences

**📱 Mobile-First Responsive** - Flawless experience across all devices with an adaptive sidebar and touch-friendly navigation

**⚡ Lightning Fast** - Minimal dependencies, optimized performance, and single-page application architecture for instant page loads

**♿ Accessibility First** - Built with WCAG guidelines in mind, featuring proper semantic markup, keyboard navigation, and screen reader support

**🎛️ Highly Customizable** - Easy theming system with CSS custom properties, flexible layouts, and extensible components

**🔧 Developer Friendly** - Simple markdown workflow, automatic config generation, and hot reload development experience

<img src="docs/diagonal_comparison.png" width=100%/>

## Key Features

### 📝 Markdown-Powered Content

Write your documentation in familiar GitHub-flavored markdown with support for:

- Tables and task lists
- Code syntax highlighting
- Footnotes and strikethrough

### 🗂️ Smart Organization

- Automatic navigation generation from folder structure
- Configurable page ordering and section management
- Search functionality across all content
- Breadcrumb navigation for easy orientation

### 🎯 Zero Configuration Required

Get started instantly with our intelligent config generator:

- Auto-detects your documentation structure
- Extracts page titles from markdown headers
- Interactive setup wizard for customization
- Command-line options for automation

### 😍 Beautiful Theme System

Professional styling that adapts to your needs:

- Modern CSS custom properties for easy customization
- Automatic light/dark mode with smooth transitions
- Carefully crafted typography and visual hierarchy
- Responsive design that looks stunning on every device
- Persistent theme preferences across sessions

## Wicked Fast & Efficient

GlowDoc is super fast, able to handle thousands of documents with ease.

### Performance Benchmark: 10,000 Documents

<table style="width: 100%; border-collapse: collapse; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 12px rgba(0,0,0,0.1);">
  <thead>
    <tr style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white;">
      <th style="padding: 16px; text-align: left; font-weight: 600;">Feature</th>
      <th style="padding: 16px; text-align: left; font-weight: 600;">GlowDoc</th>
      <th style="padding: 16px; text-align: left; font-weight: 600;">Astro Starlight</th>
    </tr>
  </thead>
  <tbody>
    <tr style="background-color: rgba(102, 126, 234, 0.05);">
      <td style="padding: 16px; font-weight: 600; border-bottom: 1px solid rgba(0,0,0,0.1);">Compile Time</td>
      <td style="padding: 16px; color: #10b981; font-weight: 500; border-bottom: 1px solid rgba(0,0,0,0.1);">1.32 seconds</td>
      <td style="padding: 16px; color: #ef4444; font-weight: 500; border-bottom: 1px solid rgba(0,0,0,0.1);">20+ minutes</td>
    </tr>
    <tr>
      <td style="padding: 16px; font-weight: 600; border-bottom: 1px solid rgba(0,0,0,0.1);">Output Size</td>
      <td style="padding: 16px; color: #10b981; font-weight: 500; border-bottom: 1px solid rgba(0,0,0,0.1);">17MB (gzipped)</td>
      <td style="padding: 16px; color: #ef4444; font-weight: 500; border-bottom: 1px solid rgba(0,0,0,0.1);">10+ GB</td>
    </tr>
    <tr style="background-color: rgba(102, 126, 234, 0.05);">
      <td style="padding: 16px; font-weight: 600;">Document Capacity</td>
      <td style="padding: 16px; color: #10b981; font-weight: 500;">Tens of thousands</td>
      <td style="padding: 16px; color: #ef4444; font-weight: 500;">Limited by build time/storage space</td>
    </tr>
  </tbody>
</table>

*Testing environment: MacBook Air M1, 16GB RAM. Results may vary based on hardware and document complexity.*

<br/>

<img src="docs/performance-chart.svg" width=100%/>


## Quick Start Guide

Ready to create amazing documentation? Here's how to get started in under 5 minutes:

```bash
# 1. Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Clone or create your documentation project
# (or use GlowDoc in your existing project)

# 3. Generate your configuration
cargo run init-config

# 4. Build your beautiful documentation site
cargo run --release

# 5. Preview locally
python3 -m http.server 8000
# Visit http://localhost:8000 to see your docs!
```

## Perfect For Every Documentation Need

### 📚 **Project Documentation**

Document your open source projects, internal tools, or enterprise software with professional-grade presentation

### 🔌 **API Documentation**

Create comprehensive API guides with code examples, endpoint references, and interactive exploration

### 📖 **User Guides & Tutorials**

Build step-by-step guides that help users master your product with clear, visual instructions

### 🧠 **Knowledge Bases**

Organize team knowledge, best practices, and institutional wisdom in an easily searchable format

### ✍️ **Technical Blogs**

Publish technical content with beautiful formatting, syntax highlighting, and professional presentation

### 📋 **Documentation Hubs**

Centralize multiple projects or teams' documentation in one cohesive, branded experience

## What Makes GlowDoc Special?

### 🏗️ **Single Binary Simplicity**

No complex build pipelines or dependency management - just one Rust binary that does everything you need.

### 🎨 **Design That Delights**

We obsess over the details so you don't have to. Every element is carefully designed for optimal readability and user experience.

### 🚀 **Performance Obsessed**

Your documentation loads instantly and stays responsive, even with hundreds of pages of content.

### 🔍 **Search-First Experience**

Built-in search helps users find exactly what they're looking for, when they need it.

## Our Philosophy

> **Great documentation should be invisible technology.**

We believe that the best documentation tools get out of your way and let your content shine. GlowDoc follows these core principles:

- **Content First**: Your words matter most - everything else should support them
- **Accessibility Always**: Documentation should be usable by everyone, regardless of ability or device
- **Performance Matters**: Fast load times and smooth interactions create better reading experiences
- **Beauty Serves Purpose**: Good design isn't decoration - it improves comprehension and usability

## Ready to Get Started?

🚀 **Jump right in** with our [Quick Start Guide](#quick-start) for a guided setup experience

⚙️ **Learn the details** in our [Installation Guide](#installation) for comprehensive setup instructions

🎨 **Make it yours** with our [Customization Guide](#theming) to match your brand and style

💡 **Need help?** Check out our [Configuration Guide](#configuration) for advanced setup options

---

_Ready to transform your documentation? Let's build something beautiful together!_ 🌟

<br/>

⚡️ More projects can be found at my [website](https://hec.works) 🌊 and the project is fully open source on [github](https://github.com/paradise-runner/glowdoc) 🤩

<br/>
<br/>
<center>Built with ♥️ in Fort Collins, CO</center>
