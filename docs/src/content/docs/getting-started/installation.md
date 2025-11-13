---
title: Installation
description: How to install tedlt on your system
---

## Download

Download the latest release for your platform from the [GitHub releases page](https://github.com/brequet/tedlt/releases/latest).

### Available Platforms

tedlt provides pre-built binaries for the following platforms:

- **Windows** (x64)
- **Linux** (x64)
- **macOS** (x64 and ARM64)

## Installation Steps

### Windows

1. Download `tedlt-windows-x64.zip` from the releases page
2. Extract the archive
3. Move `tedlt.exe` to a directory in your PATH (e.g., `C:\Program Files\tedlt\`)
4. Open a new terminal and verify installation:
   ```sh
   tedlt --version
   ```

#### Adding to PATH (Windows)

If the command isn't found, add the directory to your PATH:

1. Search for "Environment Variables" in Windows settings
2. Click "Edit the system environment variables"
3. Click "Environment Variables"
4. Under "User variables", select "Path" and click "Edit"
5. Click "New" and add the directory containing `tedlt.exe`
6. Click "OK" on all dialogs
7. Open a new terminal window

### macOS

1. Download the appropriate file for your Mac:
   - **Intel Macs**: `tedlt-macos-x64.tar.gz`
   - **Apple Silicon (M1/M2/M3)**: `tedlt-macos-arm64.tar.gz`
2. Extract the archive:
   ```sh
   tar -xzf tedlt-macos-*.tar.gz
   ```
3. Move the binary to a directory in your PATH:
   ```sh
   sudo mv tedlt /usr/local/bin/
   ```
4. Make it executable:
   ```sh
   chmod +x /usr/local/bin/tedlt
   ```
5. Verify installation:
   ```sh
   tedlt --version
   ```

#### macOS Security Note

The first time you run tedlt, macOS may block it because it's not from an identified developer. To allow it:

1. Open System Preferences → Security & Privacy
2. Click "Allow Anyway" next to the message about tedlt
3. Run tedlt again and click "Open" when prompted

### Linux

1. Download `tedlt-linux-x64.tar.gz` from the releases page
2. Extract the archive:
   ```sh
   tar -xzf tedlt-linux-x64.tar.gz
   ```
3. Move the binary to a directory in your PATH:
   ```sh
   sudo mv tedlt /usr/local/bin/
   ```
4. Make it executable:
   ```sh
   chmod +x /usr/local/bin/tedlt
   ```
5. Verify installation:
   ```sh
   tedlt --version
   ```

## Building from Source

If you prefer to build from source or your platform isn't supported:

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)

### Build Steps

1. Clone the repository:
   ```sh
   git clone https://github.com/brequet/tedlt.git
   cd tedlt
   ```

2. Build the release binary:
   ```sh
   cargo build --release
   ```

3. The binary will be at `target/release/tedlt` (or `tedlt.exe` on Windows)

4. Move it to a directory in your PATH:
   ```sh
   # Linux/macOS
   sudo mv target/release/tedlt /usr/local/bin/

   # Windows (PowerShell as Administrator)
   Move-Item target\release\tedlt.exe "C:\Program Files\tedlt\"
   ```

## Verify Installation

After installation, verify tedlt is working:

```sh
tedlt --version
```

You should see output like:
```
tedlt 0.1.4
```

## Next Steps

Now that tedlt is installed, continue to the [Quick Start Guide](/tedlt/getting-started/quick-start/) to set up your Jira credentials and create your first ticket.