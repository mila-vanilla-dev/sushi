# 🍣 SUSHI – Simple Unified Shopping Interface

> Formerly `tps-orders-api`

**SUSHI** is a clean, free, open-source, and optimized drop-in solution to put small to large businesses online.\
It allows customers to place orders with automatic **shipping** and **payment integration**, while giving businesses a **unified, self-hosted platform** to manage e-commerce needs.

______________________________________________________________________

## ✨ Features

- 🟢 **Simple** – easy to deploy and use.
- 🔗 **Unified** – integrates orders, shipping, and payments in one place.
- 🛒 **Shopping** – full e-commerce capabilities, ready for real-world use.
- 🖥️ **Interface** – clean front-end experience for customers and admins.
- 🛠️ **Self-hosted** – isolate data per instance, full control and ownership.
- 📦 **Future option** – instant deployment service for non-technical businesses.

______________________________________________________________________

## 🏗️ Tech Stack

- **Frontend:** [Next.js](https://nextjs.org/)
- **Backend:** [Rust](https://www.rust-lang.org/)
- **Database:** Self-hosted PostgreSQL per instance (isolated by design)
- **Hosting:** Always self-hosted

______________________________________________________________________

## 🚀 Getting Started

### Prerequisites

- Node.js (for the Next.js frontend)
- Rust (for the backend API)
- PostgreSQL or another supported database
- Docker (optional, for deployment)

### Installation

```bash
# Clone the repo
git clone https://github.com/ofluffydev/sushi.git
cd sushi

# Setup backend
cd backend
cargo build --release

# Setup frontend
cd ../frontend
npm install
npm run build
```

______________________________________________________________________

## 🧩 Roadmap

- [ ] Core ordering system
- [ ] Payment integration (Stripe, PayPal, etc.)
- [ ] Shipping integration (UPS, USPS, FedEx, etc.)
- [ ] Admin dashboard
- [ ] Deployment helper for instant managed instances

<!-- See [TODO.md](TODO.md) for details. -->

______________________________________________________________________

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!
Feel free to open a PR or start a discussion.

______________________________________________________________________

## 📜 License

SUSHI is made available under a **permissive dual license**:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

You may use either license at your option.

______________________________________________________________________

⚖️ **Note:** Attribution such as “Powered by SUSHI” is **appreciated but not required**.\
This project is open source to maximize adoption and flexibility.
