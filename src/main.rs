use std::fs;
use std::path::Path;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Options, html};

mod config_builder;
use config_builder::ConfigBuilder;

#[derive(Debug, Deserialize, Serialize)]
struct NavigationItem {
    title: String,
    id: String,
    file: Option<String>,
    #[serde(default)]
    items: Vec<NavigationItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NavigationSection {
    title: String,
    id: String,
    items: Vec<NavigationItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    title: String,
    description: String,
    navigation: Vec<NavigationSection>,
    #[serde(default = "default_theme")]
    theme: String,
    #[serde(default)]
    social: SocialLinks,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
struct SocialLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    instagram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mastodon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads: Option<String>,
}

fn default_theme() -> String {
    "default".to_string()
}

struct GlowDocBuilder {
    docs_path: String,
    config_path: String,
    entry_path: String,
    output_path: String,
}

impl GlowDocBuilder {
    fn new() -> Self {
        GlowDocBuilder {
            docs_path: "docs".to_string(),
            config_path: "docs/config.yaml".to_string(),
            entry_path: "docs/entry.md".to_string(),
            output_path: "index.html".to_string(),
        }
    }

    fn get_current_year(&self) -> i32 {
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let seconds = duration_since_epoch.as_secs();
        
        // Convert seconds to years (approximate)
        // 1970 + (seconds / seconds_per_year)
        // Using 365.25 days per year to account for leap years
        let seconds_per_year = 365.25 * 24.0 * 60.0 * 60.0;
        let years_since_1970 = (seconds as f64) / seconds_per_year;
        1970 + years_since_1970 as i32
    }

    fn load_config(&self) -> Result<Config, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(&self.config_path)?;
        let config: Config = serde_yaml::from_str(&config_content)?;
        Ok(config)
    }

    fn load_markdown_file(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let full_path = Path::new(&self.docs_path).join(file_path);
        let content = fs::read_to_string(full_path)?;
        
        // Set up markdown parser with GitHub-flavored markdown
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        Ok(html_output)
    }


