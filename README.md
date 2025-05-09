# cotp - command line TOTP/HOTP authenticator

[![Actions Status](https://github.com/replydev/cotp/workflows/Build/badge.svg)](https://github.com/replydev/cotp/actions)
[![crates.io](https://img.shields.io/crates/v/cotp)](https://crates.io/crates/cotp) 
[![Downloads](https://img.shields.io/crates/d/cotp)](https://crates.io/crates/cotp)

[![Packaging status](https://repology.org/badge/vertical-allrepos/cotp.svg)](https://repology.org/project/cotp/versions)

I believe that security is of paramount importance, especially in this digital world. I created cotp because I needed a
minimalist, secure, desktop accessible software to manage my two-factor authentication codes.

# Overview

## Interface

<p align="center"><img src="/img/cotp_demo.gif?raw=true"/></p>

Type `i` to get some instruction. Otherwise just enter `cotp --help`.
In the first run you will be prompted to insert a password to initialize the database.

## TL;DR

```bash
# Display all the OTP codes in the interactive dashboard
cotp # select any code with arrow keys, press enter to copy into the clipboard, even in an SSH remote shell

# Add a new TOTP code from a BASE32 secret key
cotp add --label myaccount@gmail.com --issuer Google

# Add a new HOTP code with custom algorithm, digits and counter
cotp add --label example --type hotp --algorithm SHA256 --digits 8 --counter 10

# Edit the digits of the 4th OTP code
cotp edit --index 4 --digits 8

# List all the codes in JSON format passing password through stdin
echo "mysecretpassword" | cotp --password-stdin list --json

# Extract the first matching OTP code with "google" issuer and copy it into the clipboard
cotp extract --issuer google --copy-clipboard

# Import an encrypted Aegis Database backup
cotp import --path my_db.json --aegis-encrypted

# Export the cotp database
cotp export
```

## Compatibility

cotp can generate both **TOTP** and **HOTP** codes, compliant with **rfc6238** and **rfc4226** specifications. Also, it is possible to customize settings like **HMAC algorithm** and **digits**, to provide compatibility to other two-factor authentication systems.

Latest releases also include support for **Steam**, **Yandex**, **MOTP** codes.

## Encryption

This program relies on only one database file encrypted
with [XChaCha20Poly1305](https://docs.rs/chacha20poly1305/latest/chacha20poly1305/) authenticated encryption
and [Argon2id](https://en.wikipedia.org/wiki/Argon2) for key derivation.

It also uses [AES-GCM](https://docs.rs/aes-gcm/latest/aes_gcm/) to import from encrypted Aegis backups.


## Cross Platform

cotp should be easily compiled on the most used platforms, but it is mostly tested on Linux, Windows and macOS.

# Install

## Arch Linux / Manjaro / Debian / Ubuntu / NixOS / others...

cotp is distributed in some of the mayor Linux distro repositories. Please check the repology badge at the top to know if your distribution already provides a package.
I will try to push to more repositories over time.

Otherwise you have other options as explained in the next paragraph.

## Other linux distributions and \*nix

Before beginning check that you have the required build dependencies to use the rust compiler.

You need to install the **libxcb-devel** dependency, needed for clipboard coping in X11:

- Debian based: `sudo apt install libxcb1-dev libx11-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev xclip`
- Fedora / RHEL based: `sudo dnf install libX11-devel`
- Void Linux `sudo xbps-install -S libxcb-devel`

## macOS

```
brew install cotp
```

## Windows

Building is supported with both `x86_64-pc-windows-gnu` and `x86_64-pc-windows-msvc` toolchains.

If you want to use `x86_64-pc-windows-msvc` you will need to install
the [Visual C++ 2019 Build Tools](https://visualstudio.microsoft.com/it/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)

Once you have the rust toolchain installed just run `cargo install cotp`.

### Use the crates.io repository

Just type `cargo install cotp` and wait for the installation.

### Clone the GitHub repository and manually install

You can build cotp using these commands:

```
git clone https://github.com/replydev/cotp.git
cargo install --path cotp/
```

# Migration from other apps

cotp supports TOTP codes migration from various apps.
Some needs to be converted using simple python script you can find listed in the table below.

| App                                                                                                          | How to fetch backup                                                                                                                                                 | Needs conversion                                                          | cotp argument               |
|--------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------|-----------------------------|
| [andOTP](https://github.com/andOTP/andOTP)                                                                   | Make a backup using the app itself.                                                                                                                                 | No                                                                        | `--andotp`                  |
| [Aegis](https://github.com/beemdevelopment/Aegis)                                                            | Make a backup using the app itself.                                                                                                                                 | No                                                                        | `--aegis`                   |
| [Aegis](https://github.com/beemdevelopment/Aegis) (encrypted)                                                | Make an encrypted backup using the app itself.                                                                                                                      | No                                                                        | `--aegis-encrypted`         |
| [Authy](https://authy.com/)                                                                                  | Obtain `/data/data/com.authy.authy/shared_prefs/com.authy.storage.tokens.authenticator.xml` from your phone.                                                        | [Yes](https://github.com/replydev/cotp/blob/master/converters/authy.py)   | `--authy`                   |
| [Authy](https://authy.com/) (2nd method)                                                                     | Follow this guide: https://gist.github.com/gboudreau/94bb0c11a6209c82418d01a59d958c93.                                                                              | No                                                                        | `--authy-exported`          |
| [cotp](https://github.com/replydev/cotp)                                                                     | Export your database using `cotp export`.                                                                                                                           | No                                                                        | `--cotp`                    |
| [FreeOTP](https://freeotp.github.io/)                                                                        | Obtain `/data/data/org.fedorahosted.freeotp/shared_prefs/tokens.xml` from your phone.                                                                               | [Yes](https://github.com/replydev/cotp/blob/master/converters/freeotp.py) | `--freeotp`                 |
| [FreeOTP+](https://github.com/helloworld1/FreeOTPPlus)                                                       | Make a backup using the app itself.                                                                                                                                 | No                                                                        | `--freeotp-plus`            |
| [Google Authenticator](https://play.google.com/store/apps/details?id=com.google.android.apps.authenticator2) | Obtain `/data/data/com.google.android.apps.authenticator2/databases/databases` from your phone                                                                      | [Yes](https://github.com/replydev/cotp/blob/master/converters/gauth.py)   | `--google-authenticator`    |
| [Microsoft Authenticator](https://play.google.com/store/apps/details?id=com.azure.authenticator)             | Obtain `/data/data/com.azure.authenticator/databases/PhoneFactor` from your phone. Take also `PhoneFactor-wal`, `PhoneFactor-shm` if they exist in the same folder. | [Yes](https://github.com/replydev/cotp/blob/master/converters/mauth.py)   | `--microsoft-authenticator` |
| [OTP URI list](https://docs.yubico.com/yesdk/users-manual/application-oath/uri-string-format.html)           | Create a JSON file which contains a items property. It will contains a string array where each element is an OTP URI.                                               | No                                                                        | `--otp-uri`                 |

## How to convert

Once you got the correct files run the right python script located in the **converters/** folder in this source code.

**Example:**
`python authy.py path/to/database.xml converted.json`

It will convert the database in a json format readable by cotp.

To terminate the import:
`cotp import --authy --path path/to/converted_database.json`

# Configuration

By default database is located in `$HOME/.cotp/db.cotp`. If you want to use a different path, you can use `COTP_DB_PATH` environment variable or use the `--database-path` argument.
The first can be configured in shell configuration files, the second in package managers where the configuration of environment variables is not allowed, like Scoop.

These are examples of how to do this in bash:
```bash
$ COTP_DB_PATH=/home/user/.local/custom-folder/db.cotp cotp
```
or
```bash
$ cotp --database-path /home/user/.local/custom-folder/db.cotp
```

# Planned features

Currently, there is not any planned feature. If you need something new that could improve the software feel free to open
an issue.

# Contribution

I created this project for my own needs, but I would be happy if this little program is useful to someone else, and I
gratefully accept any pull requests.
