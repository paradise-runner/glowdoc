use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedItem {
    pub title: String,
    pub id: String,
    pub file: Option<String>,
    pub original_filename: String,
    #[serde(default)]
    pub items: Vec<DetectedItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedSection {
    pub title: String,
    pub id: String,
    pub original_folder: String,
    pub items: Vec<DetectedItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedConfig {
    pub title: String,
    pub description: String,
    pub navigation: Vec<DetectedSection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigItem {
    pub title: String,
    pub id: String,
    pub file: Option<String>,
    #[serde(default)]
    pub items: Vec<ConfigItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSection {
    pub title: String,
    pub id: String,
    pub items: Vec<ConfigItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub navigation: Vec<ConfigSection>,
    #[serde(default)]
    pub social: SocialLinks,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastodon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigOptions {
    pub title: Option<String>,
    pub description: Option<String>,
    pub section_order: Option<Vec<String>>,
    pub section_renames: HashMap<String, String>,
    pub page_renames: HashMap<String, String>, // "section/file" -> "new title"
    pub page_orders: HashMap<String, Vec<String>>, // "section" -> ["file1", "file2"]
    pub exclude_sections: Vec<String>,
    pub social: SocialLinks,
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            section_order: None,
            section_renames: HashMap::new(),
            page_renames: HashMap::new(),
            page_orders: HashMap::new(),
            exclude_sections: Vec::new(),
            social: SocialLinks::default(),
        }
    }
}

pub struct ConfigBuilder {
    docs_path: PathBuf,
}

impl ConfigBuilder {
    pub fn new<P: AsRef<Path>>(docs_path: P) -> Self {
        Self {
            docs_path: docs_path.as_ref().to_path_buf(),
        }
    }
    
    /// Parse command-line arguments into ConfigOptions
    pub fn parse_options(args: &[String]) -> Result<ConfigOptions, Box<dyn std::error::Error>> {
        let mut options = ConfigOptions::default();
        let mut i = 0;
        
        while i < args.len() {
            match args[i].as_str() {
                "--title" => {
                    i += 1;
                    if i < args.len() {
                        options.title = Some(args[i].clone());
                    } else {
                        return Err("--title requires a value".into());
                    }
                }
                "--description" => {
                    i += 1;
                    if i < args.len() {
                        options.description = Some(args[i].clone());
                    } else {
                        return Err("--description requires a value".into());
                    }
                }
                "--section-order" => {
                    i += 1;
                    if i < args.len() {
                        options.section_order = Some(args[i].split(',').map(|s| s.trim().to_string()).collect());
                    } else {
                        return Err("--section-order requires a comma-separated list".into());
                    }
                }
                "--rename-section" => {
                    i += 1;
                    if i < args.len() {
                        let parts: Vec<&str> = args[i].splitn(2, '=').collect();
                        if parts.len() == 2 {
                            options.section_renames.insert(parts[0].to_string(), parts[1].to_string());
                        } else {
                            return Err("--rename-section requires format 'old=new'".into());
                        }
                    } else {
                        return Err("--rename-section requires a value".into());
                    }
                }
                "--rename-page" => {
                    i += 1;
                    if i < args.len() {
                        let parts: Vec<&str> = args[i].splitn(2, '=').collect();
                        if parts.len() == 2 {
                            options.page_renames.insert(parts[0].to_string(), parts[1].to_string());
                        } else {
                            return Err("--rename-page requires format 'section/file.md=New Title'".into());
                        }
                    } else {
                        return Err("--rename-page requires a value".into());
                    }
                }
                "--page-order" => {
                    i += 1;
                    if i < args.len() {
                        let parts: Vec<&str> = args[i].splitn(2, '=').collect();
                        if parts.len() == 2 {
                            let files: Vec<String> = parts[1].split(',').map(|s| s.trim().to_string()).collect();
                            options.page_orders.insert(parts[0].to_string(), files);
                        } else {
                            return Err("--page-order requires format 'section=file1.md,file2.md'".into());
                        }
                    } else {
                        return Err("--page-order requires a value".into());
                    }
                }
                "--exclude-section" => {
                    i += 1;
                    if i < args.len() {
                        options.exclude_sections.push(args[i].clone());
                    } else {
                        return Err("--exclude-section requires a value".into());
                    }
                }
                "--instagram" => {
                    i += 1;
                    if i < args.len() {
                        options.social.instagram = Some(args[i].clone());
                    } else {
                        return Err("--instagram requires a value".into());
                    }
                }
                "--github" => {
                    i += 1;
                    if i < args.len() {
                        options.social.github = Some(args[i].clone());
                    } else {
                        return Err("--github requires a value".into());
                    }
                }
                "--mastodon" => {
                    i += 1;
                    if i < args.len() {
                        options.social.mastodon = Some(args[i].clone());
                    } else {
                        return Err("--mastodon requires a value".into());
                    }
                }
                "--threads" => {
                    i += 1;
                    if i < args.len() {
                        options.social.threads = Some(args[i].clone());
                    } else {
                        return Err("--threads requires a value".into());
                    }
                }
                _ => {
                    // Ignore unknown arguments for now
                }
            }
            i += 1;
        }
        
        Ok(options)
    }
    
    /// Print help for command-line options
    pub fn print_help() {
        println!("GlowDoc Configuration Builder");
        println!("============================");
        println!();
        println!("USAGE:");
        println!("    cargo run init-config [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    --title <TITLE>                    Set the site title");
        println!("    --description <DESC>               Set the site description");
        println!("    --section-order <ORDER>            Comma-separated list of section folders in order");
        println!("                                       Example: --section-order introduction,getting-started,advanced");
        println!();
        println!("    --rename-section <OLD=NEW>         Rename a section");
        println!("                                       Example: --rename-section getting-started=\"Quick Start\"");
        println!();
        println!("    --rename-page <PATH=TITLE>         Rename a page");
        println!("                                       Example: --rename-page introduction/quick-start.md=\"Getting Started\"");
        println!();
        println!("    --page-order <SECTION=FILES>       Set page order within a section");
        println!("                                       Example: --page-order getting-started=installation.md,config.md");
        println!();
        println!("    --exclude-section <SECTION>        Exclude a section from the config");
        println!("                                       Example: --exclude-section drafts");
        println!();
        println!("    --instagram <USERNAME>             Instagram username");
        println!("    --github <USERNAME>                GitHub username");
        println!("    --mastodon <URL>                   Mastodon profile URL");
        println!("    --threads <USERNAME>               Threads username");
        println!();
        println!("    --help                             Show this help message");
        println!();
        println!("EXAMPLES:");
        println!("    # Interactive mode");
        println!("    cargo run init-config");
        println!();
        println!("    # Fully automated");
        println!("    cargo run init-config \\");
        println!("        --title \"My Documentation\" \\");
        println!("        --description \"Comprehensive guide\" \\");
        println!("        --section-order introduction,setup,advanced \\");
        println!("        --rename-section setup=\"Quick Setup\" \\");
        println!("        --page-order setup=install.md,configure.md");
        println!();
        println!("    # Auto-detect with some customizations");
        println!("    cargo run init-config \\");
        println!("        --title \"API Docs\" \\");
        println!("        --exclude-section drafts");
    }

    /// Auto-detect the documentation structure
    pub fn detect_structure(&self) -> Result<DetectedConfig, Box<dyn std::error::Error>> {
        let mut sections = Vec::new();
        
        // Read all directories in docs/
        let entries = fs::read_dir(&self.docs_path)?;
        let mut folders = Vec::new();
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                    folders.push((folder_name.to_string(), path));
                }
            }
        }
        
        // Sort folders alphabetically for consistent ordering
        folders.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (folder_name, folder_path) in folders {
            let section = self.detect_section(&folder_name, &folder_path)?;
            if !section.items.is_empty() {
                sections.push(section);
            }
        }
        
        Ok(DetectedConfig {
            title: "Documentation".to_string(),
            description: "Generated documentation site".to_string(),
            navigation: sections,
        })
    }
    
    fn detect_section(&self, folder_name: &str, folder_path: &Path) -> Result<DetectedSection, Box<dyn std::error::Error>> {
        let items = self.detect_items_recursive(folder_path, folder_name, "")?;
        
        Ok(DetectedSection {
            title: self.format_title(folder_name),
            id: self.generate_id(folder_name),
            original_folder: folder_name.to_string(),
            items,
        })
    }

    fn detect_items_recursive(&self, dir_path: &Path, base_folder: &str, relative_path: &str) -> Result<Vec<DetectedItem>, Box<dyn std::error::Error>> {
        let mut items = Vec::new();
        
        // Read all entries in the directory
        let entries = fs::read_dir(dir_path)?;
        let mut files = Vec::new();
        let mut folders = Vec::new();
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    files.push((filename.to_string(), path));
                }
            } else if path.is_dir() {
                if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                    folders.push((folder_name.to_string(), path));
                }
            }
        }
        
        // Sort files and folders alphabetically
        files.sort_by(|a, b| a.0.cmp(&b.0));
        folders.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Process files first
        for (filename, file_path) in files {
            let item = self.detect_file_item(&filename, &file_path, base_folder, relative_path)?;
            items.push(item);
        }
        
        // Process nested folders
        for (folder_name, folder_path) in folders {
            let nested_relative_path = if relative_path.is_empty() {
                folder_name.clone()
            } else {
                format!("{}/{}", relative_path, folder_name)
            };
            
            let nested_items = self.detect_items_recursive(&folder_path, base_folder, &nested_relative_path)?;
            
            if !nested_items.is_empty() {
                // Create a folder item that contains nested items
                let folder_item = DetectedItem {
                    title: self.format_title(&folder_name),
                    id: self.generate_id(&format!("{}_{}", relative_path.replace('/', "_"), folder_name)),
                    file: None,
                    original_filename: folder_name,
                    items: nested_items,
                };
                items.push(folder_item);
            }
        }
        
        Ok(items)
    }
    
    fn detect_file_item(&self, filename: &str, file_path: &Path, base_folder: &str, relative_path: &str) -> Result<DetectedItem, Box<dyn std::error::Error>> {
        let file_stem = filename.strip_suffix(".md").unwrap_or(filename);
        
        // Try to extract title from the first H1 in the markdown file
        let title = self.extract_title_from_markdown(file_path)
            .unwrap_or_else(|| self.format_title(file_stem));
        
        let full_relative_path = if relative_path.is_empty() {
            format!("{}/{}", base_folder, filename)
        } else {
            format!("{}/{}/{}", base_folder, relative_path, filename)
        };
        
        Ok(DetectedItem {
            title,
            id: self.generate_id(&format!("{}_{}", relative_path.replace('/', "_"), file_stem)),
            file: Some(full_relative_path),
            original_filename: filename.to_string(),
            items: Vec::new(),
        })
    }
    
    fn extract_title_from_markdown(&self, file_path: &Path) -> Option<String> {
        let content = fs::read_to_string(file_path).ok()?;
        let parser = Parser::new(&content);
        
        let mut in_heading = false;
        let mut title_text = String::new();
        
        for event in parser {
            match event {
                Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
                    in_heading = true;
                    title_text.clear();
                }
                Event::End(Tag::Heading(HeadingLevel::H1, _, _)) => {
                    if in_heading && !title_text.trim().is_empty() {
                        return Some(title_text.trim().to_string());
                    }
                    in_heading = false;
                }
                Event::Text(text) if in_heading => {
                    title_text.push_str(&text);
                }
                _ => {}
            }
        }
        
        None
    }
    
    fn format_title(&self, input: &str) -> String {
        input
            .replace('-', " ")
            .replace('_', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    fn generate_id(&self, input: &str) -> String {
        input
            .to_lowercase()
            .replace(' ', "-")
            .replace('_', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }
    
    /// Non-interactive configuration builder with options
    pub fn build_config_with_options(&self, options: ConfigOptions) -> Result<Config, Box<dyn std::error::Error>> {
        println!("🔍 Scanning docs folder...");
        let mut detected = self.detect_structure()?;
        
        // Apply site info
        if let Some(title) = options.title {
            detected.title = title;
        }
        if let Some(description) = options.description {
            detected.description = description;
        }
        
        // Apply section renames
        for section in &mut detected.navigation {
            if let Some(new_name) = options.section_renames.get(&section.original_folder) {
                section.title = new_name.clone();
                section.id = self.generate_id(new_name);
            }
        }
        
        // Apply page renames
        for section in &mut detected.navigation {
            for item in &mut section.items {
                let key = format!("{}/{}", section.original_folder, item.original_filename);
                if let Some(new_title) = options.page_renames.get(&key) {
                    item.title = new_title.clone();
                    item.id = self.generate_id(new_title);
                }
            }
        }
        
        // Apply page reordering within sections
        for section in &mut detected.navigation {
            if let Some(file_order) = options.page_orders.get(&section.original_folder) {
                let mut reordered_items = Vec::new();
                
                // First, add items in the specified order
                for filename in file_order {
                    if let Some(pos) = section.items.iter().position(|item| &item.original_filename == filename) {
                        reordered_items.push(section.items.remove(pos));
                    }
                }
                
                // Then add any remaining items that weren't specified
                reordered_items.extend(section.items.drain(..));
                section.items = reordered_items;
            }
        }
        
        // Filter out excluded sections
        detected.navigation.retain(|section| !options.exclude_sections.contains(&section.original_folder));
        
        // Apply section reordering
        if let Some(section_order) = options.section_order {
            let mut reordered_sections = Vec::new();
            
            // First, add sections in the specified order
            for folder_name in &section_order {
                if let Some(pos) = detected.navigation.iter().position(|section| &section.original_folder == folder_name) {
                    reordered_sections.push(detected.navigation.remove(pos));
                }
            }
            
            // Then add any remaining sections that weren't specified
            reordered_sections.extend(detected.navigation.drain(..));
            detected.navigation = reordered_sections;
        }
        
        // Convert to final config format
        let config = Config {
            title: detected.title,
            description: detected.description,
            navigation: detected.navigation.into_iter().map(|section| {
                ConfigSection {
                    title: section.title,
                    id: section.id,
                    items: section.items.into_iter().map(|item| {
                        self.convert_detected_item_to_config(item)
                    }).collect(),
                }
            }).collect(),
            social: options.social,
        };
        
        println!("✅ Configuration generated with {} sections", config.navigation.len());
        Ok(config)
    }

    fn convert_detected_item_to_config(&self, item: DetectedItem) -> ConfigItem {
        ConfigItem {
            title: item.title,
            id: item.id,
            file: item.file,
            items: item.items.into_iter().map(|nested_item| {
                self.convert_detected_item_to_config(nested_item)
            }).collect(),
        }
    }

    /// Interactive configuration builder
    pub fn build_config_interactive(&self) -> Result<Config, Box<dyn std::error::Error>> {
        println!("🚀 GlowDoc Configuration Builder");
        println!("=================================\n");
        
        // Auto-detect structure
        println!("🔍 Scanning docs folder...");
        let mut detected = self.detect_structure()?;
        
        // Show detected structure
        self.print_detected_structure(&detected);
        
        // Get basic site info
        println!("\n📝 Site Information");
        println!("-------------------");
        detected.title = self.prompt_with_default("Site title", &detected.title)?;
        detected.description = self.prompt_with_default("Site description", &detected.description)?;
        
        // Get social media links
        println!("\n🔗 Social Media Links (optional)");
        println!("--------------------------------");
        println!("Enter social media handles/URLs (press Enter to skip):");
        
        let mut social = SocialLinks::default();
        
        print!("Instagram username: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let instagram = input.trim();
        if !instagram.is_empty() {
            social.instagram = Some(instagram.to_string());
        }
        
        print!("GitHub username: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let github = input.trim();
        if !github.is_empty() {
            social.github = Some(github.to_string());
        }
        
        print!("Mastodon profile URL: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let mastodon = input.trim();
        if !mastodon.is_empty() {
            social.mastodon = Some(mastodon.to_string());
        }
        
        print!("Threads username: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let threads = input.trim();
        if !threads.is_empty() {
            social.threads = Some(threads.to_string());
        }
        
        // Ask what to do next
        println!("\n🛠️  Configuration Options");
        println!("-------------------------");
        println!("1. Accept all (generate config as-is)");
        println!("2. Customize sections (order, titles, remove)");
        println!("3. Customize pages (titles, reorder within sections)");
        println!("4. Full customization (sections + pages)");
        
        let choice = self.prompt_choice("Choose option (1-4)", 1, 4)?;
        
        match choice {
            1 => {
                println!("✅ Using auto-detected structure");
            }
            2 => {
                detected = self.customize_sections(detected)?;
            }
            3 => {
                detected = self.customize_pages(detected)?;
            }
            4 => {
                detected = self.customize_sections(detected)?;
                detected = self.customize_pages(detected)?;
            }
            _ => unreachable!()
        }
        
        // Convert to final config format
        let config = Config {
            title: detected.title,
            description: detected.description,
            navigation: detected.navigation.into_iter().map(|section| {
                ConfigSection {
                    title: section.title,
                    id: section.id,
                    items: section.items.into_iter().map(|item| {
                        self.convert_detected_item_to_config(item)
                    }).collect(),
                }
            }).collect(),
            social,
        };
        
        Ok(config)
    }
    
    fn print_detected_structure(&self, config: &DetectedConfig) {
        println!("📋 Detected Structure:");
        println!("----------------------");
        println!("Site: {} - {}", config.title, config.description);
        let total_pages = config.navigation.iter().map(|s| self.count_pages(&s.items)).sum::<usize>();
        println!("Found {} sections with {} total pages\n", 
                 config.navigation.len(),
                 total_pages);
        
        for (i, section) in config.navigation.iter().enumerate() {
            let page_count = self.count_pages(&section.items);
            println!("{}. 📁 {} ({})", i + 1, section.title, page_count);
            self.print_items(&section.items, 1);
            println!();
        }
    }

    fn count_pages(&self, items: &[DetectedItem]) -> usize {
        items.iter().map(|item| {
            if item.file.is_some() {
                1 + self.count_pages(&item.items)
            } else {
                self.count_pages(&item.items)
            }
        }).sum()
    }

    fn print_items(&self, items: &[DetectedItem], depth: usize) {
        let indent = "   ".repeat(depth);
        for item in items {
            if item.file.is_some() {
                println!("{}📄 {}", indent, item.title);
            } else if !item.items.is_empty() {
                println!("{}📁 {} ({})", indent, item.title, self.count_pages(&item.items));
                self.print_items(&item.items, depth + 1);
            }
        }
    }
    
    fn customize_sections(&self, mut config: DetectedConfig) -> Result<DetectedConfig, Box<dyn std::error::Error>> {
        println!("\n🔧 Customizing Sections");
        println!("-----------------------");
        
        // Show current order and allow reordering
        loop {
            println!("\nCurrent section order:");
            for (i, section) in config.navigation.iter().enumerate() {
                println!("{}. {}", i + 1, section.title);
            }
            
            println!("\nOptions:");
            println!("1. Rename a section");
            println!("2. Reorder sections");
            println!("3. Remove a section");
            println!("4. Customize pages within a section");
            println!("5. Continue");
            
            let choice = self.prompt_choice("Choose option (1-5)", 1, 5)?;
            
            match choice {
                1 => config = self.rename_section(config)?,
                2 => config = self.reorder_sections(config)?,
                3 => config = self.remove_section(config)?,
                4 => config = self.customize_section_pages(config)?,
                5 => break,
                _ => unreachable!()
            }
        }
        
        Ok(config)
    }
    
    fn customize_pages(&self, mut config: DetectedConfig) -> Result<DetectedConfig, Box<dyn std::error::Error>> {
        println!("\n📝 Customizing Pages");
        println!("--------------------");
        
        for section in &mut config.navigation {
            if section.items.is_empty() {
                continue;
            }
            
            loop {
                println!("\n📁 Section: {}", section.title);
                println!("Current page order:");
                for (i, item) in section.items.iter().enumerate() {
                    println!("{}. {}", i + 1, item.title);
                }
                
                println!("\nOptions:");
                println!("1. Rename a page title");
                println!("2. Reorder pages in this section");
                println!("3. Continue to next section");
                
                let choice = self.prompt_choice("Choose option (1-3)", 1, 3)?;
                
                match choice {
                    1 => {
                        section.items = self.rename_page(section.items.clone())?;
                    }
                    2 => {
                        section.items = self.reorder_pages(section.items.clone())?;
                    }
                    3 => break,
                    _ => unreachable!()
                }
            }
        }
        
        Ok(config)
    }
    
    fn rename_page(&self, mut items: Vec<DetectedItem>) -> Result<Vec<DetectedItem>, Box<dyn std::error::Error>> {
        if items.is_empty() {
            println!("No pages to rename.");
            return Ok(items);
        }
        
        println!("\nWhich page to rename?");
        for (i, item) in items.iter().enumerate() {
            println!("{}. {} ({})", i + 1, item.title, item.original_filename);
        }
        
        let index = self.prompt_choice("Page number", 1, items.len())? - 1;
        let old_title = &items[index].title;
        
        let new_title = self.prompt_with_default("New title", old_title)?;
        
        if new_title != *old_title {
            items[index].title = new_title.clone();
            items[index].id = self.generate_id(&new_title);
            println!("✅ Renamed page to '{}'", new_title);
        }
        
        Ok(items)
    }
    
    fn reorder_pages(&self, items: Vec<DetectedItem>) -> Result<Vec<DetectedItem>, Box<dyn std::error::Error>> {
        if items.len() < 2 {
            println!("Need at least 2 pages to reorder.");
            return Ok(items);
        }
        
        println!("\nCurrent page order:");
        for (i, item) in items.iter().enumerate() {
            println!("{}. {}", i + 1, item.title);
        }
        
        println!("\nEnter new order as space-separated numbers (e.g., '2 1 3 4'):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let new_order: Result<Vec<usize>, _> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().map(|n| n - 1))
            .collect();
        
        match new_order {
            Ok(indices) if indices.len() == items.len() && 
                          indices.iter().all(|&i| i < items.len()) => {
                let reordered_items = indices.into_iter().map(|i| items[i].clone()).collect();
                println!("✅ Pages reordered");
                Ok(reordered_items)
            }
            _ => {
                println!("❌ Invalid order. Keeping original order.");
                Ok(items)
            }
        }
    }
    
    fn rename_section(&self, mut config: DetectedConfig) -> Result<DetectedConfig, Box<dyn std::error::Error>> {
        if config.navigation.is_empty() {
            println!("No sections to rename.");
            return Ok(config);
        }
        
        println!("\nWhich section to rename?");
        for (i, section) in config.navigation.iter().enumerate() {
            println!("{}. {}", i + 1, section.title);
        }
        
        let index = self.prompt_choice("Section number", 1, config.navigation.len())? - 1;
        let old_title = &config.navigation[index].title;
        
        let new_title = self.prompt_with_default("New title", old_title)?;
        config.navigation[index].title = new_title.clone();
        config.navigation[index].id = self.generate_id(&new_title);
        
        println!("✅ Renamed section to '{}'", new_title);
        Ok(config)
    }
    
    fn reorder_sections(&self, mut config: DetectedConfig) -> Result<DetectedConfig, Box<dyn std::error::Error>> {
        if config.navigation.len() < 2 {
            println!("Need at least 2 sections to reorder.");
            return Ok(config);
        }
        
        println!("\nCurrent order:");
        for (i, section) in config.navigation.iter().enumerate() {
            println!("{}. {}", i + 1, section.title);
        }
        
        println!("\nEnter new order as space-separated numbers (e.g., '2 1 3 4'):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let new_order: Result<Vec<usize>, _> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().map(|n| n - 1))
            .collect();
        
        match new_order {
            Ok(indices) if indices.len() == config.navigation.len() && 
                          indices.iter().all(|&i| i < config.navigation.len()) => {
                let old_sections = config.navigation.clone();
                config.navigation = indices.into_iter().map(|i| old_sections[i].clone()).collect();
                println!("✅ Sections reordered");
            }
            _ => {
                println!("❌ Invalid order. Keeping original order.");
            }
        }
        
        Ok(config)
    }
    
    fn customize_section_pages(&self, mut config: DetectedConfig) -> Result<DetectedConfig, Box<dyn std::error::Error>> {
        if config.navigation.is_empty() {
            println!("No sections available.");
            return Ok(config);
        }
        
        println!("\nWhich section's pages do you want to customize?");
        for (i, section) in config.navigation.iter().enumerate() {
            println!("{}. {} ({} pages)", i + 1, section.title, section.items.len());
        }
        
        let section_index = self.prompt_choice("Section number", 1, config.navigation.len())? - 1;
        
        if config.navigation[section_index].items.is_empty() {
            println!("This section has no pages to customize.");
            return Ok(config);
        }
        
        // Customize pages for the selected section
        let section = &mut config.navigation[section_index];
        loop {
            println!("\n📁 Section: {}", section.title);
            println!("Current page order:");
            for (i, item) in section.items.iter().enumerate() {
                println!("{}. {}", i + 1, item.title);
            }
            
            println!("\nOptions:");
            println!("1. Rename a page title");
            println!("2. Reorder pages in this section");
            println!("3. Done with this section");
            
            let choice = self.prompt_choice("Choose option (1-3)", 1, 3)?;
            
            match choice {
                1 => {
                    section.items = self.rename_page(section.items.clone())?;
                }
                2 => {
                    section.items = self.reorder_pages(section.items.clone())?;
                }
                3 => break,
                _ => unreachable!()
            }
        }
        
        Ok(config)
    }

    fn remove_section(&self, mut config: DetectedConfig) -> Result<DetectedConfig, Box<dyn std::error::Error>> {
        if config.navigation.is_empty() {
            println!("No sections to remove.");
            return Ok(config);
        }
        
        println!("\nWhich section to remove?");
        for (i, section) in config.navigation.iter().enumerate() {
            println!("{}. {}", i + 1, section.title);
        }
        
        let index = self.prompt_choice("Section number", 1, config.navigation.len())? - 1;
        let removed = config.navigation.remove(index);
        
        println!("✅ Removed section '{}'", removed.title);
        Ok(config)
    }
    
    fn prompt_with_default(&self, prompt: &str, default: &str) -> Result<String, Box<dyn std::error::Error>> {
        print!("{} [{}]: ", prompt, default);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Ok(default.to_string())
        } else {
            Ok(trimmed.to_string())
        }
    }
    
    fn prompt_choice(&self, prompt: &str, min: usize, max: usize) -> Result<usize, Box<dyn std::error::Error>> {
        loop {
            print!("{}: ", prompt);
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim().parse::<usize>() {
                Ok(n) if n >= min && n <= max => return Ok(n),
                _ => println!("Please enter a number between {} and {}", min, max),
            }
        }
    }
    
    /// Save the configuration to docs/config.yaml
    pub fn save_config(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.docs_path.join("config.yaml");
        let yaml_content = serde_yaml::to_string(config)?;
        fs::write(&config_path, yaml_content)?;
        
        println!("✅ Configuration saved to {}", config_path.display());
        Ok(())
    }
}