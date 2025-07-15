pub fn generate_javascript(enable_hot_reload: bool) -> String {
    let mut js = String::new();
    
    if enable_hot_reload {
        js.push_str(r#"
        // Hot reload functionality
        function initHotReload() {
            const ws = new WebSocket('ws://localhost:8081');
            
            ws.onopen = function() {
                // Only log in debug mode or if explicitly enabled
                if (localStorage.getItem('glowdoc-debug') === 'true') {
                    console.log('🔥 Hot reload connected');
                }
            };
            
            ws.onmessage = function(event) {
                if (event.data === 'reload') {
                    console.log('🔄 Reloading page...');
                    window.location.reload();
                }
            };
            
            ws.onclose = function() {
                // Only log reconnection attempts in debug mode
                if (localStorage.getItem('glowdoc-debug') === 'true') {
                    console.log('🔌 Hot reload disconnected, attempting to reconnect...');
                }
                setTimeout(initHotReload, 1000);
            };
            
            ws.onerror = function(error) {
                // Only log errors in debug mode
                if (localStorage.getItem('glowdoc-debug') === 'true') {
                    console.log('🚫 Hot reload error:', error);
                }
            };
        }
        
        // Initialize hot reload when page loads
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', initHotReload);
        } else {
            initHotReload();
        }
        "#);
    }
    
    js.push_str(r#"
        function toggleTheme() {
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
            
            // Clear table of contents when showing homepage
            clearTableOfContents();
            
            // Update URL to homepage
            history.pushState({ page: 'homepage' }, '', window.location.pathname);
        }

        function showDocs() {
            document.getElementById('homepage').classList.remove('active');
            document.getElementById('docs-layout').classList.add('active');
        }

        function showContent(contentId, updateUrl = true, headerId = null) {
            console.log('showContent called with contentId:', contentId, 'headerId:', headerId);
            
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
                
                // Generate table of contents for this page
                generateTableOfContents(targetContent);
                
                // Scroll to header if specified
                if (headerId) {
                    setTimeout(() => {
                        const headerElement = document.getElementById(headerId);
                        if (headerElement) {
                            headerElement.scrollIntoView({ behavior: 'smooth', block: 'start' });
                            updateTocActiveState(headerId);
                        }
                    }, 100);
                } else {
                    // Update TOC active state based on scroll position
                    updateTocActiveState();
                }
                
                // Update URL if requested
                if (updateUrl) {
                    const newUrl = headerId ? 
                        window.location.pathname + '#' + contentId + '#' + headerId :
                        window.location.pathname + '#' + contentId;
                    history.pushState({ contentId: contentId, headerId: headerId }, '', newUrl);
                }
            } else {
                console.error('Content not found for ID:', contentId);
                // List all available content sections for debugging
                const allContentIds = Array.from(allSections).map(s => s.id);
                console.log('Available content IDs:', allContentIds);
                // Clear TOC if no content found
                clearTableOfContents();
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

        function generateTableOfContents(contentElement) {
            const tocNav = document.getElementById('toc-nav');
            const tocContainer = document.getElementById('table-of-contents');
            
            if (!contentElement) {
                clearTableOfContents();
                return;
            }
            
            // Find all headers in the content
            const headers = contentElement.querySelectorAll('h1, h2, h3, h4, h5, h6');
            
            if (headers.length === 0) {
                clearTableOfContents();
                return;
            }
            
            // Show the TOC container
            tocContainer.style.display = 'block';
            
            // Build the TOC structure
            let tocHtml = '<ul>';
            headers.forEach(header => {
                const level = parseInt(header.tagName.substring(1));
                const text = header.textContent;
                const id = header.id;
                
                if (id) {
                    const listItem = document.createElement('li');
                    listItem.className = 'toc-level-' + level;
                    
                    const link = document.createElement('a');
                    link.href = '#' + id;
                    link.className = 'toc-link';
                    link.setAttribute('data-header-id', id);
                    link.textContent = text;
                    
                    listItem.appendChild(link);
                    tocHtml += listItem.outerHTML;
                }
            });
            tocHtml += '</ul>';
            
            tocNav.innerHTML = tocHtml;
            
            // Add click handlers for TOC links
            const tocLinks = tocNav.querySelectorAll('.toc-link');
            tocLinks.forEach(link => {
                link.addEventListener('click', function(e) {
                    e.preventDefault();
                    const headerId = this.getAttribute('data-header-id');
                    const headerElement = document.getElementById(headerId);
                    if (headerElement) {
                        headerElement.scrollIntoView({ behavior: 'smooth', block: 'start' });
                        updateTocActiveState(headerId);
                        
                        // Update URL with header
                        const currentContent = document.querySelector('.content-section.active');
                        if (currentContent) {
                            const contentId = currentContent.id;
                            const newUrl = window.location.pathname + '#' + contentId + '#' + headerId;
                            history.pushState({ contentId: contentId, headerId: headerId }, '', newUrl);
                        }
                    }
                });
            });
            
            // Set up scroll spy for TOC
            setupScrollSpy();
        }

        function clearTableOfContents() {
            const tocNav = document.getElementById('toc-nav');
            const tocContainer = document.getElementById('table-of-contents');
            
            if (tocNav) {
                tocNav.innerHTML = '';
            }
            
            if (tocContainer) {
                tocContainer.style.display = 'none';
            }
        }

        function updateTocActiveState(activeHeaderId) {
            const tocLinks = document.querySelectorAll('.toc-link');
            tocLinks.forEach(link => {
                link.classList.remove('active');
                if (activeHeaderId && link.getAttribute('data-header-id') === activeHeaderId) {
                    link.classList.add('active');
                }
            });
        }

        function setupScrollSpy() {
            const headers = document.querySelectorAll('.content-section.active h1, .content-section.active h2, .content-section.active h3, .content-section.active h4, .content-section.active h5, .content-section.active h6');
            
            if (headers.length === 0) return;
            
            // Clear any existing scroll spy
            if (window.currentScrollObserver) {
                window.currentScrollObserver.disconnect();
            }
            
            const observer = new IntersectionObserver((entries) => {
                let activeHeader = null;
                let maxRatio = 0;
                
                // Find the header with the highest intersection ratio
                entries.forEach(entry => {
                    if (entry.isIntersecting && entry.intersectionRatio > maxRatio) {
                        maxRatio = entry.intersectionRatio;
                        activeHeader = entry.target.id;
                    }
                });
                
                // If no header is intersecting, find the one closest to the top
                if (!activeHeader) {
                    let closestHeader = null;
                    let closestDistance = Infinity;
                    
                    headers.forEach(header => {
                        const rect = header.getBoundingClientRect();
                        const distance = Math.abs(rect.top - 100); // Account for sticky header
                        if (distance < closestDistance && rect.top <= window.innerHeight / 2) {
                            closestDistance = distance;
                            closestHeader = header.id;
                        }
                    });
                    
                    activeHeader = closestHeader;
                }
                
                if (activeHeader) {
                    updateTocActiveState(activeHeader);
                    
                    // Ensure active TOC item is visible in mobile collapsed mode
                    const activeTocLink = document.querySelector('.toc-link.active');
                    if (activeTocLink) {
                        const tocContainer = document.getElementById('table-of-contents');
                        if (tocContainer && !tocContainer.classList.contains('collapsed')) {
                            const tocNav = document.getElementById('toc-nav');
                            if (tocNav) {
                                const navRect = tocNav.getBoundingClientRect();
                                const linkRect = activeTocLink.getBoundingClientRect();
                                
                                if (linkRect.bottom > navRect.bottom || linkRect.top < navRect.top) {
                                    activeTocLink.scrollIntoView({ 
                                        behavior: 'smooth', 
                                        block: 'nearest',
                                        inline: 'nearest'
                                    });
                                }
                            }
                        }
                    }
                }
            }, {
                rootMargin: '-80px 0px -50% 0px',
                threshold: [0, 0.1, 0.25, 0.5, 0.75, 1]
            });
            
            headers.forEach(header => {
                if (header.id) {
                    observer.observe(header);
                }
            });
            
            window.currentScrollObserver = observer;
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
                let resultsHtml = '';
                results.forEach(result => {
                    const resultDiv = document.createElement('div');
                    resultDiv.className = 'search-result';
                    resultDiv.onclick = () => showContentFromSearch(result.id);
                    
                    const titleDiv = document.createElement('div');
                    titleDiv.className = 'search-result-title';
                    titleDiv.textContent = result.title;
                    resultDiv.appendChild(titleDiv);
                    
                    const sectionDiv = document.createElement('div');
                    sectionDiv.className = 'search-result-section';
                    sectionDiv.textContent = result.section;
                    resultDiv.appendChild(sectionDiv);
                    
                    if (result.snippet) {
                        const snippetDiv = document.createElement('div');
                        snippetDiv.className = 'search-result-snippet';
                        snippetDiv.innerHTML = result.snippet;
                        resultDiv.appendChild(snippetDiv);
                    }
                    
                    resultsHtml += resultDiv.outerHTML;
                });
                
                searchResultsList.innerHTML = resultsHtml;
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
                // Decode URL-encoded hash (handles %23 -> #)
                const decodedHash = decodeURIComponent(hash);
                
                // Check if this is a header link (contains # in the decoded version)
                const hashParts = decodedHash.split('#');
                if (hashParts.length === 2) {
                    const [contentId, headerId] = hashParts;
                    if (document.getElementById(contentId)) {
                        showContent(contentId, false, headerId);
                        return;
                    }
                } else {
                    // Regular content ID
                    if (document.getElementById(decodedHash)) {
                        showContent(decodedHash, false);
                        return;
                    }
                }
                
                // If hash doesn't match any content, show homepage
                showHomepage();
            } else {
                // No hash - show homepage
                showHomepage();
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

        // Handle TOC mobile toggle
        document.addEventListener('click', function(event) {
            const tocHeader = event.target.closest('.toc-header');
            
            if (tocHeader && window.innerWidth <= 768) {
                const tocContainer = tocHeader.closest('.table-of-contents');
                if (tocContainer) {
                    tocContainer.classList.toggle('collapsed');
                }
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
        });"#);
        
    js
}