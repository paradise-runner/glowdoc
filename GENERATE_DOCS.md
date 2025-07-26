# Documentation Generation Prompt

You are an AI agent tasked with generating comprehensive documentation for a codebase. Create a `./docs` folder structure following modern documentation best practices.

## Task Overview

1. **Analyze the codebase** to understand its purpose, architecture, and key components
2. **Generate markdown documentation** organized in a logical hierarchy
3. **Create a config.yaml** file that defines navigation structure
4. **Follow the content ordering principles** outlined below

## Documentation Structure

Create documentation in `./docs/` with this recommended structure:

```
docs/
├── config.yaml              # Navigation configuration
├── entry.md                 # Homepage/landing content
├── introduction/            # Foundation concepts
│   ├── what-is-[project].md
│   └── quick-start.md
├── getting-started/         # Basic setup and first steps
│   ├── first-steps.md
│   └── setup/
│       ├── installation.md
│       └── configuration.md
├── [feature-sections]/      # Core functionality
├── customization/           # Advanced configuration
├── advanced/                # Complex topics
│   ├── api.md
│   ├── deployment.md
│   └── [advanced-topics].md
└── [assets]/                # Images, diagrams, etc.
```

## Content Ordering Principles

**Order sections from simple to complex:**

1. **Introduction** - High-level overview and quick start
2. **Getting Started** - Installation, setup, first steps
3. **Core Features** - Main functionality (name sections by feature/domain)
4. **Customization** - Configuration and theming
5. **Advanced** - Complex topics, API reference, deployment

**Within each section, order pages:**
- Concepts before implementation
- Simple before complex
- Common use cases before edge cases
- Tutorials before reference

## Config.yaml Format

Generate a `config.yaml` following this structure:

```yaml
title: [Project Name]
description: [Brief project description]
theme: default  # or purple, vibrant
navigation:
- title: Introduction
  id: introduction
  items:
  - title: What is [Project]?
    id: what-is-[project]
    file: introduction/what-is-[project].md
    items: []
  - title: Quick Start
    id: quick-start
    file: introduction/quick-start.md
    items: []
- title: Getting Started
  id: getting-started
  items:
  - title: First Steps
    id: first-steps
    file: getting-started/first-steps.md
    items: []
  - title: Setup
    id: setup
    file: null
    items:
    - title: Installation
      id: setup-installation
      file: getting-started/setup/installation.md
      items: []
    - title: Configuration
      id: setup-configuration
      file: getting-started/setup/configuration.md
      items: []
# ... continue with additional sections
social:
  github: [username/repo]  # Optional
```

## Content Guidelines

### entry.md (Homepage)
- Project overview and value proposition
- Key features and benefits
- Quick navigation to important sections

### Introduction Section
- **what-is-[project].md**: High-level explanation, use cases, target audience
- **quick-start.md**: Minimal working example, 5-minute setup

### Getting Started Section
- **first-steps.md**: Detailed walkthrough for new users
- **installation.md**: Comprehensive setup instructions
- **configuration.md**: Basic configuration options

### Core Feature Sections
- One section per major feature/domain
- Start with overview, then specific capabilities
- Include code examples and common patterns

### Advanced Section
- **api.md**: Complete API reference
- **deployment.md**: Production deployment strategies
- Complex configuration and edge cases

## Writing Standards

- Use clear, scannable headings (H1-H4)
- Include code examples with syntax highlighting
- Add diagrams/screenshots where helpful
- Write for your target audience's skill level
- Maintain consistent tone and terminology
- Include practical examples and real-world scenarios

## Execution Steps

1. **Explore the codebase** thoroughly
2. **Identify key concepts** and user journeys
3. **Create the folder structure** in `./docs/`
4. **Write entry.md** with project overview
5. **Generate introduction/** content
6. **Create getting-started/** documentation
7. **Document core features** in logical sections
8. **Add customization/** and **advanced/** content
9. **Generate config.yaml** with proper navigation hierarchy
10. **Review and refine** for clarity and completeness

Focus on creating documentation that helps users succeed with the project, starting from their first encounter through advanced usage.