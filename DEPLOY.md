# PumpChain Deployment Guide

## 🌐 Domain: www.pumpchain.org

### Option 1: Vercel Deployment (Recommended)

1. **Install Vercel CLI:**
```bash
npm i -g vercel
```

2. **Deploy:**
```bash
cd frontend/public
vercel --prod
```

3. **Add Domain:**
- Go to Vercel Dashboard → Project Settings → Domains
- Add `www.pumpchain.org`
- Follow DNS configuration instructions

### Option 2: Cloudflare Pages

1. **Build:**
```bash
cd frontend/public
# No build needed for static HTML
```

2. **Deploy via Dashboard:**
- Connect GitHub repo to Cloudflare Pages
- Build command: `echo "No build"`
- Output directory: `frontend/public`

### Option 3: GitHub Pages

1. **Enable Pages:**
- Repo Settings → Pages
- Source: Deploy from branch
- Branch: `gh-pages` / `/frontend/public`

2. **Custom Domain:**
- Add `www.pumpchain.org` in Pages settings
- Add CNAME file in public folder

---

## 🔧 DNS Configuration

### For Vercel:
```
Type: CNAME
Name: www
Value: cname.vercel-dns.com
TTL: 3600
```

### For Root Domain (pumpchain.org → www.pumpchain.org):
```
Type: ALIAS or ANAME
Name: @
Value: www.pumpchain.org
TTL: 3600
```

Or use redirect:
```
Type: URL Redirect
Name: @
Value: https://www.pumpchain.org
```

---

## 🚀 Quick Deploy Script

```bash
#!/bin/bash
# deploy.sh

echo "🚀 Deploying PumpChain to pumpchain.org..."

cd frontend/public

# Deploy to Vercel
vercel --prod --confirm

echo "✅ Deployment complete!"
echo "🌐 Check: https://www.pumpchain.org"
```

---

## 📋 SSL Certificate

Vercel/Cloudflare/GitHub Pages otomatik SSL sağlar.

Manuel kontrol:
```bash
curl -I https://www.pumpchain.org
```

---

## 🔄 CI/CD Pipeline (GitHub Actions)

Already configured in `.github/workflows/ci.yml`

Auto-deploy on push to main:
```yaml
- name: Deploy to Vercel
  uses: amondnet/vercel-action@v25
  with:
    vercel-token: ${{ secrets.VERCEL_TOKEN }}
    vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
    vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
    working-directory: ./frontend/public
```

---

## 🎯 Post-Deploy Checklist

- [ ] DNS propagation complete (check with `dig www.pumpchain.org`)
- [ ] SSL certificate active
- [ ] Mobile responsive test
- [ ] Social media meta tags working
- [ ] Analytics connected (optional)
- [ ] Favicon showing

---

Need help? Contact: team@pumpchain.io