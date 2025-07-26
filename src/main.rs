use std::fs;
use std::path::Path;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Options, html, Event, Tag, HeadingLevel};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use notify::{Watcher, RecursiveMode, recommended_watcher};
use std::sync::mpsc;
use std::thread;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

mod config_builder;
use config_builder::ConfigBuilder;

mod css;
mod javascript;

#[derive(Debug, Deserialize, Serialize)]
struct NavigationItem {
    title: String,
    id: String,
    file: Option<String>,
    #[serde(default)]
    items: Vec<NavigationItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<HeaderItem>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct HeaderItem {
    title: String,
    id: String,
    level: u8,
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

    fn extract_headers_from_markdown(&self, content: &str) -> Vec<HeaderItem> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        
        let parser = Parser::new_ext(content, options);
        let mut headers = Vec::new();
        let mut in_heading = false;
        let mut current_level = 1;
        let mut current_text = String::new();
        
        for event in parser {
            match event {
                Event::Start(Tag::Heading(level, _, _)) => {
                    in_heading = true;
                    current_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    current_text.clear();
                }
                Event::End(Tag::Heading(_, _, _)) => {
                    if in_heading && !current_text.is_empty() {
                        let id = self.slugify(&current_text);
                        headers.push(HeaderItem {
                            title: current_text.clone(),
                            id,
                            level: current_level,
                        });
                    }
                    in_heading = false;
                }
                Event::Text(text) if in_heading => {
                    current_text.push_str(&text);
                }
                _ => {}
            }
        }
        
        headers
    }

    fn slugify(&self, text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
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
        
        // Add IDs to headers in the generated HTML
        let processed_html = self.add_header_ids_to_html(&html_output, &content);
        
        Ok(processed_html)
    }

