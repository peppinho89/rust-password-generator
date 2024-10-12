# 🦀 Rust PwdGen

**Rust PwdGen** is a simple **password generator and strength checker** CLI application built with Rust. This project was created for **educational purposes** to practice Rust and explore CLI development. It is not intended for production use but serves as a fun way to learn Rust by building something useful along the way. Plus, it generates QR codes for your passwords!

## ✨ Features

**Generate Secure Passwords**:
- Customize the length and character set (special characters, numbers, uppercase).

**Password Strength Checker**:
- Evaluate the strength of an existing password with a built-in strength assessment.

**QR Code Generation**:
- Generate a QR code for your passwords for easy sharing.

## 🚀 Usage

### 1. Install Rust
First, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

### 2. Clone the repository
```
git clone https://github.com/peppinho89/rust-password-generator.git

cd rust-pwdgen
```

### 3. Run the CLI
Use `cargo` to build and run the CLI application
```
$ cargo run
```

### 4. Generate Password
```
$ cargo run
Choose an option:
1. Generate a password
2. Check password strength
> 1

Enter the desired password length:
> 16
Include special characters? (y/n): y
Include numbers? (y/n): y
Include uppercase letters? (y/n): y
Generated Password: S3cure!Pa$$w0rd
```

### 5. Check Password Strength
```
$ cargo run
Choose an option:
1. Generate a password
2. Check password strength
> 2

Enter the password you want to check:
> S3cure!Pa$$w0rd
Password Strength: Very Strong
```

## ⚠️ Disclaimer
This project is for **educational purposes** only and is **not production-ready**. It is meant to help practice Rust by building a CLI tool along my personal learning journey. Use it as a learning tool, but do not rely on it for critical applications.

## 📝 License
This project is licensed under the MIT License. See the LICENSE file for details.