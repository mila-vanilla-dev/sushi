# 📝 TODO – SUSHI (Simple Unified Shopping Interface)

This file tracks the development roadmap for **SUSHI**.  
Each section is organized by **phases** and **categories** with short descriptions so contributors know what to work on.

---

## 🟢 Phase 1 – Core Foundations
**Goal: Establish a minimal but functional e-commerce backend and frontend.**

- [ ] **Repository Setup**
  - [ ] Configure backend (`backend/`) and frontend (`frontend/`) structure.
  - [ ] Add `.editorconfig`, `.gitignore`, formatting + linting rules.
  - [ ] Setup CI (GitHub Actions) for tests, builds, and linting.

- [ ] **Backend: Core Ordering System**
  - [ ] Define **PostgreSQL schema**:
    - [ ] Users (customers, admins, business owners).
    - [ ] Products (name, SKU, description, price, inventory).
    - [ ] Orders (status, line items, totals).
    - [ ] Payments (reference, provider, status).
    - [ ] Shipping (address, status, tracking).
  - [ ] Expose REST/GraphQL API endpoints:
    - [ ] CRUD for Products
    - [ ] CRUD for Orders
    - [ ] User auth (sessions/JWT).
  - [ ] Implement inventory deduction when orders are placed.

- [ ] **Frontend: Basic Shop**
  - [ ] Customer-facing product catalog.
  - [ ] Shopping cart (add/remove items).
  - [ ] Checkout flow (address + payment placeholder).
  - [ ] Order confirmation page.

- [ ] **Admin Panel (Basic)**
  - [ ] Admin login.
  - [ ] Product management (add/edit/delete).
  - [ ] Order list with status updates.

---

## 💳 Phase 2 – Integrations
**Goal: Add real-world payments and shipping.**

- [ ] **Payments**
  - [ ] Stripe integration.
  - [ ] PayPal integration.
  - [ ] Abstraction layer for other providers.

- [ ] **Shipping**
  - [ ] UPS integration.
  - [ ] USPS integration.
  - [ ] FedEx integration.
  - [ ] Unified API for shipping cost + label printing.

---

## 🎨 Phase 3 – User Experience Improvements
**Goal: Make SUSHI polished and production-ready.**

- [ ] **Frontend Enhancements**
  - [ ] Responsive design (mobile/tablet).
  - [ ] Customer accounts (profile + order history).
  - [ ] Product search & filtering.
  - [ ] Multi-language support (i18n).

- [ ] **Admin Enhancements**
  - [ ] Analytics dashboard (sales, orders, revenue).
  - [ ] Bulk product import/export (CSV/Excel).
  - [ ] Role-based permissions (staff vs admin).
  - [ ] Inventory management UI.

---

## 🛠️ Phase 4 – Deployment & Scaling
**Goal: Simplify running and scaling SUSHI.**

- [ ] **Deployment Tools**
  - [ ] Docker Compose setup (frontend + backend + PostgreSQL).
  - [ ] Helm chart for Kubernetes.
  - [ ] Self-host installer script (Linux servers).

- [ ] **Performance & Security**
  - [ ] API rate limiting.
  - [ ] Secure session & cookie handling.
  - [ ] HTTPS/TLS setup guide.
  - [ ] Load testing baseline.

---

## 📦 Phase 5 – Managed Instance (Future Option)
**Goal: Offer "instant deployment" for non-technical businesses.**

- [ ] Multi-tenant hosting model.
- [ ] Automated provisioning (domain, SSL, database, instance isolation).
- [ ] Billing for hosted SUSHI instances.

---

## 🧪 Continuous Tasks
- [ ] Write **tests** for backend API (Rust).
- [ ] Write **frontend unit/integration tests**.
- [ ] Improve documentation (`README.md`, `CONTRIBUTING.md`, setup guides).
- [ ] Collect community feedback + reprioritize roadmap.

---