    fn add_header_ids_to_html(&self, html: &str, _markdown_content: &str) -> String {
        use regex::Regex;
        
        let re = Regex::new(r"<h([1-6])>([^<]+)</h[1-6]>").unwrap();
        
        re.replace_all(html, |caps: &regex::Captures| {
            let level = &caps[1];
            let text = &caps[2];
            let id = self.slugify(text);
            format!("<h{} id=\"{}\">{}</h{}>", level, id, text, level)
        }).to_string()
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

    fn extract_headers_and_update_navigation(&self, navigation: &mut [NavigationSection]) -> Result<(), Box<dyn std::error::Error>> {
        for section in navigation.iter_mut() {
            for item in section.items.iter_mut() {
                self.update_item_headers(item)?;
            }
        }
        Ok(())
    }

    fn update_item_headers(&self, item: &mut NavigationItem) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(file) = &item.file {
            let full_path = Path::new(&self.docs_path).join(file);
            if let Ok(content) = fs::read_to_string(full_path) {
                let headers = self.extract_headers_from_markdown(&content);
                if !headers.is_empty() {
                    item.headers = Some(headers);
                }
            }
        }
        
        // Recursively process nested items
        for nested_item in item.items.iter_mut() {
            self.update_item_headers(nested_item)?;
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

    fn generate_html(&self, config: &Config, sidebar_html: &str, content_html: &str, homepage_html: &str, search_index: &str, enable_hot_reload: bool) -> String {
        // Get the first page ID for the Docs link
        let first_page_url = config.navigation
            .first()
            .and_then(|section| section.items.first().map(|item| format!("{}/{}", section.id, item.id)))
            .unwrap_or_else(|| "introduction/what-is-glowdoc".to_string());
        
        // Generate social media links HTML
        let social_links_html = self.generate_social_links_html(&config.social);
        let current_year = self.get_current_year();
        
        // Check if favicon.ico exists in docs folder
        let favicon_html = if Path::new("docs/favicon.ico").exists() {
            "\n    <link rel=\"shortcut icon\" type=\"image/x-icon\" href=\"favicon.ico\">"
        } else {
            ""
        };
        
        format!("<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>{} - Modern Documentation</title>
    <meta name=\"description\" content=\"{}\">{}
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
            <div class=\"content-wrapper\">
                <div class=\"content-area\">
{}
                </div>
                <aside class=\"table-of-contents\" id=\"table-of-contents\">
                    <div class=\"toc-header\">
                        <h3>On this page</h3>
                    </div>
                    <nav class=\"toc-nav\" id=\"toc-nav\">
                        <!-- Table of contents will be populated by JavaScript -->
                    </nav>
                </aside>
            </div>
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
            favicon_html,
            css::generate_css(&config.theme),
            config.title,
            first_page_url,
            social_links_html,
            homepage_html,
            sidebar_html,
            content_html,
            current_year,
            config.title,
            search_index,
            javascript::generate_javascript(enable_hot_reload)
        )
    }

    fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Building GlowDoc...");
        
        let mut config = self.load_config()?;
        
        // Extract headers from markdown files and update navigation
        self.extract_headers_and_update_navigation(&mut config.navigation)?;
        
        // Generate homepage, sidebar and content
        let homepage_html = self.load_homepage()?;
        let sidebar_html = self.generate_sidebar(&config.navigation);
        let (content_html, search_index) = self.generate_content(&config.navigation)?;
        
        // Generate complete HTML
        let html_content = self.generate_html(&config, &sidebar_html, &content_html, &homepage_html, &search_index, false);
        
        // Write the HTML file
        fs::write(&self.output_path, html_content)?;
        
        println!("Build completed successfully!");
        println!("Generated files:");
        println!("- {}", self.output_path);
        
        Ok(())
    }
    
    fn build_with_hot_reload(&self, enable_hot_reload: bool) -> Result<(), Box<dyn std::error::Error>> {
        if std::env::var("GLOWDOC_DEBUG").is_ok() {
            println!("Building GlowDoc...");
        }
        
        let mut config = self.load_config()?;
        
        // Extract headers from markdown files and update navigation
        self.extract_headers_and_update_navigation(&mut config.navigation)?;
        
        // Generate homepage, sidebar and content
        let homepage_html = self.load_homepage()?;
        let sidebar_html = self.generate_sidebar(&config.navigation);
        let (content_html, search_index) = self.generate_content(&config.navigation)?;
        
        // Generate complete HTML
        let html_content = self.generate_html(&config, &sidebar_html, &content_html, &homepage_html, &search_index, enable_hot_reload);
        
        // Write the HTML file
        fs::write(&self.output_path, html_content)?;
        
        if std::env::var("GLOWDOC_DEBUG").is_ok() {
            println!("Build completed successfully!");
            println!("Generated files:");
            println!("- {}", self.output_path);
        }
        
        Ok(())
    }
    
    async fn start_development_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔥 Starting development server...");
        
        let (reload_tx, _) = broadcast::channel(16);
        let reload_tx_clone = reload_tx.clone();
        
        // Start HTTP server for serving the documentation
        let http_server = {
            let make_svc = make_service_fn(|_conn| async {
                Ok::<_, Infallible>(service_fn(Self::handle_http_request))
            });
            
            let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
            println!("📖 HTTP server starting on http://localhost:8000");
            
            let server = Server::bind(&addr).serve(make_svc);
            tokio::spawn(async move {
                if let Err(e) = server.await {
                    eprintln!("❌ HTTP server error: {}", e);
                }
            })
        };
        
        // Start WebSocket server for hot reload
        let ws_server = {
            let listener = TcpListener::bind("127.0.0.1:8081").await?;
            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                println!("🔥 WebSocket server starting on ws://localhost:8081");
            }
            
            tokio::spawn(async move {
                while let Ok((stream, addr)) = listener.accept().await {
                    let reload_rx = reload_tx.subscribe();
                    tokio::spawn(Self::handle_websocket(stream, addr, reload_rx));
                }
            })
        };
        