    fn generate_sidebar(&self, navigation: &[NavigationSection]) -> String {
        let mut sidebar_html = String::new();
        
        // Add search input at the top of the sidebar
        sidebar_html.push_str(r#"
                <div class="search-container">
                    <input type="text" id="search-input" class="search-input" placeholder="Search pages..." oninput="performSearch()">
                    <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="11" cy="11" r="8"/>
                        <path d="m21 21-4.35-4.35"/>
                    </svg>
                </div>
                <div id="search-results" class="search-results" style="display: none;">
                    <div class="search-results-header">Search Results</div>
                    <div id="search-results-list"></div>
                </div>
                <div id="navigation-container">"#);
        
        for section in navigation {
            sidebar_html.push_str(&format!(
                "\n                <div class=\"nav-section\">\n                    <div class=\"nav-section-title\" onclick=\"toggleSection('{}')\">\n                        <span>{}</span>\n                        <svg class=\"nav-section-toggle\" width=\"12\" height=\"12\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\">\n                            <polyline points=\"6 9 12 15 18 9\"></polyline>\n                        </svg>\n                    </div>\n                    <ul class=\"nav-items\" id=\"{}-items\">",
                section.id, section.title, section.id
            ));
            
            for (index, item) in section.items.iter().enumerate() {
                let is_first = navigation.first().map(|s| s.id == section.id).unwrap_or(false) && index == 0 && item.file.is_some();
                let active_class = if is_first { " active" } else { "" };
                
                self.generate_nav_item(&mut sidebar_html, item, section, 0, active_class);
            }
            
            sidebar_html.push_str("\n                    </ul>\n                </div>");
        }
        
        sidebar_html.push_str("\n                </div>"); // Close navigation-container
        
        sidebar_html
    }

    fn generate_nav_item(&self, html: &mut String, item: &NavigationItem, section: &NavigationSection, depth: usize, active_class: &str) {
        self.generate_nav_item_with_path(html, item, section, depth, active_class, &section.id)
    }

    fn generate_nav_item_with_path(&self, html: &mut String, item: &NavigationItem, section: &NavigationSection, depth: usize, active_class: &str, path_prefix: &str) {
        let indent = "    ".repeat(depth + 6); // Base indentation + depth
        
        if item.file.is_some() {
            // This is a page item - use combined path for URL and content ID
            let combined_id = format!("{}/{}", path_prefix, item.id);
            html.push_str(&format!(
                "\n{}<li class=\"nav-item\">\n{}    <a href=\"#{}\" class=\"nav-link{}\" data-content-id=\"{}\" data-section-id=\"{}\">{}</a>\n{}</li>",
                indent, indent, combined_id, active_class, combined_id, section.id, item.title, indent
            ));
        } else if !item.items.is_empty() {
            // This is a nested folder
            let folder_path = format!("{}/{}", path_prefix, item.id);
            html.push_str(&format!(
                "\n{}<li class=\"nav-item nav-folder\">\n{}    <div class=\"nav-folder-title\" onclick=\"toggleNestedSection('{}')\">\n{}        <span>{}</span>\n{}        <svg class=\"nav-folder-toggle\" width=\"10\" height=\"10\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\">\n{}            <polyline points=\"6 9 12 15 18 9\"></polyline>\n{}        </svg>\n{}    </div>\n{}    <ul class=\"nav-nested-items\" id=\"{}-items\">",
                indent, indent, folder_path.replace("/", "-"), indent, item.title, indent, indent, indent, indent, indent, folder_path.replace("/", "-")
            ));
            
            for nested_item in &item.items {
                self.generate_nav_item_with_path(html, nested_item, section, depth + 1, "", &folder_path);
            }
            
            html.push_str(&format!("\n{}    </ul>\n{}</li>", indent, indent));
        }
    }

    fn generate_content(&self, navigation: &[NavigationSection]) -> Result<(String, String), Box<dyn std::error::Error>> {
        let mut content_html = String::new();
        let mut search_index = String::new();
        
        search_index.push_str("const searchIndex = {\n");
        
        for section in navigation {
            for (index, item) in section.items.iter().enumerate() {
                let is_first = navigation.first().map(|s| s.id == section.id).unwrap_or(false) && index == 0 && item.file.is_some();
                let active_class = if is_first { " active" } else { "" };
                
                self.process_content_item(item, section, &mut content_html, &mut search_index, active_class)?;
            }
        }
        
        // Remove trailing comma and close the object
        if search_index.ends_with(",\n") {
            search_index.pop();
            search_index.pop();
            search_index.push('\n');
        }
        search_index.push_str("};\n");
        
        Ok((content_html, search_index))
    }

    fn process_content_item(&self, item: &NavigationItem, section: &NavigationSection, content_html: &mut String, search_index: &mut String, active_class: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.process_content_item_with_path(item, section, content_html, search_index, active_class, &section.id)
    }

    fn process_content_item_with_path(&self, item: &NavigationItem, section: &NavigationSection, content_html: &mut String, search_index: &mut String, active_class: &str, path_prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(file) = &item.file {
            // This is a page item - use combined path for content ID
            let combined_id = format!("{}/{}", path_prefix, item.id);
            let full_path = Path::new(&self.docs_path).join(file);
            match fs::read_to_string(full_path) {
                Ok(raw_content) => {
                    // Convert markdown to HTML
                    let processed_content = match self.load_markdown_file(file) {
                        Ok(content) => content,
                        Err(e) => {
                            eprintln!("Error loading markdown file {}: {}", file, e);
                            format!("<p>Error loading content: {}</p>", file)
                        }
                    };
                    
                    content_html.push_str(&format!(
                        "\n            <section class=\"content-section{}\" id=\"{}\">\n                {}\n            </section>",
                        active_class, combined_id, processed_content
                    ));
                    
                    // Add to search index - escape quotes and newlines
                    let escaped_content = raw_content
                        .replace("\\", "\\\\")
                        .replace("\"", "\\\"")
                        .replace("\n", "\\n")
                        .replace("\r", "");
                    
                    search_index.push_str(&format!(
                        "    \"{}\": {{\n        \"title\": \"{}\",\n        \"section\": \"{}\",\n        \"content\": \"{}\"\n    }},\n",
                        combined_id, 
                        item.title.replace("\"", "\\\""), 
                        section.title.replace("\"", "\\\""), 
                        escaped_content
                    ));
                }
                Err(e) => {
                    eprintln!("Error loading raw markdown file {}: {}", file, e);
                    let combined_id = format!("{}/{}", path_prefix, item.id);
                    content_html.push_str(&format!(
                        "\n            <section class=\"content-section{}\" id=\"{}\">\n                <p>Error loading content: {}</p>\n            </section>",
                        active_class, combined_id, file
                    ));
                    
                    // Add minimal entry to search index
                    search_index.push_str(&format!(
                        "    \"{}\": {{\n        \"title\": \"{}\",\n        \"section\": \"{}\",\n        \"content\": \"Error loading content\"\n    }},\n",
                        combined_id, 
                        item.title.replace("\"", "\\\""), 
                        section.title.replace("\"", "\\\"")
                    ));
                }
            }
        }
        
        // Process nested items with extended path
        for nested_item in &item.items {
            let nested_path = format!("{}/{}", path_prefix, item.id);
            self.process_content_item_with_path(nested_item, section, content_html, search_index, "", &nested_path)?;
        }
        
        Ok(())
    }

    fn load_homepage(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !Path::new(&self.entry_path).exists() {
            return Err(format!(
                "Entry file not found: {}\n\nThe entry.md file is required for the homepage content.\nPlease create this file in your docs/ folder with your homepage markdown content.",
                self.entry_path
            ).into());
        }
        
        let content = fs::read_to_string(&self.entry_path)?;
        
        // Set up markdown parser with GitHub-flavored markdown
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        Ok(html_output)
    }

    fn generate_css(&self, theme: &str) -> String {
        let theme_vars = match theme {
            "purple" => r#"        :root {
            --background: 310 100% 98%;
            --foreground: 270 15% 15%;
            --card: 310 100% 98%;
            --card-foreground: 270 15% 15%;
            --popover: 310 100% 98%;
            --popover-foreground: 270 15% 15%;
            --primary: 270 91% 65%;
            --primary-foreground: 310 100% 98%;
            --secondary: 270 20% 92%;
            --secondary-foreground: 270 15% 15%;
            --muted: 270 20% 92%;
            --muted-foreground: 270 10% 55%;
            --accent: 270 20% 92%;
            --accent-foreground: 270 15% 15%;
            --destructive: 0 84.2% 60.2%;
            --destructive-foreground: 310 100% 98%;
            --border: 270 20% 88%;
            --input: 270 20% 88%;
            --ring: 270 91% 65%;
            --link: 50 100% 40%;
            --radius: 0.5rem;
        }

        [data-theme="dark"] {
            --background: 270 20% 12%;
            --foreground: 310 40% 92%;
            --card: 270 20% 12%;
            --card-foreground: 310 40% 92%;
            --popover: 270 20% 12%;
            --popover-foreground: 310 40% 92%;
            --primary: 270 91% 75%;
            --primary-foreground: 270 20% 12%;
            --secondary: 270 15% 20%;
            --secondary-foreground: 310 40% 92%;
            --muted: 270 15% 20%;
            --muted-foreground: 270 10% 65%;
            --accent: 270 15% 20%;
            --accent-foreground: 310 40% 92%;
            --destructive: 0 62.8% 50%;
            --destructive-foreground: 310 40% 92%;
            --border: 270 15% 20%;
            --input: 270 15% 20%;
            --ring: 270 91% 75%;
            --link: 50 100% 60%;
        }"#,
            "vibrant" => r#"        :root {
            --background: 45 100% 92%;
            --foreground: 220 30% 25%;
            --card: 50 80% 88%;
            --card-foreground: 220 30% 25%;
            --popover: 50 80% 88%;
            --popover-foreground: 220 30% 25%;
            --primary: 200 100% 50%;
            --primary-foreground: 0 0% 98%;
            --secondary: 120 40% 75%;
            --secondary-foreground: 220 30% 25%;
            --muted: 60 60% 85%;
            --muted-foreground: 220 20% 40%;
            --accent: 330 80% 70%;
            --accent-foreground: 0 0% 98%;
            --destructive: 10 80% 60%;
            --destructive-foreground: 0 0% 98%;
            --border: 40 50% 80%;
            --input: 40 50% 80%;
            --ring: 200 100% 50%;
            --link: 320 80% 50%;
            --radius: 0.5rem;
        }

        [data-theme="dark"] {
            --background: 220 25% 20%;
            --foreground: 45 85% 85%;
            --card: 215 20% 25%;
            --card-foreground: 45 85% 85%;
            --popover: 215 20% 25%;
            --popover-foreground: 45 85% 85%;
            --primary: 200 90% 65%;
            --primary-foreground: 220 25% 20%;
            --secondary: 120 30% 45%;
            --secondary-foreground: 45 85% 85%;
            --muted: 210 15% 30%;
            --muted-foreground: 45 40% 70%;
            --accent: 330 70% 60%;
            --accent-foreground: 220 25% 20%;
            --destructive: 10 70% 55%;
            --destructive-foreground: 45 85% 85%;
            --border: 210 20% 35%;
            --input: 210 20% 35%;
            --ring: 200 90% 65%;
            --link: 320 80% 60%;
        }"#,
            _ => r#"        :root {
            --background: 0 0% 100%;
            --foreground: 222.2 84% 4.9%;
            --card: 0 0% 100%;
            --card-foreground: 222.2 84% 4.9%;
            --popover: 0 0% 100%;
            --popover-foreground: 222.2 84% 4.9%;
            --primary: 222.2 47.4% 11.2%;
            --primary-foreground: 210 40% 98%;
            --secondary: 210 40% 96%;
            --secondary-foreground: 222.2 47.4% 11.2%;
            --muted: 210 40% 96%;
            --muted-foreground: 215.4 16.3% 46.9%;
            --accent: 210 40% 96%;
            --accent-foreground: 222.2 47.4% 11.2%;
            --destructive: 0 84.2% 60.2%;
            --destructive-foreground: 210 40% 98%;
            --border: 214.3 31.8% 91.4%;
            --input: 214.3 31.8% 91.4%;
            --ring: 222.2 84% 4.9%;
            --link: 120 100% 35%;
            --radius: 0.5rem;
        }

