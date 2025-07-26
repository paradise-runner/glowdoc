pub fn generate_mobile_css() -> String {
    r#"        @media (max-width: 768px) {
            .nav {
                gap: 1rem;
            }

            .mobile-menu-toggle {
                display: block;
            }

            .layout {
                flex-direction: column;
            }

            .sidebar {
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
            }

            .sidebar.visible {
                transform: translateX(0);
            }

            // .main-content {
            //     padding: 1rem;
            // }

            .content-wrapper {
                flex-direction: column;
                gap-top: 1rem;
                margin: 1rem;
            }

            .table-of-contents {
                position: static;
                top: auto;
                right: auto;
                width: auto;
                max-height: 40vh;
                order: -1;
                // margin-bottom: 1rem;
                margin-top: 0;
                z-index: 20;
            }

            .toc-header {
                cursor: pointer;
                user-select: none;
                display: flex;
                justify-content: space-between;
                align-items: center;
            }

            .toc-header::after {
                content: '▼';
                font-size: 0.75rem;
                transition: transform 0.2s ease;
            }

            .table-of-contents.collapsed .toc-header::after {
                transform: rotate(-90deg);
            }

            .table-of-contents.collapsed .toc-nav {
                display: none;
            }

            .content-area {
                max-width: none;
                padding-right: 1rem;
                padding-left: 1rem;
            }

            .social-links {
                margin-left: 0;
                gap: 0.25rem;
            }

            .social-link {
                width: 1.75rem;
                height: 1.75rem;
            }

            .social-link svg {
                width: 14px;
                height: 14px;
            }
        }"#.to_string()
}