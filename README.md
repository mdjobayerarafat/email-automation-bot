<div align="center">

# 📮 Email Automation Bot

<img src="https://raw.githubusercontent.com/yourusername/email-automation-bot/main/src/assets/email-icon.svg" alt="Email Icon" width="120" height="120">

**A powerful cross-platform desktop application for automated email workflows**

*Built with Rust, Tauri, React, and SQLite3*

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript">
  <img src="https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB" alt="React">
  <img src="https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri">
  <img src="https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white" alt="SQLite">
</p>

</div>

---

## ✨ Features

### 🔐 Authentication & Security
- Secure user authentication with JWT tokens
- Password hashing with bcrypt/argon2
- Session management

### 📧 Email Management
- **Multiple Email Account Support**: Connect and manage multiple email accounts
- **SMTP Configuration**: Easy setup for various email providers
- **Connection Testing**: Verify email account configurations
- **Email Composition**: Rich email composer with template support

### 📝 Template System
- **Custom Templates**: Create and manage reusable email templates
- **Variable Support**: Dynamic content with template variables
- **HTML & Text Content**: Support for both HTML and plain text emails

### ⚡ Automation Rules
- **Scheduled Emails**: Set up automated email sending with cron expressions
- **Rule-based Automation**: Create complex automation workflows
- **Monitoring Dashboard**: Track automation performance

### 📊 Dashboard & Analytics
- **Real-time Statistics**: Monitor email sending statistics
- **Account Overview**: Quick view of all connected accounts
- **Template Management**: Easy access to all templates
- **Rule Monitoring**: Track automation rule performance

### 🎨 Modern UI/UX
- **Dark/Light Theme**: Toggle between themes
- **Responsive Design**: Works on different screen sizes
- **Loading Animations**: Smooth Lottie animations
- **Intuitive Navigation**: Clean sidebar navigation

## 🖼️ Screenshots

### Dashboard Overview
![Dashboard](demo/Screenshot%202025-07-31%20013706.png)
*Main dashboard showing email statistics and quick access to features*

### Email Accounts Management
![Email Accounts](demo/Screenshot%202025-07-31%20013713.png)
*Manage multiple email accounts with SMTP configuration*

### Email Templates
![Email Templates](demo/Screenshot%202025-07-31%20013738.png)
*Create and manage reusable email templates*

### Compose Email
![Compose Email](demo/Screenshot%202025-07-31%20013753.png)
*Rich email composer with template integration*

### Automation Rules
![Automation Rules](demo/Screenshot%202025-07-31%20013801.png)
*Set up automated email workflows*

### Settings & Configuration
![Settings](demo/Screenshot%202025-07-31%20013819.png)
*Application settings and configuration options*

### Documentation
![Documentation](demo/Screenshot%202025-07-31%20013828.png)
*Built-in documentation and help system*

## 🛠️ Tech Stack

### Frontend
- **React 18** - Modern React with hooks
- **TypeScript** - Type-safe development
- **Vite** - Fast build tool and dev server
- **Lottie React** - Beautiful animations
- **CSS3** - Modern styling with CSS variables

### Backend
- **Rust** - High-performance backend
- **Tauri** - Secure desktop app framework
- **SQLite** - Local database with rusqlite
- **Tokio** - Async runtime
- **Lettre** - Email sending library
- **IMAP** - Email receiving capabilities

### Security & Authentication
- **JWT** - JSON Web Tokens for authentication
- **bcrypt/argon2** - Password hashing
- **Secure storage** - Local encrypted data storage

## 🚀 Getting Started

### Prerequisites

- **Node.js** (v18 or higher)
- **Rust** (latest stable version)
- **Visual Studio Build Tools** (Windows) or equivalent build tools

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/email-automation-bot.git
   cd email-automation-bot
   ```

2. **Install frontend dependencies**
   ```bash
   npm install
   ```

3. **Install Rust dependencies**
   ```bash
   cd src-tauri
   cargo build
   ```

4. **Set up environment variables**
   ```bash
   # Copy environment template
   cp .env.example .env
   cp src-tauri/.env.example src-tauri/.env
   
   # Edit the .env files with your configuration
   ```

### Development

1. **Start the development server**
   ```bash
   cargo tauri dev
   ```
   This will start both the Vite dev server and the Tauri application.

2. **Frontend only development**
   ```bash
   npm run dev
   ```

### Building for Production

1. **Build the application**
   ```bash
   cargo tauri build
   ```

2. **The built application will be available in:**
   - Windows: `src-tauri/target/release/bundle/msi/`
   - macOS: `src-tauri/target/release/bundle/dmg/`
   - Linux: `src-tauri/target/release/bundle/deb/` or `src-tauri/target/release/bundle/appimage/`

## 📋 Configuration

### Email Provider Setup

The application supports various email providers. Here are common SMTP configurations:

#### Gmail
- **SMTP Server**: smtp.gmail.com
- **Port**: 587 (TLS) or 465 (SSL)
- **Security**: TLS/SSL
- **Note**: Use App Passwords for 2FA-enabled accounts

#### Outlook/Hotmail
- **SMTP Server**: smtp-mail.outlook.com
- **Port**: 587
- **Security**: TLS

#### Custom SMTP
- Configure your own SMTP server settings
- Support for custom ports and security protocols

### Database

The application uses SQLite for local data storage:
- User accounts and authentication
- Email account configurations (encrypted)
- Templates and automation rules
- Email sending history and statistics

## 🔧 Development Setup

### Project Structure
```
email-automation-bot/
├── src/                    # React frontend source
│   ├── components/         # Reusable UI components
│   ├── pages/             # Application pages
│   ├── hooks/             # Custom React hooks
│   ├── contexts/          # React contexts
│   ├── types/             # TypeScript type definitions
│   └── assets/            # Static assets and animations
├── src-tauri/             # Rust backend source
│   ├── src/               # Rust source code
│   ├── migrations/        # Database migrations
│   └── templates/         # Email templates
├── demo/                  # Application screenshots
└── dist/                  # Built frontend assets
```

### Available Scripts

- `npm run dev` - Start Vite development server
- `npm run build` - Build frontend for production
- `npm run preview` - Preview production build
- `cargo tauri dev` - Start full development environment
- `cargo tauri build` - Build application for production

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🐛 Troubleshooting

### Common Issues

1. **Build Tools Error (Windows)**
   - Install Visual Studio Build Tools with C++ workload
   - Ensure MSVC toolchain is installed

2. **Port Already in Use**
   - Kill processes using ports 1420/1421
   - Use `netstat -ano | findstr :1420` to find processes

3. **Email Connection Issues**
   - Verify SMTP settings
   - Check firewall and antivirus settings
   - Ensure app passwords are used for 2FA accounts

4. **Database Issues**
   - Delete `src-tauri/database.db` to reset database
   - Check file permissions

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/email-automation-bot/issues) page
2. Create a new issue with detailed information
3. Include error logs and system information

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing desktop app framework
- [React](https://reactjs.org/) - For the powerful UI library
- [Rust](https://www.rust-lang.org/) - For the safe and fast backend
- [Lettre](https://lettre.rs/) - For email sending capabilities
- [Lottie](https://lottiefiles.com/) - For beautiful animations

---

**Made with ❤️ using Tauri, React, and Rust**
