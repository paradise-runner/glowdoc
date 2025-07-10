# Deployment

Comprehensive guide to deploying your GlowDoc documentation site across various hosting platforms, from simple static hosting to advanced CI/CD pipelines.

## Overview

GlowDoc generates a single `index.html` file containing your entire documentation site, making deployment simple and flexible. This approach offers several advantages:

- **Single File Deployment**: No complex directory structures or dependencies
- **Universal Compatibility**: Works with any static hosting service
- **Fast Loading**: All assets embedded, no additional HTTP requests
- **Easy Backup**: Single file contains everything
- **CDN Friendly**: Perfect for content delivery networks

## Pre-Deployment Checklist

Before deploying your documentation:

### 1. Build Verification

```bash
# Build your documentation
cargo run --release

# Verify the build succeeded
ls -la index.html

# Test locally
python3 -m http.server 8000
# Visit http://localhost:8000 to verify everything works
```

### 2. Content Review

- [ ] All pages load correctly
- [ ] Navigation works properly
- [ ] Search functionality operates
- [ ] All themes (light/dark) display correctly
- [ ] Mobile responsive design works
- [ ] All links are functional

### 3. Performance Optimization

```bash
# Check file size (typical range: 500KB - 2MB)
du -h index.html

# Optional: Minify if needed (for very large sites)
# Note: GlowDoc output is already optimized
```

## Static Hosting Platforms

### GitHub Pages

Deploy directly from your GitHub repository with automated builds.

#### Method 1: GitHub Actions (Recommended)

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy GlowDoc

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Cache Cargo dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Build documentation
      run: cargo run --release
    
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      if: github.ref == 'refs/heads/main'
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: .
        publish_branch: gh-pages
        force_orphan: true
        enable_jekyll: false
        exclude_assets: |
          .github
          .gitignore
          Cargo.toml
          Cargo.lock
          src
          docs
          target
          README.md
```

#### Method 2: Manual Upload

```bash
# Build locally
cargo run --release

# Create gh-pages branch
git checkout --orphan gh-pages
git rm -rf .
git add index.html
git commit -m "Deploy documentation"
git push origin gh-pages

# Return to main branch
git checkout main
```

#### GitHub Pages Configuration

1. Go to your repository → Settings → Pages
2. Set Source to "Deploy from a branch"
3. Select `gh-pages` branch
4. Choose `/ (root)` folder
5. Save settings

**Custom Domain Setup:**
```bash
# Add CNAME file to repository root
echo "docs.yoursite.com" > CNAME
git add CNAME
git commit -m "Add custom domain"
git push
```

### Netlify

Professional hosting with advanced features and global CDN.

#### Method 1: Git Integration (Recommended)

1. **Connect Repository:**
   - Sign up at [netlify.com](https://netlify.com)
   - Click "New site from Git"
   - Connect your GitHub/GitLab repository

2. **Build Configuration:**
   ```toml
   # netlify.toml
   [build]
     command = "cargo run --release"
     publish = "."
   
   [build.environment]
     RUST_VERSION = "1.70"
   
   [[headers]]
     for = "/*"
     [headers.values]
       X-Frame-Options = "DENY"
       X-XSS-Protection = "1; mode=block"
       X-Content-Type-Options = "nosniff"
       Referrer-Policy = "strict-origin-when-cross-origin"
   
   [[redirects]]
     from = "/docs/*"
     to = "/#:splat"
     status = 200
   ```

3. **Deploy Settings:**
   - Build command: `cargo run --release`
   - Publish directory: `.` (root)
   - Node version: Latest LTS

#### Method 2: Manual Upload

```bash
# Build documentation
cargo run --release

# Deploy via Netlify CLI
npm install -g netlify-cli
netlify login
netlify deploy --prod --dir=.
```

#### Advanced Netlify Features

**Form Handling:**
```html
<!-- Add to your documentation for feedback forms -->
<form name="feedback" method="POST" data-netlify="true">
  <input type="hidden" name="form-name" value="feedback" />
  <input type="text" name="name" placeholder="Your name" required />
  <textarea name="message" placeholder="Feedback" required></textarea>
  <button type="submit">Send Feedback</button>