        // Start file watcher in a separate thread
        let docs_path = self.docs_path.clone();
        let builder = GlowDocBuilder::new();
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            
            let mut watcher = recommended_watcher(move |res| {
                match res {
                    Ok(event) => {
                        if let Err(e) = tx.send(event) {
                            eprintln!("Error sending file event: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Watch error: {:?}", e),
                }
            }).expect("Failed to create file watcher");
            
            watcher.watch(Path::new(&docs_path), RecursiveMode::Recursive)
                .expect("Failed to watch docs directory");
            
            println!("👀 Watching for changes in {}/", docs_path);
            
            // Debouncing mechanism to prevent duplicate rebuilds
            let mut last_rebuild_times: HashMap<String, Instant> = HashMap::new();
            let debounce_duration = Duration::from_millis(200); // 200ms debounce
            
            loop {
                match rx.recv() {
                    Ok(event) => {
                        // Extract file paths from the event
                        let file_paths: Vec<String> = event.paths
                            .iter()
                            .filter_map(|p| p.to_str().map(|s| s.to_string()))
                            .collect();
                        
                        if file_paths.is_empty() {
                            continue;
                        }
                        
                        if std::env::var("GLOWDOC_DEBUG").is_ok() {
                            println!("📁 File change detected: {:?}", event);
                        }
                        
                        // Check if we should rebuild based on debouncing
                        let now = Instant::now();
                        let should_rebuild = file_paths.iter().any(|path| {
                            if let Some(&last_time) = last_rebuild_times.get(path) {
                                now.duration_since(last_time) > debounce_duration
                            } else {
                                true // First time seeing this file
                            }
                        });
                        
                        if should_rebuild {
                            // Update the last rebuild time for all affected files
                            for path in &file_paths {
                                last_rebuild_times.insert(path.clone(), now);
                            }
                            
                            // Show a simple rebuild message for normal use
                            let file_name = file_paths.first()
                                .and_then(|p| std::path::Path::new(p).file_name())
                                .and_then(|name| name.to_str())
                                .unwrap_or("files");
                            println!("🔄 Rebuilding after {} change...", file_name);
                            
                            // Rebuild the site
                            if let Err(e) = builder.build_with_hot_reload(true) {
                                eprintln!("❌ Build failed: {}", e);
                            } else {
                                println!("✅ Ready");
                                
                                // Send reload signal to all connected clients
                                if let Err(e) = reload_tx_clone.send("reload".to_string()) {
                                    if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                        eprintln!("Error sending reload signal: {}", e);
                                    }
                                }
                            }
                        } else {
                            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                println!("⏭️  Skipping rebuild (debounced)");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("File watcher error: {}", e);
                        break;
                    }
                }
            }
        });
        
        // Wait for servers to complete (they run indefinitely)
        let _ = tokio::join!(http_server, ws_server);
        