        [data-theme="dark"] {
            --background: 222.2 84% 4.9%;
            --foreground: 210 40% 98%;
            --card: 222.2 84% 4.9%;
            --card-foreground: 210 40% 98%;
            --popover: 222.2 84% 4.9%;
            --popover-foreground: 210 40% 98%;
            --primary: 210 40% 98%;
            --primary-foreground: 222.2 47.4% 11.2%;
            --secondary: 217.2 32.6% 17.5%;
            --secondary-foreground: 210 40% 98%;
            --muted: 217.2 32.6% 17.5%;
            --muted-foreground: 215 20.2% 65.1%;
            --accent: 217.2 32.6% 17.5%;
            --accent-foreground: 210 40% 98%;
            --destructive: 0 62.8% 30.6%;
            --destructive-foreground: 210 40% 98%;
            --border: 217.2 32.6% 17.5%;
            --input: 217.2 32.6% 17.5%;
            --ring: 212.7 26.8% 83.9%;
            --link: 120 100% 50%;
        }"#,
        };

        format!("{}

        * {{
            border-color: hsl(var(--border));
        }}

        body {{
            background-color: hsl(var(--background));
            color: hsl(var(--foreground));
            font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 0;
            transition: all 0.3s ease;
        }}

        .content-section a, .homepage a {{
            color: hsl(var(--link));
            text-decoration: none;
            transition: color 0.2s ease;
        }}

        .content-section a:hover, .homepage a:hover {{
            color: hsl(var(--link) / 0.8);
            text-decoration: underline;
        }}

        .content-section a:visited, .homepage a:visited {{
            color: hsl(var(--link) / 0.9);
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 1rem;
        }}

        header {{
            border-bottom: 1px solid hsl(var(--border));
            padding: 1rem 0;
            position: sticky;
            top: 0;
            background-color: hsl(var(--background) / 0.95);
            backdrop-filter: blur(10px);
            z-index: 50;
        }}

        .header-content {{
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}

        .logo {{
            font-size: 1.5rem;
            font-weight: 700;
            color: hsl(var(--primary));
            text-decoration: none;
        }}

        .nav {{
            display: flex;
            gap: 2rem;
            align-items: center;
        }}

        .nav a {{
            color: hsl(var(--muted-foreground));
            text-decoration: none;
            font-weight: 500;
            transition: color 0.2s;
        }}

        .nav a:hover {{
            color: hsl(var(--foreground));
        }}

        .mobile-menu-toggle {{
            display: none;
            background: none;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            padding: 0.5rem;
            cursor: pointer;
            color: hsl(var(--foreground));
            transition: all 0.2s;
        }}

        .mobile-menu-toggle:hover {{
            background-color: hsl(var(--accent));
        }}

        .theme-toggle {{
            background: none;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            padding: 0.5rem;
            cursor: pointer;
            color: hsl(var(--foreground));
            transition: all 0.2s;
        }}

        .theme-toggle:hover {{
            background-color: hsl(var(--accent));
        }}

        .social-links {{
            display: flex;
            gap: 0.5rem;
            align-items: center;
            margin-left: 0;
        }}

        .social-link {{
            display: flex;
            align-items: center;
            justify-content: center;
            width: 2rem;
            height: 2rem;
            color: hsl(var(--muted-foreground));
            text-decoration: none;
            border-radius: 0.375rem;
            transition: all 0.2s ease;
        }}

        .social-link:hover {{
            color: hsl(var(--foreground));
            background-color: hsl(var(--accent));
        }}

        .social-link svg {{
            width: 16px;
            height: 16px;
        }}

        .homepage {{
            display: none;
            width: 100%;
        }}

        .homepage.active {{
            display: block;
        }}

        .homepage-content {{
            max-width: 800px;
            margin: 0 auto;
            padding: 4rem 2rem;
        }}

        .homepage h1 {{
            font-size: 3rem;
            font-weight: 800;
            margin-bottom: 1rem;
            background: linear-gradient(135deg, hsl(var(--primary)), hsl(var(--primary) / 0.7));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            text-align: center;
        }}

        .homepage h2 {{
            font-size: 1.75rem;
            font-weight: 600;
            margin-top: 3rem;
            margin-bottom: 1rem;
            color: hsl(var(--foreground));
        }}

        .homepage p {{
            font-size: 1.125rem;
            margin-bottom: 1.5rem;
            color: hsl(var(--muted-foreground));
            line-height: 1.7;
        }}

        .homepage ul {{
            margin-bottom: 1.5rem;
            padding-left: 0;
            list-style: none;
        }}

        .homepage li {{
            margin-bottom: 1rem;
            padding-left: 0;
            color: hsl(var(--muted-foreground));
            line-height: 1.6;
        }}

        .homepage li:before {{
            content: \"✨\";
            margin-right: 0.75rem;
        }}

        .homepage pre {{
            background-color: hsl(var(--muted));
            border-radius: var(--radius);
            padding: 1.5rem;
            overflow-x: auto;
            border: 1px solid hsl(var(--border));
            margin: 2rem 0;
        }}

        .homepage code {{
            background-color: hsl(var(--muted));
            padding: 0.2rem 0.4rem;
            border-radius: calc(var(--radius) - 2px);
            font-size: 0.875rem;
            border: 1px solid hsl(var(--border));
        }}

        .homepage pre code {{
            background: none;
            padding: 0;
            border: none;
            font-size: 0.875rem;
        }}

        .homepage strong {{
            color: hsl(var(--foreground));
            font-weight: 600;
        }}

        .layout {{
            display: none;
            min-height: calc(100vh - 80px);
        }}

        .layout.active {{
            display: flex;
        }}

        .sidebar {{
            width: 280px;
            background-color: hsl(var(--card));
            border-right: 1px solid hsl(var(--border));
            padding: 1rem 0;
            position: sticky;
            top: 80px;
            height: calc(100vh - 80px);
            overflow-y: auto;
            transition: transform 0.3s ease;
        }}

        .sidebar.hidden {{
            transform: translateX(-100%);
        }}

        .sidebar-nav {{
            padding: 0 1rem;
        }}

        .search-container {{
            position: relative;
            margin-bottom: 1.5rem;
        }}

        .search-input {{
            width: 100%;
            padding: 0.75rem 2.5rem 0.75rem 1rem;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            background-color: hsl(var(--background));
            color: hsl(var(--foreground));
            font-size: 0.875rem;
            transition: all 0.2s;
            box-sizing: border-box;
        }}

        .search-input:focus {{
            outline: none;
            border-color: hsl(var(--primary));
            box-shadow: 0 0 0 2px hsl(var(--primary) / 0.1);
        }}

        .search-input::placeholder {{
            color: hsl(var(--muted-foreground));
        }}

        .search-icon {{
            position: absolute;
            right: 0.75rem;
            top: 50%;
            transform: translateY(-50%);
            color: hsl(var(--muted-foreground));
            pointer-events: none;
        }}

        .search-results {{
            margin-bottom: 1.5rem;
        }}

        .search-results-header {{
            font-weight: 600;
            color: hsl(var(--foreground));
            margin-bottom: 0.75rem;
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 0.025em;
            border-bottom: 1px solid hsl(var(--border));
            padding-bottom: 0.5rem;
        }}

        .search-result {{
            padding: 0.75rem;
            margin-bottom: 0.5rem;
            border: 1px solid hsl(var(--border));
            border-radius: var(--radius);
            cursor: pointer;
            transition: all 0.2s;
            background-color: hsl(var(--card));
        }}

        .search-result:hover {{
            background-color: hsl(var(--accent));
            border-color: hsl(var(--primary));
        }}

        .search-result-title {{
            font-weight: 600;
            color: hsl(var(--foreground));
            margin-bottom: 0.25rem;
            font-size: 0.875rem;
        }}

        .search-result-section {{
            color: hsl(var(--muted-foreground));
            font-size: 0.75rem;
            margin-bottom: 0.5rem;
        }}

        .search-result-snippet {{
            color: hsl(var(--muted-foreground));
            font-size: 0.8rem;
            line-height: 1.4;
            overflow: hidden;
            display: -webkit-box;
            -webkit-line-clamp: 2;
            -webkit-box-orient: vertical;
        }}

        .search-result-snippet mark {{
            background-color: hsl(var(--primary) / 0.2);
            color: hsl(var(--primary));
            padding: 0.1rem 0.2rem;
            border-radius: 2px;
        }}

        .no-results {{
            text-align: center;
            color: hsl(var(--muted-foreground));
            padding: 2rem;
            font-style: italic;
        }}

        .nav-section {{
            margin-bottom: 1.5rem;
        }}

        .nav-section-title {{
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
        }}

        .nav-section-title:hover {{
            background-color: hsl(var(--accent));
        }}

        .nav-section-title.collapsed {{
            margin-bottom: 0;
        }}

        .nav-section-toggle {{
            transition: transform 0.2s;
        }}

        .nav-section-toggle.collapsed {{
            transform: rotate(-90deg);
        }}

        .nav-items {{
            list-style: none;
            padding: 0;
            margin: 0;
            transition: all 0.3s ease;
        }}

        .nav-items.collapsed {{
            opacity: 0;
            max-height: 0;
            overflow: hidden;
        }}

        .nav-item {{
            margin-bottom: 0.25rem;
        }}

        .nav-link {{
            display: block;
            padding: 0.5rem 0.75rem;
            color: hsl(var(--muted-foreground));
            text-decoration: none;
            border-radius: var(--radius);
            transition: all 0.2s;
            font-size: 0.875rem;
            border-left: 2px solid transparent;
        }}

        .nav-link:hover {{
            background-color: hsl(var(--accent));
            color: hsl(var(--foreground));
            border-left-color: hsl(var(--primary));
        }}

        .nav-link.active {{
            background-color: hsl(var(--primary) / 0.1);
            color: hsl(var(--primary));
            border-left-color: hsl(var(--primary));
            font-weight: 500;
        }}

        .nav-folder {{
            margin-bottom: 0.25rem;
        }}

        .nav-folder-title {{
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
        }}

        .nav-folder-title:hover {{
            background-color: hsl(var(--accent));
            color: hsl(var(--foreground));
        }}

        .nav-folder-toggle {{
            transition: transform 0.2s;
            margin-left: 0.5rem;
        }}

        .nav-folder-toggle.collapsed {{
            transform: rotate(-90deg);
        }}

        .nav-nested-items {{
            list-style: none;
            padding: 0;
            margin: 0.5rem 0 0 1rem;
            border-left: 1px solid hsl(var(--border));
            transition: all 0.3s ease;
        }}

        .nav-nested-items.collapsed {{
            opacity: 0;
            max-height: 0;
            overflow: hidden;
            margin-top: 0;
        }}

        .nav-nested-items .nav-item {{
            margin-bottom: 0.2rem;
        }}

        .nav-nested-items .nav-link {{
            margin-left: 0.5rem;
            padding: 0.4rem 0.5rem;
            font-size: 0.8rem;
            border-left-width: 1px;
        }}

        .nav-nested-items .nav-folder-title {{
            margin-left: 0.5rem;
            padding: 0.35rem 0.5rem;
            font-size: 0.8rem;
            background-color: hsl(var(--muted) / 0.2);
        }}

        .main-content {{
            flex: 1;
            padding: 2rem;
            overflow-y: auto;
            display: flex;
            justify-content: center;
        }}

        .content-section {{
            display: none;
            max-width: 800px;
            width: 100%;
        }}

        .content-section.active {{
            display: block;
        }}

        .content-section h1 {{
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 1rem;
            color: hsl(var(--foreground));
        }}

        .content-section h2 {{
            font-size: 1.75rem;
            font-weight: 600;
            margin-top: 2rem;
            margin-bottom: 1rem;
            color: hsl(var(--foreground));
        }}

        .content-section p {{
            margin-bottom: 1rem;
            color: hsl(var(--muted-foreground));
        }}

        .content-section ul {{
            margin-bottom: 1rem;
            padding-left: 1.5rem;
        }}

        .content-section li {{
            margin-bottom: 0.5rem;
            color: hsl(var(--muted-foreground));
        }}

        .content-section pre {{
            background-color: hsl(var(--muted));
            border-radius: var(--radius);
            padding: 1rem;
            overflow-x: auto;
            border: 1px solid hsl(var(--border));
            margin-bottom: 1rem;
        }}

        .content-section code {{
            background-color: hsl(var(--muted));
            padding: 0.2rem 0.4rem;
            border-radius: calc(var(--radius) - 2px);
            font-size: 0.875rem;
            border: 1px solid hsl(var(--border));
        }}

        .content-section pre code {{
            background: none;
            padding: 0;
            border: none;
            font-size: 0.875rem;
        }}

        footer {{
            border-top: 1px solid hsl(var(--border));
            padding: 2rem 0;
            text-align: center;
            color: hsl(var(--muted-foreground));
        }}

        @media (max-width: 768px) {{
            .nav {{
                gap: 1rem;
            }}

            .mobile-menu-toggle {{
                display: block;
            }}

            .layout {{
                flex-direction: column;
            }}

            .sidebar {{
                position: fixed;
                top: 80px;
                left: 0;
                width: 100%;
                height: calc(100vh - 80px);
                z-index: 40;
                background-color: hsl(var(--background));
                border-right: none;
                border-bottom: 1px solid hsl(var(--border));
                transform: translateX(-100%);
            }}

            .sidebar.visible {{
                transform: translateX(0);
            }}

            .main-content {{
                padding: 1rem;
            }}

            .social-links {{
                margin-left: 0;
                gap: 0.25rem;
            }}

            .social-link {{
                width: 1.75rem;
                height: 1.75rem;
            }}

            .social-link svg {{
                width: 14px;
                height: 14px;
            }}
        }}", theme_vars)
    }

    fn generate_javascript(&self) -> &'static str {
        r#"        function toggleTheme() {
            const html = document.documentElement;
            const currentTheme = html.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            
            html.setAttribute('data-theme', newTheme);
            localStorage.setItem('theme', newTheme);
        }

        function toggleSidebar() {
            const sidebar = document.getElementById('sidebar');
            sidebar.classList.toggle('visible');
        }

        function toggleSection(sectionId) {
            const items = document.getElementById(sectionId + '-items');
            const toggle = event.target.closest('.nav-section-title').querySelector('.nav-section-toggle');
            
            items.classList.toggle('collapsed');
            toggle.classList.toggle('collapsed');
            event.target.closest('.nav-section-title').classList.toggle('collapsed');
        }

        function toggleNestedSection(sectionId) {
            const items = document.getElementById(sectionId + '-items');
            const toggle = event.target.closest('.nav-folder-title').querySelector('.nav-folder-toggle');
            
            items.classList.toggle('collapsed');
            toggle.classList.toggle('collapsed');
        }

        function showHomepage() {
            document.getElementById('homepage').classList.add('active');
            document.getElementById('docs-layout').classList.remove('active');
            
            // Update URL to homepage
            history.pushState({ page: 'homepage' }, '', window.location.pathname);
        }

        function showDocs() {
            document.getElementById('homepage').classList.remove('active');
            document.getElementById('docs-layout').classList.add('active');
        }

        function showContent(contentId, updateUrl = true) {
            console.log('showContent called with contentId:', contentId);
            
            // Switch to docs view first
            showDocs();
            
            // Hide all content sections
            const allSections = document.querySelectorAll('.content-section');
            console.log('Found sections:', allSections.length);
            allSections.forEach(section => {
                section.classList.remove('active');
            });
            
            // Show selected content using the combined ID
            const targetContent = document.getElementById(contentId);
            console.log('Target content element:', targetContent);
            if (targetContent) {
                targetContent.classList.add('active');
                console.log('Successfully activated content:', contentId);
                
                // Update URL if requested
                if (updateUrl) {
                    const newUrl = window.location.pathname + '#' + contentId;
                    history.pushState({ contentId: contentId }, '', newUrl);
                }
            } else {
                console.error('Content not found for ID:', contentId);
                // List all available content sections for debugging
                const allContentIds = Array.from(allSections).map(s => s.id);
                console.log('Available content IDs:', allContentIds);
            }
            
            // Update active nav link
            document.querySelectorAll('.nav-link').forEach(link => {
                link.classList.remove('active');
            });
            
            // Find and activate the correct nav link using the combined content ID
            const navLinks = document.querySelectorAll('.nav-link');
            for (const link of navLinks) {
                if (link.getAttribute('data-content-id') === contentId) {
                    link.classList.add('active');
                    break;
                }
            }
            
            // Close sidebar on mobile
            if (window.innerWidth <= 768) {
                document.getElementById('sidebar').classList.remove('visible');
            }
            
        }


        // Load saved theme or default to light mode
        const savedTheme = localStorage.getItem('theme') || 'light';
        document.documentElement.setAttribute('data-theme', savedTheme);

        // Respect system preference if no saved theme
        if (!localStorage.getItem('theme')) {
            const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            if (prefersDark) {
                document.documentElement.setAttribute('data-theme', 'dark');
            }
        }

        function performSearch() {
            const searchInput = document.getElementById('search-input');
            const searchTerm = searchInput.value.toLowerCase().trim();
            const searchResults = document.getElementById('search-results');
            const searchResultsList = document.getElementById('search-results-list');
            const navigationContainer = document.getElementById('navigation-container');
            
            if (searchTerm === '') {
                // Show navigation, hide search results
                searchResults.style.display = 'none';
                navigationContainer.style.display = 'block';
                return;
            }
            
            // Hide navigation, show search results
            navigationContainer.style.display = 'none';
            searchResults.style.display = 'block';
            
            // Search through the index
            const results = [];
            
            for (const [pageId, pageData] of Object.entries(searchIndex)) {
                const titleMatch = pageData.title.toLowerCase().includes(searchTerm);
                const sectionMatch = pageData.section.toLowerCase().includes(searchTerm);
                const contentMatch = pageData.content.toLowerCase().includes(searchTerm);
                
                if (titleMatch || sectionMatch || contentMatch) {
                    // Find context around the match in content
                    let snippet = '';
                    if (contentMatch) {
                        const contentLower = pageData.content.toLowerCase();
                        const matchIndex = contentLower.indexOf(searchTerm);
                        const start = Math.max(0, matchIndex - 60);
                        const end = Math.min(pageData.content.length, matchIndex + searchTerm.length + 60);
                        snippet = pageData.content.substring(start, end);
                        
                        // Add ellipsis if we're in the middle of the content
                        if (start > 0) snippet = '...' + snippet;
                        if (end < pageData.content.length) snippet = snippet + '...';
                        
                        // Highlight the search term
                        const regex = new RegExp(`(${searchTerm})`, 'gi');
                        snippet = snippet.replace(regex, '<mark>$1</mark>');
                    }
                    
                    results.push({
                        id: pageId,
                        title: pageData.title,
                        section: pageData.section,
                        snippet: snippet,
                        titleMatch: titleMatch,
                        sectionMatch: sectionMatch,
                        contentMatch: contentMatch
                    });
                }
            }
            
            // Sort results by relevance (title matches first, then section matches, then content matches)
            results.sort((a, b) => {
                if (a.titleMatch && !b.titleMatch) return -1;
                if (!a.titleMatch && b.titleMatch) return 1;
                if (a.sectionMatch && !b.sectionMatch) return -1;
                if (!a.sectionMatch && b.sectionMatch) return 1;
                return 0;
            });
            
            // Display results
            if (results.length === 0) {
                searchResultsList.innerHTML = '<div class="no-results">No results found</div>';
            } else {
                searchResultsList.innerHTML = results.map(result => `
                    <div class="search-result" onclick="showContentFromSearch('${result.id}')">
                        <div class="search-result-title">${result.title}</div>
                        <div class="search-result-section">${result.section}</div>
                        ${result.snippet ? `<div class="search-result-snippet">${result.snippet}</div>` : ''}
                    </div>
                `).join('');
            }
        }
        
        function showContentFromSearch(contentId) {
            // Clear search and show navigation
            document.getElementById('search-input').value = '';
            document.getElementById('search-results').style.display = 'none';
            document.getElementById('navigation-container').style.display = 'block';
            
            // Show the content directly using the combined ID
            showContent(contentId);
        }

        // Handle browser back/forward navigation
        window.addEventListener('popstate', function(event) {
            if (event.state && event.state.contentId) {
                // Show content without updating URL (to avoid infinite loop)
                showContent(event.state.contentId, false);
            } else if (event.state && event.state.page === 'homepage') {
                showHomepage();
            } else {
                // Check URL hash for content ID
                loadFromUrl();
            }
        });

        // Load content based on URL on page load
        function loadFromUrl() {
            const hash = window.location.hash.substring(1); // Remove the # symbol
            
            if (hash) {
                // Check if the content exists with the provided hash
                if (document.getElementById(hash)) {
                    // Show the content without updating URL (it's already correct)
                    showContent(hash, false);
                    return;
                }
            }
            
            // Check if we're on homepage by checking if there's no hash and no docs content
            if (!hash) {
                // Show homepage and update URL
                history.replaceState({ page: 'homepage' }, '', window.location.pathname);
            }
        }

        // Handle navigation link clicks
        document.addEventListener('click', function(event) {
            const navLink = event.target.closest('.nav-link[data-content-id]');
            if (navLink) {
                event.preventDefault();
                const contentId = navLink.getAttribute('data-content-id');
                showContent(contentId);
            }
        });

        // Load content on initial page load
        document.addEventListener('DOMContentLoaded', function() {
            loadFromUrl();
        });

        // Close sidebar when clicking outside on mobile
        document.addEventListener('click', function(event) {
            const sidebar = document.getElementById('sidebar');
            const mobileToggle = document.querySelector('.mobile-menu-toggle');
            
            if (window.innerWidth <= 768 && 
                sidebar.classList.contains('visible') && 
                !sidebar.contains(event.target) && 
                !mobileToggle.contains(event.target)) {
                sidebar.classList.remove('visible');
            }
        });"#
    }

    fn generate_social_links_html(&self, social: &SocialLinks) -> String {
        let mut social_html = String::new();
        
        if social.instagram.is_some() || social.github.is_some() || social.mastodon.is_some() || social.threads.is_some() {
            social_html.push_str(r#"<div class="social-links">"#);
            
            if let Some(instagram) = &social.instagram {
                social_html.push_str(&format!(r#"
                    <a href="https://instagram.com/{}" class="social-link" target="_blank" rel="noopener noreferrer" aria-label="Instagram">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <rect x="2" y="2" width="20" height="20" rx="5" ry="5"/>
                            <path d="m16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"/>
                            <line x1="17.5" y1="6.5" x2="17.51" y2="6.5"/>
                        </svg>
                    </a>"#, instagram));
            }
            
            if let Some(github) = &social.github {
                social_html.push_str(&format!(r#"
                    <a href="https://github.com/{}" class="social-link" target="_blank" rel="noopener noreferrer" aria-label="GitHub">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/>
                        </svg>
                    </a>"#, github));
            }
            
            if let Some(mastodon) = &social.mastodon {
                social_html.push_str(&format!(r#"
                    <a href="{}" class="social-link" target="_blank" rel="noopener noreferrer" aria-label="Mastodon">
                        <svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M21.327 8.566c0-4.339-2.843-5.61-2.843-5.61-1.433-.658-3.894-.935-6.451-.956h-.063c-2.557.021-5.016.298-6.45.956 0 0-2.843 1.272-2.843 5.61 0 .993-.019 2.181.012 3.441.103 4.243.778 8.425 4.701 9.463 1.809.479 3.362.579 4.612.51 2.268-.126 3.541-.809 3.541-.809l-.075-1.646s-1.621.513-3.441.449c-1.804-.062-3.707-.194-3.999-2.409a4.523 4.523 0 0 1-.04-.621s1.77.433 4.014.536c1.372.063 2.658-.08 3.965-.236 2.506-.299 4.688-1.843 4.962-3.254.434-2.223.398-5.424.398-5.424zm-3.353 5.59h-2.081V9.057c0-1.075-.452-1.62-1.357-1.62-1 0-1.501.647-1.501 1.927v2.791h-2.069V9.364c0-1.28-.501-1.927-1.502-1.927-.905 0-1.357.546-1.357 1.62v5.099H6.026V8.903c0-1.074.273-1.927.823-2.558.566-.631 1.307-.955 2.228-.955 1.065 0 1.872.409 2.405 1.228l.518.869.519-.869c.533-.819 1.34-1.228 2.405-1.228.92 0 1.662.324 2.228.955.549.631.822 1.484.822 2.558v5.253z"/>
                        </svg>
                    </a>"#, mastodon));
            }
            
            if let Some(threads) = &social.threads {
                social_html.push_str(&format!(r#"
                    <a href="https://threads.net/@{}" class="social-link" target="_blank" rel="noopener noreferrer" aria-label="Threads">
                        <svg viewBox="0 0 192 192" fill="currentColor">
                            <path d="M141.537 88.9883C140.71 88.5919 139.87 88.2104 139.019 87.8451C137.537 60.5382 122.616 44.905 97.5619 44.745C97.4484 44.7443 97.3355 44.7443 97.222 44.7443C82.2364 44.7443 69.7731 51.1409 62.102 62.7807L75.881 72.2328C81.6116 63.5383 90.6052 61.6848 97.2286 61.6848C97.3051 61.6848 97.3819 61.6848 97.4576 61.6855C105.707 61.7381 111.932 64.1366 115.961 68.814C118.893 72.2193 120.854 76.925 121.825 82.8638C114.511 81.6207 106.601 81.2385 98.145 81.7233C74.3247 83.0954 59.0111 96.9879 60.0396 116.292C60.5615 126.084 65.4397 134.508 73.775 140.011C80.8224 144.663 89.899 146.938 99.3323 146.423C111.79 145.74 121.563 140.987 128.381 132.296C133.559 125.696 136.834 117.143 138.28 106.366C144.217 109.949 148.617 114.664 151.047 120.332C155.179 129.967 155.42 145.8 142.501 158.708C131.182 170.016 117.576 174.908 97.0135 175.059C74.2042 174.89 56.9538 167.575 45.7381 153.317C35.2355 139.966 29.8077 120.682 29.6052 96C29.8077 71.3178 35.2355 52.0336 45.7381 38.6827C56.9538 24.4249 74.2039 17.11 97.0132 16.9405C119.988 17.1113 137.539 24.4614 149.184 38.788C154.894 45.8136 159.199 54.6488 162.037 64.9503L178.184 60.6422C174.744 47.9622 169.331 37.0357 161.965 27.974C147.036 9.60668 125.202 0.195148 97.0695 0H96.9569C68.8816 0.19447 47.2921 9.6418 32.7883 28.0793C19.8819 44.4864 13.2244 67.3157 13.0007 95.9325L13 96L13.0007 96.0675C13.2244 124.684 19.8819 147.514 32.7883 163.921C47.2921 182.358 68.8816 191.806 96.9569 192H97.0695C122.03 191.827 139.624 185.292 154.118 170.811C173.081 151.866 172.51 128.119 166.26 113.541C161.776 103.087 153.227 94.5962 141.537 88.9883ZM98.4405 129.507C88.0005 130.095 77.1544 125.409 76.6196 115.372C76.2232 107.93 81.9158 99.626 99.0812 98.6368C101.047 98.5234 102.976 98.468 104.871 98.468C111.106 98.468 116.939 99.0737 122.242 100.233C120.264 124.935 108.662 128.946 98.4405 129.507Z"/>
                        </svg>
                    </a>"#, threads));
            }
            
            social_html.push_str("</div>");
        }
        
        social_html
    }

    fn generate_html(&self, config: &Config, sidebar_html: &str, content_html: &str, homepage_html: &str, search_index: &str) -> String {
        // Get the first page ID for the Docs link
        let first_page_url = config.navigation
            .first()
            .and_then(|section| section.items.first().map(|item| format!("{}/{}", section.id, item.id)))
            .unwrap_or_else(|| "introduction/what-is-glowdoc".to_string());
        
        // Generate social media links HTML
        let social_links_html = self.generate_social_links_html(&config.social);
        let current_year = self.get_current_year();
        format!("<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>{} - Modern Documentation</title>
    <meta name=\"description\" content=\"{}\">
    <style>
{}
    </style>
</head>
<body>
    <header>
        <div class=\"container\">
            <div class=\"header-content\">
                <a href=\"#\" class=\"logo\" onclick=\"showHomepage()\">{}</a>
                <nav class=\"nav\">
                    <button class=\"mobile-menu-toggle\" onclick=\"toggleSidebar()\">
                        <svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\">
                            <line x1=\"3\" y1=\"6\" x2=\"21\" y2=\"6\"/>
                            <line x1=\"3\" y1=\"12\" x2=\"21\" y2=\"12\"/>
                            <line x1=\"3\" y1=\"18\" x2=\"21\" y2=\"18\"/>
                        </svg>
                    </button>
                    <a href=\"#\" onclick=\"showHomepage()\">Home</a>
                    <a href=\"#{}\">Docs</a>
                    <button class=\"theme-toggle\" onclick=\"toggleTheme()\">
                        <svg width=\"16\" height=\"16\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\">
                            <circle cx=\"12\" cy=\"12\" r=\"5\"/>
                            <path d=\"M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42\"/>
                        </svg>
                    </button>
                    {}
                </nav>
            </div>
        </div>
    </header>

    <div class=\"homepage active\" id=\"homepage\">
        <div class=\"homepage-content\">
{}
        </div>
    </div>

    <div class=\"layout\" id=\"docs-layout\">
        <aside class=\"sidebar\" id=\"sidebar\">
            <nav class=\"sidebar-nav\">
{}
            </nav>
        </aside>

        <main class=\"main-content\">
{}
        </main>
    </div>

    <footer>
        <div class=\"container\">
            <p>&copy; {} {}. Built with modern web standards.</p>
        </div>
    </footer>

    <script>
{}
{}
    </script>
</body>
</html>", 
            config.title, 
            config.description, 
            self.generate_css(&config.theme),
            config.title,
            first_page_url,
            social_links_html,
            homepage_html,
            sidebar_html,
            content_html,
            current_year,
            config.title,
            search_index,
            self.generate_javascript()
        )
    }

    fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Building GlowDoc...");
        
        let config = self.load_config()?;
        
        // Generate homepage, sidebar and content
        let homepage_html = self.load_homepage()?;
        let sidebar_html = self.generate_sidebar(&config.navigation);
        let (content_html, search_index) = self.generate_content(&config.navigation)?;
        
        // Generate complete HTML
        let html_content = self.generate_html(&config, &sidebar_html, &content_html, &homepage_html, &search_index);
        
        // Write the HTML file
        fs::write(&self.output_path, html_content)?;
        
        println!("Build completed successfully!");
        println!("Generated files:");
        println!("- {}", self.output_path);
        
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check for config generation command
    if args.len() > 1 && args[1] == "init-config" {
        // Pass remaining arguments (skip program name and "init-config")
        let config_args = &args[2..];
        if let Err(e) = generate_config_interactive(config_args) {
            eprintln!("Config generation failed: {}", e);
            std::process::exit(1);
        }
        return;
    }
    
    // Default behavior: build the site
    let builder = GlowDocBuilder::new();
    
    // Check if config.yaml exists
    if !Path::new(&builder.config_path).exists() {
        eprintln!("❌ Configuration file not found: {}", builder.config_path);
        eprintln!("");
        eprintln!("To get started, run:");
        eprintln!("  glowdoc init-config");
        eprintln!("");
        eprintln!("This will create a config.yaml file with your documentation structure.");
        std::process::exit(1);
    }
    
    if let Err(e) = builder.build() {
        eprintln!("Build failed: {}", e);
        std::process::exit(1);
    }
}

fn generate_config_interactive(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let config_builder = ConfigBuilder::new("docs");
    
    // Check for help flag
    if args.iter().any(|arg| arg == "--help") {
        ConfigBuilder::print_help();
        return Ok(());
    }
    
    // Parse command-line options
    let options = ConfigBuilder::parse_options(args)?;
    
    // Determine if we have any CLI options (non-interactive mode)
    let has_options = options.title.is_some() 
        || options.description.is_some() 
        || options.section_order.is_some()
        || !options.section_renames.is_empty()
        || !options.page_renames.is_empty()
        || !options.page_orders.is_empty()
        || !options.exclude_sections.is_empty()
        || options.social.instagram.is_some()
        || options.social.github.is_some()
        || options.social.mastodon.is_some()
        || options.social.threads.is_some();
    
    // Check if config.yaml already exists
    if Path::new("docs/config.yaml").exists() {
        println!("⚠️  docs/config.yaml already exists!");
        
        if has_options {
            // Non-interactive mode: always backup and proceed
            let backup_name = format!("docs/config.yaml.backup.{}", 
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs());
            fs::copy("docs/config.yaml", &backup_name)?;
            println!("✅ Backed up existing config to {}", backup_name);
        } else {
            // Interactive mode: ask user
            println!("Do you want to:");
            println!("1. Backup existing and create new");
            println!("2. Cancel");
            
            print!("Choose option (1-2): ");
            std::io::Write::flush(&mut std::io::stdout())?;
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "1" => {
                    // Backup existing config
                    let backup_name = format!("docs/config.yaml.backup.{}", 
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)?
                            .as_secs());
                    fs::copy("docs/config.yaml", &backup_name)?;
                    println!("✅ Backed up existing config to {}", backup_name);
                }
                _ => {
                    println!("❌ Cancelled");
                    return Ok(());
                }
            }
        }
    }
    
    // Generate new config
    let config = if has_options {
        // Non-interactive mode with CLI options
        config_builder.build_config_with_options(options)?
    } else {
        // Interactive mode
        config_builder.build_config_interactive()?
    };
    
    config_builder.save_config(&config)?;
    
    println!("\n🎉 Success!");
    println!("Now run 'cargo run --release' to build your documentation site.");
    
    Ok(())
}