</form>
```

**Analytics Integration:**
```javascript
// Add to your custom JavaScript
if (window.netlifyIdentity) {
  window.netlifyIdentity.on('init', user => {
    if (!user) {
      window.netlifyIdentity.on('login', () => {
        document.location.href = '/admin/';
      });
    }
  });
}
```

### Vercel

Zero-configuration deployment with excellent performance.

#### Method 1: Git Integration

1. **Connect Repository:**
   - Sign up at [vercel.com](https://vercel.com)
   - Import your Git repository
   - Vercel auto-detects the setup

2. **Configuration File:**
   ```json
   {
     "version": 2,
     "name": "glowdoc-docs",
     "builds": [
       {
         "src": "package.json",
         "use": "@vercel/static-build"
       }
     ],
     "routes": [
       {
         "src": "/(.*)",
         "dest": "/index.html"
       }
     ],
     "env": {
       "RUST_VERSION": "1.70"
     }
   }
   ```

3. **Package.json for Build:**
   ```json
   {
     "name": "glowdoc-site",
     "scripts": {
       "build": "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source ~/.cargo/env && cargo run --release"
     }
   }
   ```

#### Method 2: Vercel CLI

```bash
# Install Vercel CLI
npm install -g vercel

# Build and deploy
cargo run --release
vercel --prod
```

### Cloudflare Pages

High-performance hosting with global edge network.

#### Setup Process:

1. **Connect Repository:**
   - Sign up at [pages.cloudflare.com](https://pages.cloudflare.com)
   - Connect your Git repository

2. **Build Configuration:**
   - Build command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source ~/.cargo/env && cargo run --release`
   - Build output directory: `.`
   - Environment variables: `RUST_VERSION=1.70`

3. **Custom Domains:**
   ```bash
   # Configure custom domain in Cloudflare Dashboard
   # DNS automatically managed
   ```

### Firebase Hosting

Google's hosting platform with global CDN.

#### Setup Process:

```bash
# Install Firebase CLI
npm install -g firebase-tools

# Initialize Firebase
firebase login
firebase init hosting

# Configure firebase.json
```

```json
{
  "hosting": {
    "public": ".",
    "ignore": [
      "firebase.json",
      "**/.*",
      "**/node_modules/**",
      "src/**",
      "docs/**",
      "target/**"
    ],
    "rewrites": [
      {
        "source": "**",
        "destination": "/index.html"
      }
    ],
    "headers": [
      {
        "source": "**/*.@(js|css)",
        "headers": [
          {
            "key": "Cache-Control",
            "value": "max-age=31536000"
          }
        ]
      }
    ]
  }
}
```

```bash
# Build and deploy
cargo run --release
firebase deploy
```

## Traditional Web Hosting

### Apache Configuration

For traditional web hosting with Apache:

```apache
# .htaccess
RewriteEngine On

# Handle client-side routing
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteRule . /index.html [L]

# Security headers
Header always set X-Frame-Options DENY
Header always set X-Content-Type-Options nosniff
Header always set X-XSS-Protection "1; mode=block"
Header always set Strict-Transport-Security "max-age=31536000; includeSubDomains"

# Compression
<IfModule mod_deflate.c>
    AddOutputFilterByType DEFLATE text/html text/css application/javascript
</IfModule>

# Caching
<IfModule mod_expires.c>
    ExpiresActive On
    ExpiresByType text/html "access plus 1 hour"
    ExpiresByType text/css "access plus 1 year"
    ExpiresByType application/javascript "access plus 1 year"
</IfModule>
```

### Nginx Configuration

For Nginx hosting:

```nginx
server {
    listen 80;
    listen [::]:80;
    server_name yourdomain.com;
    
    # Redirect HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name yourdomain.com;
    
    # SSL configuration
    ssl_certificate /path/to/certificate.crt;
    ssl_certificate_key /path/to/private.key;
    
    # Document root
    root /var/www/glowdoc;
    index index.html;
    
    # Handle client-side routing
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains";
    
    # Compression
    gzip on;
    gzip_types text/html text/css application/javascript;
    
    # Caching
    location ~* \.(css|js)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

## Content Delivery Networks (CDN)

### Cloudflare CDN

Enhance performance with Cloudflare:

1. **DNS Setup:**
   - Add your domain to Cloudflare
   - Update nameservers
   - Enable "Proxied" status

2. **Optimization Settings:**
   - Auto Minify: HTML, CSS, JS
   - Brotli compression: Enabled
   - Rocket Loader: Enabled
   - Cache Level: Standard

3. **Page Rules:**
   ```
   yourdomain.com/*
   - Cache Level: Cache Everything
   - Edge Cache TTL: 1 month
   - Browser Cache TTL: 1 day
   ```

### AWS CloudFront

Enterprise-grade CDN with AWS integration:

```json
{
  "Distribution": {
    "Origins": [
      {
        "Id": "S3-glowdoc",
        "DomainName": "your-bucket.s3.amazonaws.com",
        "S3OriginConfig": {
          "OriginAccessIdentity": ""
        }
      }
    ],
    "DefaultCacheBehavior": {
      "TargetOriginId": "S3-glowdoc",
      "ViewerProtocolPolicy": "redirect-to-https",
      "Compress": true,
      "CachePolicyId": "managed-caching-optimized"
    },
    "CustomErrorResponses": [
      {
        "ErrorCode": 404,
        "ResponseCode": 200,
        "ResponsePagePath": "/index.html"
      }
    ]
  }
}
```

## Automation and CI/CD

### GitHub Actions Advanced Workflow

Complete CI/CD pipeline with testing and deployment:

```yaml
name: Build, Test, and Deploy

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Build documentation
      run: cargo run --release
    
    - name: Validate HTML
      run: |
        npm install -g html-validate
        html-validate index.html
    
    - name: Check file size
      run: |
        SIZE=$(stat -c%s index.html)
        echo "Generated file size: $SIZE bytes"
        if [ $SIZE -gt 5242880 ]; then
          echo "Warning: File size exceeds 5MB"
          exit 1
        fi
    
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: documentation
        path: index.html

  deploy-staging:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/develop'
    steps:
    - name: Download artifacts
      uses: actions/download-artifact@v3
      with:
        name: documentation
    
    - name: Deploy to staging
      run: |
        # Deploy to staging environment
        echo "Deploying to staging..."

  deploy-production:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
    - name: Download artifacts
      uses: actions/download-artifact@v3
      with:
        name: documentation
    
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: .
        force_orphan: true
    
    - name: Notify deployment
      run: |
        curl -X POST ${{ secrets.SLACK_WEBHOOK }} \
          -H 'Content-type: application/json' \
          --data '{"text":"📚 Documentation deployed successfully!"}'
```

### GitLab CI/CD

```yaml
# .gitlab-ci.yml
stages:
  - test
  - build
  - deploy

variables:
  RUST_VERSION: "1.70"

before_script:
  - apt-get update -qq
  - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  - source ~/.cargo/env

test:
  stage: test
  script:
    - cargo test --verbose
    - cargo fmt -- --check
    - cargo clippy -- -D warnings
  only:
    - merge_requests
    - main

build:
  stage: build
  script:
    - cargo run --release
  artifacts:
    paths:
      - index.html
    expire_in: 1 hour
  only:
    - main

pages:
  stage: deploy
  script:
    - mkdir public
    - cp index.html public/
  artifacts:
    paths:
      - public
  only:
    - main
```

## Domain and SSL Configuration

### Custom Domain Setup

#### DNS Configuration:

```
# A Records for root domain
@ 3600 IN A 185.199.108.153
@ 3600 IN A 185.199.109.153
@ 3600 IN A 185.199.110.153
@ 3600 IN A 185.199.111.153

# CNAME for www subdomain
www 3600 IN CNAME your-username.github.io.

# CNAME for docs subdomain
docs 3600 IN CNAME your-site.netlify.app.
```

#### SSL Certificate Setup:

Most modern hosting platforms provide automatic SSL:

- **GitHub Pages**: Automatic with custom domains
- **Netlify**: Automatic Let's Encrypt certificates
- **Vercel**: Automatic SSL for all deployments
- **Cloudflare**: Universal SSL included

### Manual SSL Configuration

For traditional hosting:

```bash
# Generate Let's Encrypt certificate
certbot certonly --webroot -w /var/www/glowdoc -d yourdomain.com

# Auto-renewal
echo "0 12 * * * /usr/bin/certbot renew --quiet" | crontab -
```

## Performance Optimization

### Build Optimization

```bash
# Optimize for production
RUSTFLAGS="-C target-cpu=native" cargo run --release

# Profile build performance
cargo build --release --timings
```

### Content Optimization

1. **Image Optimization**: Use optimized images in markdown
2. **Font Subsetting**: Include only needed font weights
3. **Code Splitting**: Implement lazy loading for large sections

### Monitoring and Analytics

#### Performance Monitoring:

```javascript
// Add to your custom JavaScript
function trackPerformance() {
  window.addEventListener('load', () => {
    const perfData = performance.timing;
    const loadTime = perfData.loadEventEnd - perfData.navigationStart;
    
    // Send to analytics
    gtag('event', 'page_load_time', {
      value: loadTime,
      custom_parameter: 'documentation_site'
    });
  });
}
```

#### Uptime Monitoring:

Set up monitoring with services like:
- **StatusCake**: Free uptime monitoring
- **Pingdom**: Comprehensive monitoring suite
- **UptimeRobot**: Free and paid monitoring options

## Security Considerations

### Content Security Policy

```html
<!-- Add to your HTML head -->
<meta http-equiv="Content-Security-Policy" content="
  default-src 'self';
  script-src 'self' 'unsafe-inline' 'unsafe-eval' https://www.googletagmanager.com;
  style-src 'self' 'unsafe-inline' https://fonts.googleapis.com;
  font-src 'self' https://fonts.gstatic.com;
  img-src 'self' data: https:;
  connect-src 'self' https://www.google-analytics.com;
">
```

### Security Headers

Implement security headers across all hosting platforms:

```
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Referrer-Policy: strict-origin-when-cross-origin
```

## Troubleshooting Deployment Issues

### Common Problems

**Build Failures:**
```bash
# Check Rust version compatibility
rustc --version

# Verify dependencies
cargo check

# Clean and rebuild
cargo clean && cargo run --release
```

**Routing Issues:**
- Ensure hosting platform supports SPA routing
- Configure redirects for client-side routing
- Verify base URL configuration

**Performance Issues:**
- Check file size (should be under 5MB)
- Verify compression is enabled
- Test CDN configuration

**SSL Certificate Problems:**
- Verify DNS propagation
- Check certificate chain
- Ensure HTTPS redirects are configured

### Debug Deployment

```bash
# Test local build
cargo run --release
python3 -m http.server 8000

# Validate HTML
html-validate index.html

# Check file permissions
ls -la index.html

# Test from different networks
curl -I https://yourdomain.com
```

## Best Practices

1. **Version Control**: Always commit before deploying
2. **Staging Environment**: Test changes before production
3. **Automated Backups**: Regular backup of source files
4. **Performance Monitoring**: Track load times and uptime
5. **Security Updates**: Keep hosting platform updated
6. **Documentation**: Document deployment process for team
7. **Rollback Plan**: Maintain ability to quickly revert changes

This comprehensive deployment guide ensures your GlowDoc documentation is accessible, performant, and secure across any hosting platform.