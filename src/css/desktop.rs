pub fn generate_desktop_css() -> String {
    r#"        * {
            border-color: hsl(var(--border));
        }

        body {
            background-color: hsl(var(--background));
            color: hsl(var(--foreground));
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 0;
            transition: all 0.3s ease;
        }

        .content-section a, .homepage a {
            color: hsl(var(--link));
            text-decoration: none;
            transition: color 0.2s ease;
        }

        .content-section a:hover, .homepage a:hover {
            color: hsl(var(--link) / 0.8);
            text-decoration: underline;
        }

        .content-section a:visited, .homepage a:visited {
            color: hsl(var(--link) / 0.9);
        }

        .container {
            max-width: 95%;
            margin: 0 auto;
            padding: 0 2rem;
        }

        header {
            border-bottom: 1px solid hsl(var(--border));
            padding: 1rem 0;
            position: sticky;
            top: 0;
            background-color: hsl(var(--background) / 0.95);
            backdrop-filter: blur(10px);
            z-index: 50;
        }

        .header-content {
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .logo {
            font-size: 1.5rem;
            font-weight: 700;
            color: hsl(var(--primary));
            text-decoration: none;
        }

        .nav {
            display: flex;
            gap: 1rem;
            align-items: center;
        }

        .nav a {
            color: hsl(var(--muted-foreground));
            text-decoration: none;
            font-weight: 500;
            transition: color 0.2s;
        }

        .nav a:hover {
            color: hsl(var(--foreground));
        }

        .mobile-menu-toggle {
            display: none;
            background: none;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            padding: 0.5rem;
            cursor: pointer;
            color: hsl(var(--foreground));
            transition: all 0.2s;
        }

        .mobile-menu-toggle:hover {
            background-color: hsl(var(--accent));
        }

        .theme-toggle {
            background: none;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            padding: 0.5rem;
            cursor: pointer;
            color: hsl(var(--foreground));
            transition: all 0.2s;
        }

        .theme-toggle:hover {
            background-color: hsl(var(--accent));
        }

        .social-links {
            display: flex;
            gap: 0.5rem;
            align-items: center;
            margin-left: 0;
        }

        .social-link {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 2rem;
            height: 2rem;
            color: hsl(var(--muted-foreground));
            text-decoration: none;
            border-radius: 0.375rem;
            transition: all 0.2s ease;
        }

        .social-link:hover {
            color: hsl(var(--foreground));
            background-color: hsl(var(--accent));
        }

        .social-link svg {
            width: 16px;
            height: 16px;
        }

        .homepage {
            display: none;
            width: 100%;
        }

        .homepage.active {
            display: block;
        }

        .homepage-content {
            max-width: 800px;
            margin: 0 auto;
            padding: 4rem 2rem;
        }

        .homepage h1 {
            font-size: 3rem;
            font-weight: 800;
            margin-bottom: 1rem;
            background: linear-gradient(135deg, hsl(var(--primary)), hsl(var(--primary) / 0.7));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            text-align: center;
        }

        .homepage h2 {
            font-size: 1.75rem;
            font-weight: 600;
            margin-top: 3rem;
            margin-bottom: 1rem;
            color: hsl(var(--foreground));
        }

        .homepage p {
            font-size: 1.125rem;
            margin-bottom: 1.5rem;
            color: hsl(var(--muted-foreground));
            line-height: 1.7;
        }

        .homepage ul {
            margin-bottom: 1.5rem;
            padding-left: 0;
            list-style: none;
        }

        .homepage li {
            margin-bottom: 1rem;
            padding-left: 0;
            color: hsl(var(--muted-foreground));
            line-height: 1.6;
        }

        .homepage li:before {
            content: "✨";
            margin-right: 0.75rem;
        }

        .homepage pre {
            background-color: hsl(var(--muted));
            border-radius: var(--radius);
            padding: 1.5rem;
            overflow-x: auto;
            border: 1px solid hsl(var(--border));
            margin: 2rem 0;
        }

        .homepage code {
            background-color: hsl(var(--muted));
            padding: 0.2rem 0.4rem;
            border-radius: calc(var(--radius) - 2px);
            font-size: 0.875rem;
            border: 1px solid hsl(var(--border));
        }

        .homepage pre code {
            background: none;
            padding: 0;
            border: none;
            font-size: 0.875rem;
        }

        .homepage strong {
            color: hsl(var(--foreground));
            font-weight: 600;
        }

        .layout {
            display: none;
            min-height: calc(100vh - 80px);
        }

        .layout.active {
            display: flex;
        }

        .sidebar {
            width: 280px;
            background-color: hsl(var(--card));
            border-right: 1px solid hsl(var(--border));
            padding: 1rem 0;
            position: sticky;
            top: 80px;
            height: calc(100vh - 80px);
            overflow-y: auto;
            transition: transform 0.3s ease;
        }

        .sidebar.hidden {
            transform: translateX(-100%);
        }

        .sidebar-nav {
            padding: 0 1rem;
        }

        .search-container {
            position: relative;
            margin-bottom: 1.5rem;
        }

        .search-input {
            width: 100%;
            padding: 0.75rem 2.5rem 0.75rem 1rem;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            background-color: hsl(var(--background));
            color: hsl(var(--foreground));
            font-size: 0.875rem;
            transition: all 0.2s;
            box-sizing: border-box;
        }

        .search-input:focus {
            outline: none;
            border-color: hsl(var(--primary));
            box-shadow: 0 0 0 2px hsl(var(--primary) / 0.1);
        }

        .search-input::placeholder {
            color: hsl(var(--muted-foreground));
        }

        .search-icon {
            position: absolute;
            right: 0.75rem;
            top: 50%;
            transform: translateY(-50%);
            color: hsl(var(--muted-foreground));
            pointer-events: none;
        }

        .search-results {
            margin-bottom: 1.5rem;
        }

        .search-results-header {
            font-weight: 600;
            color: hsl(var(--foreground));
            margin-bottom: 0.75rem;
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 0.025em;
            border-bottom: 1px solid hsl(var(--border));
            padding-bottom: 0.5rem;
        }

        .search-result {
            padding: 0.75rem;
            margin-bottom: 0.5rem;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            cursor: pointer;
            transition: all 0.2s;
            background-color: hsl(var(--card));
        }

        .search-result:hover {
            background-color: hsl(var(--accent));
            border-color: hsl(var(--primary));
        }

        .search-result-title {
            font-weight: 600;
            color: hsl(var(--foreground));
            margin-bottom: 0.25rem;
            font-size: 0.875rem;
        }

        .search-result-section {
            color: hsl(var(--muted-foreground));
            font-size: 0.75rem;
            margin-bottom: 0.5rem;
        }

        .search-result-snippet {
            color: hsl(var(--muted-foreground));
            font-size: 0.8rem;
            line-height: 1.4;
            overflow: hidden;
            display: -webkit-box;
            -webkit-line-clamp: 2;
            -webkit-box-orient: vertical;
        }

        .search-result-snippet mark {
            background-color: hsl(var(--primary) / 0.2);
            color: hsl(var(--primary));
            padding: 0.1rem 0.2rem;
            border-radius: 2px;
        }

        .no-results {
            text-align: center;
            color: hsl(var(--muted-foreground));
            padding: 2rem;
            font-style: italic;
        }

        .nav-section {
            margin-bottom: 1.5rem;
        }

        .nav-section-title {
            font-weight: 600;
            color: hsl(var(--foreground));
            margin-bottom: 0.5rem;
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 0.025em;
            display: flex;
            align-items: center;
            justify-content: space-between;
            cursor: pointer;
            padding: 0.5rem;
            border-radius: var(--radius);
            transition: all 0.2s;
        }

        .nav-section-title:hover {
            background-color: hsl(var(--accent));
        }

        .nav-section-title.collapsed {
            margin-bottom: 0;
        }

        .nav-section-toggle {
            transition: transform 0.2s;
        }

        .nav-section-toggle.collapsed {
            transform: rotate(-90deg);
        }

        .nav-items {
            list-style: none;
            padding: 0;
            margin: 0;
            transition: all 0.3s ease;
        }

        .nav-items.collapsed {
            opacity: 0;
            max-height: 0;
            overflow: hidden;
        }

        .nav-item {
            margin-bottom: 0.25rem;
        }

        .nav-link {
            display: block;
            padding: 0.5rem 0.75rem;
            color: hsl(var(--muted-foreground));
            text-decoration: none;
            border-radius: var(--radius);
            transition: all 0.2s;
            font-size: 0.875rem;
            border-left: 2px solid transparent;
        }

        .nav-link:hover {
            background-color: hsl(var(--accent));
            color: hsl(var(--foreground));
            border-left-color: hsl(var(--primary));
        }

        .nav-link.active {
            background-color: hsl(var(--primary) / 0.1);
            color: hsl(var(--primary));
            border-left-color: hsl(var(--primary));
            font-weight: 500;
        }

        .nav-folder {
            margin-bottom: 0.25rem;
        }

        .nav-folder-title {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0.4rem 0.75rem;
            color: hsl(var(--muted-foreground));
            cursor: pointer;
            border-radius: var(--radius);
            transition: all 0.2s;
            font-size: 0.875rem;
            font-weight: 500;
            background-color: hsl(var(--muted) / 0.3);
        }

        .nav-folder-title:hover {
            background-color: hsl(var(--accent));
            color: hsl(var(--foreground));
        }

        .nav-folder-toggle {
            transition: transform 0.2s;
            margin-left: 0.5rem;
        }

        .nav-folder-toggle.collapsed {
            transform: rotate(-90deg);
        }

        .nav-nested-items {
            list-style: none;
            padding: 0;
            margin: 0.5rem 0 0 1rem;
            border-left: 1px solid hsl(var(--border));
            transition: all 0.3s ease;
        }

        .nav-nested-items.collapsed {
            opacity: 0;
            max-height: 0;
            overflow: hidden;
            margin-top: 0;
        }

        .nav-nested-items .nav-item {
            margin-bottom: 0.2rem;
        }

        .nav-nested-items .nav-link {
            margin-left: 0.5rem;
            padding: 0.4rem 0.5rem;
            font-size: 0.8rem;
            border-left-width: 1px;
        }

        .nav-nested-items .nav-folder-title {
            margin-left: 0.5rem;
            padding: 0.35rem 0.5rem;
            font-size: 0.8rem;
            background-color: hsl(var(--muted) / 0.2);
        }

        .content-wrapper {
            display: flex;
            max-width: none;
            width: 100%;
            gap: 0;
            margin-left: 1rem;
        }

        .content-area {
            flex: 1;
            max-width: none;
            min-width: 0;
            overflow-y: auto;
            height: 100%;
            padding-right: 310px;
        }

        .table-of-contents {
            width: 250px;
            flex-shrink: 0;
            position: fixed;
            top: calc(80px + 2rem);
            right: 0.5rem;
            height: fit-content;
            max-height: calc(100vh - 80px - 10rem);
            overflow-y: auto;
            background-color: hsl(var(--card));
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            padding: 1rem;
            z-index: 10;
        }

        .toc-header {
            margin-bottom: 1rem;
            padding-bottom: 0.5rem;
            border-bottom: 1px solid hsl(var(--border));
        }

        .toc-header h3 {
            margin: 0;
            font-size: 0.875rem;
            font-weight: 600;
            color: hsl(var(--foreground));
            text-transform: uppercase;
            letter-spacing: 0.025em;
        }

        .toc-nav {
            font-size: 0.875rem;
        }

        .toc-nav ul {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .toc-nav li {
            margin-bottom: 0.25rem;
        }

        .toc-nav a {
            display: block;
            padding: 0.25rem 0.5rem;
            color: hsl(var(--muted-foreground));
            text-decoration: none;
            border-radius: calc(var(--radius) - 2px);
            transition: all 0.2s;
            font-size: 0.75rem;
            line-height: 1.4;
            border-left: 2px solid transparent;
        }

        .toc-nav a:hover {
            background-color: hsl(var(--accent));
            color: hsl(var(--foreground));
            border-left-color: hsl(var(--primary) / 0.5);
        }

        .toc-nav a.active {
            background-color: hsl(var(--primary) / 0.1);
            color: hsl(var(--primary));
            border-left-color: hsl(var(--primary));
            font-weight: 500;
        }

        .toc-level-1 {
            margin-left: 0;
        }

        .toc-level-2 {
            margin-left: 0;
        }

        .toc-level-3 {
            margin-left: 1rem;
        }

        .toc-level-4 {
            margin-left: 1.5rem;
        }

        .toc-level-5 {
            margin-left: 2rem;
        }

        .toc-level-6 {
            margin-left: 2.5rem;
        }

        .toc-level-3 a,
        .toc-level-4 a,
        .toc-level-5 a,
        .toc-level-6 a {
            font-size: 0.7rem;
            padding: 0.2rem 0.4rem;
        }

        .main-content {
            flex: 1;
            padding: 0;
            overflow-y: auto;
            display: flex;
            justify-content: center;
        }

        .content-section {
            display: none;
            width: 100%;
        }

        .content-section.active {
            display: block;
        }

        .content-section h1 {
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 1rem;
            color: hsl(var(--foreground));
        }

        .content-section h2 {
            font-size: 1.75rem;
            font-weight: 600;
            margin-top: 2rem;
            margin-bottom: 1rem;
            color: hsl(var(--foreground));
        }

        .content-section p {
            margin-bottom: 1rem;
            color: hsl(var(--muted-foreground));
        }

        .content-section ul {
            margin-bottom: 1rem;
            padding-left: 1.5rem;
        }

        .content-section li {
            margin-bottom: 0.5rem;
            color: hsl(var(--muted-foreground));
        }

        .content-section pre {
            background-color: hsl(var(--muted));
            border-radius: var(--radius);
            padding: 1rem;
            overflow-x: auto;
            border: 1px solid hsl(var(--border));
            margin-bottom: 1rem;
        }

        .content-section code {
            background-color: hsl(var(--muted));
            padding: 0.2rem 0.4rem;
            border-radius: calc(var(--radius) - 2px);
            font-size: 0.875rem;
            border: 1px solid hsl(var(--border));
        }

        .content-section pre code {
            background: none;
            padding: 0;
            border: none;
            font-size: 0.875rem;
        }

        footer {
            border-top: 1px solid hsl(var(--border));
            padding: 1rem 0;
            text-align: center;
            color: hsl(var(--muted-foreground));
        }"#.to_string()
}