        Ok(())
    }
    
    async fn handle_http_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let path = req.uri().path();
        
        // Handle static assets from docs directory
        if path.starts_with("/docs/") || path.contains('.') {
            return Self::serve_static_file(path).await;
        }
        
        // Serve index.html for all other requests (SPA behavior)
        match fs::read_to_string("index.html") {
            Ok(content) => {
                Ok(Response::builder()
                    .header("content-type", "text/html; charset=utf-8")
                    .header("cache-control", "no-cache, no-store, must-revalidate")
                    .header("pragma", "no-cache")
                    .header("expires", "0")
                    .body(Body::from(content))
                    .unwrap())
            }
            Err(_) => {
                let error_html = r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>GlowDoc - File Not Found</title>
                    <style>
                        body { font-family: Arial, sans-serif; margin: 50px; text-align: center; }
                        .error { color: #e74c3c; }
                        .suggestion { color: #2c3e50; margin-top: 20px; }
                    </style>
                </head>
                <body>
                    <h1 class="error">📄 Documentation not found</h1>
                    <p>The index.html file hasn't been generated yet.</p>
                    <div class="suggestion">
                        <p>Make sure you have:</p>
                        <ul style="text-align: left; display: inline-block;">
                            <li>Created a <code>docs/config.yaml</code> file</li>
                            <li>Added some markdown files to the <code>docs/</code> directory</li>
                            <li>Run the build process</li>
                        </ul>
                        <p>Try running: <code>cargo run init-config</code> first</p>
                    </div>
                </body>
                </html>
                "#;
                
                Ok(Response::builder()
                    .status(404)
                    .header("content-type", "text/html; charset=utf-8")
                    .body(Body::from(error_html))
                    .unwrap())
            }
        }
    }
    
    async fn serve_static_file(path: &str) -> Result<Response<Body>, Infallible> {
        // Clean up the path and resolve to file system
        let clean_path = path.trim_start_matches('/');
        let file_path = if clean_path.starts_with("docs/") {
            // Direct reference to docs folder
            clean_path.to_string()
        } else {
            // Assume it's a relative reference from within the docs
            format!("docs/{}", clean_path)
        };
        
        // Try to read the file
        match fs::read(&file_path) {
            Ok(content) => {
                let content_type = Self::get_content_type(&file_path);
                
                Ok(Response::builder()
                    .header("content-type", content_type)
                    .header("cache-control", "no-cache, no-store, must-revalidate")
                    .header("pragma", "no-cache")
                    .header("expires", "0")
                    .body(Body::from(content))
                    .unwrap())
            }
            Err(_) => {
                let not_found_html = format!(r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>File Not Found</title>
                    <style>
                        body {{ font-family: Arial, sans-serif; margin: 50px; text-align: center; }}
                        .error {{ color: #e74c3c; }}
                    </style>
                </head>
                <body>
                    <h1 class="error">📄 File not found</h1>
                    <p>The requested file <code>{}</code> could not be found.</p>
                    <p><a href="/">← Back to documentation</a></p>
                </body>
                </html>
                "#, file_path);
                
                Ok(Response::builder()
                    .status(404)
                    .header("content-type", "text/html; charset=utf-8")
                    .body(Body::from(not_found_html))
                    .unwrap())
            }
        }
    }
    
    fn get_content_type(file_path: &str) -> &'static str {
        let extension = file_path
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            // Images
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "webp" => "image/webp",
            "ico" => "image/x-icon",
            "bmp" => "image/bmp",
            "tiff" | "tif" => "image/tiff",
            
            // Text and markup
            "html" | "htm" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" | "mjs" => "application/javascript; charset=utf-8",
            "json" => "application/json; charset=utf-8",
            "xml" => "application/xml; charset=utf-8",
            "txt" => "text/plain; charset=utf-8",
            "md" => "text/markdown; charset=utf-8",
            "yaml" | "yml" => "text/yaml; charset=utf-8",
            
            // Documents
            "pdf" => "application/pdf",
            "doc" => "application/msword",
            "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            
            // Audio
            "mp3" => "audio/mpeg",
            "wav" => "audio/wav",
            "ogg" => "audio/ogg",
            "m4a" => "audio/mp4",
            
            // Video
            "mp4" => "video/mp4",
            "webm" => "video/webm",
            "mov" => "video/quicktime",
            "avi" => "video/x-msvideo",
            
            // Archives
            "zip" => "application/zip",
            "tar" => "application/x-tar",
            "gz" => "application/gzip",
            
            // Fonts
            "woff" => "font/woff",
            "woff2" => "font/woff2",
            "ttf" => "font/ttf",
            "otf" => "font/otf",
            "eot" => "application/vnd.ms-fontobject",
            
            // Default
            _ => "application/octet-stream",
        }
    }
    
    async fn handle_websocket(
        stream: TcpStream,
        addr: std::net::SocketAddr,
        mut reload_rx: broadcast::Receiver<String>,
    ) {
        let ws_stream = match accept_async(stream).await {
            Ok(ws) => ws,
            Err(e) => {
                eprintln!("❌ WebSocket connection error from {}: {}", addr, e);
                return;
            }
        };
        
        // Debug level logging for connections
        if std::env::var("GLOWDOC_DEBUG").is_ok() {
            println!("🔌 WebSocket client connected: {}", addr);
        }
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        // Send initial connection confirmation
        if let Err(e) = ws_sender.send(Message::Text("connected".to_string())).await {
            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                eprintln!("❌ Failed to send initial message: {}", e);
            }
            return;
        }
        
        // Handle incoming messages and reload signals
        loop {
            tokio::select! {
                // Handle reload broadcasts
                reload_msg = reload_rx.recv() => {
                    match reload_msg {
                        Ok(msg) => {
                            if let Err(e) = ws_sender.send(Message::Text(msg)).await {
                                if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                    eprintln!("❌ Failed to send reload message to {}: {}", addr, e);
                                }
                                break;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {
                            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                eprintln!("⚠️  Client {} lagged behind, reconnection recommended", addr);
                            }
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                println!("📡 Reload channel closed, disconnecting {}", addr);
                            }
                            break;
                        }
                    }
                }
                
                // Handle incoming WebSocket messages
                ws_msg = ws_receiver.next() => {
                    match ws_msg {
                        Some(Ok(Message::Close(_))) => {
                            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                println!("🔌 Client {} disconnected", addr);
                            }
                            break;
                        }
                        Some(Err(e)) => {
                            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                eprintln!("❌ WebSocket error from {}: {}", addr, e);
                            }
                            break;
                        }
                        None => {
                            if std::env::var("GLOWDOC_DEBUG").is_ok() {
                                println!("🔌 Client {} connection closed", addr);
                            }
                            break;
                        }
                        _ => {} // Ignore other message types
                    }
                }
            }
        }
        
        if std::env::var("GLOWDOC_DEBUG").is_ok() {
            println!("🔌 WebSocket client {} disconnected", addr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generated_index_matches_current() {
        // Create a new builder instance
        let builder = GlowDocBuilder::new();
        
        // Read the current index.html content
        let current_content = fs::read_to_string(&builder.output_path)
            .expect("Failed to read current index.html file");
        
        // Load config and generate new content
        let mut config = builder.load_config()
            .expect("Failed to load config");
        
        // Extract headers and update navigation
        builder.extract_headers_and_update_navigation(&mut config.navigation)
            .expect("Failed to extract headers");
        
        // Generate all components
        let homepage_html = builder.load_homepage()
            .expect("Failed to load homepage");
        let sidebar_html = builder.generate_sidebar(&config.navigation);
        let (content_html, search_index) = builder.generate_content(&config.navigation)
            .expect("Failed to generate content");
        
        // Generate the complete HTML (without hot reload)
        let generated_content = builder.generate_html(
            &config, 
            &sidebar_html, 
            &content_html, 
            &homepage_html, 
            &search_index, 
            false
        );
        
        // Compare the contents
        assert_eq!(
            current_content.trim(), 
            generated_content.trim(),
            "Generated index.html content does not match the current index.html file"
        );
    }
}

#[tokio::main]
async fn main() {
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
    
    // Check for watch command
    if args.len() > 1 && args[1] == "watch" {
        let builder = GlowDocBuilder::new();
        
        // Check if config.yaml exists
        if !Path::new(&builder.config_path).exists() {
            eprintln!("❌ Configuration file not found: {}", builder.config_path);
            eprintln!("");
            eprintln!("To get started, run:");
            eprintln!("  cargo run init-config");
            eprintln!("");
            eprintln!("This will create a config.yaml file with your documentation structure.");
            std::process::exit(1);
        }
        
        // Build the site once with hot reload enabled
        println!("🔨 Building initial site...");
        if let Err(e) = builder.build_with_hot_reload(true) {
            eprintln!("❌ Initial build failed: {}", e);
            std::process::exit(1);
        }
        println!("✅ Initial build complete");
        
        println!("🚀 Development server starting...");
        println!("📖 Open http://localhost:8000 to view your documentation");
        println!("🔥 Hot reload enabled - changes will automatically refresh the browser");
        if std::env::var("GLOWDOC_DEBUG").is_ok() {
            println!("🐛 Debug mode enabled - verbose logging active");
        }
        println!("⏹️  Press Ctrl+C to stop the server");
        
        // Start the development server (HTTP + WebSocket + File Watcher)
        if let Err(e) = builder.start_development_server().await {
            eprintln!("❌ Hot reload server failed: {}", e);
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