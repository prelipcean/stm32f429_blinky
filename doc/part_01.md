# Part 1. Setting Up Your Rust Environment

Welcome! In this chapter, you'll set up everything you need to start developing bare-metal Rust applications for ARM Cortex-M4 microcontrollers, like the STM32F429I-DISC1 board. This guide covers both Arch Linux and Windows Subsystem for Linux (WSL2).

-----

## Prerequisites

Before you begin, make sure you have:

- Basic familiarity with the terminal/command line.
- An STM32F429I-DISC1 board and a micro-USB cable. *(This step is optional—you can use a different board, or use QEMU to test the binaries if you don't have hardware.)*
- Administrative (sudo) access on your computer.

-----

## 1.1 Install rustup

**rustup** is the official tool for installing and managing Rust versions and components. It will help you get Rust ready for embedded development. For more details, visit the official Rust website: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

-----

### 1.1.1 Arch Linux

Install `rustup` using `pacman`:

```bash
sudo pacman -S rustup
```

Then, initialize `rustup` by installing the stable Rust toolchain:

```bash
rustup install stable
```

-----

### 1.1.2 Windows Subsystem for Linux (WSL2)

Run this command in your WSL2 terminal (e.g., Ubuntu, Debian):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and choose the default installation option (option 1). This will set up the stable Rust version.

-----

### 1.1.3 Verify Installation

After installation, restart your terminal or run `source ~/.bashrc` (or `source ~/.zshrc`) to update your environment. Then, check if `rustc` (the Rust compiler) and `cargo` (Rust's package manager) are working:

```bash
rustc --version
cargo --version
```

-----

## 1.2 Add Embedded Targets for Cross-Compiling

To program microcontrollers, you need to "cross-compile" your Rust code. This means compiling code on your computer (e.g., x86) to run on a different type of processor (like the ARM Cortex-M4 on your STM32 board).

List available targets:

```bash
rustup target list
```

For the STM32F429I-DISC1 board (ARM Cortex-M4 with FPU), add the `thumbv7em-none-eabihf` target:

```bash
rustup target add thumbv7em-none-eabihf
```

**Other common targets:**

- **Cortex-M0+ chips:**
  ```bash
  rustup target add thumbv6m-none-eabi
  ```
- **Cortex-M3 chips:**
  ```bash
  rustup target add thumbv7m-none-eabi
  ```
- **Cortex-M4/M7 chips (without FPU):**
  ```bash
  rustup target add thumbv7em-none-eabi
  ```

> **Note:**
> `v7m` / `v7em` refer to ARM architecture versions:
> - **`v7m`**: ARMv7-M (e.g., Cortex-M3)
> - **`v7em`**: ARMv7E-M (Enhanced, e.g., Cortex-M4/M7, adds DSP extensions)

-----

## 1.3 (Optional) Install cargo-generate

**cargo-generate** helps you quickly set up new Rust projects using templates. For this guide, we'll build from scratch, so this step is optional.

```bash
cargo install cargo-generate
```

-----

## 1.4 Install Debugging Tools: probe-rs and arm-none-eabi-gdb

These tools let you flash code to your board and debug it.

-----

### 1.4.1 Install probe-rs (and cargo-embed)

**probe-rs** is a modern debugging tool for many debug probes, including the one built into your STM32 Discovery Kit. **cargo-embed** makes it easy to build, flash, and view debug messages.

Install probe-rs tools:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Verify installation:

```bash
probe-rs --version
cargo-embed --version
cargo-flash --version
```

-----

### 1.4.2 Install arm-none-eabi-gdb

**arm-none-eabi-gdb** is a version of the GNU Debugger for ARM microcontrollers.

On Arch Linux:

```bash
sudo pacman -S arm-none-eabi-gdb
```

On WSL2 (Ubuntu/Debian):

```bash
sudo apt update
sudo apt install gcc-arm-none-eabi
```

Verify installation:

```bash
arm-none-eabi-gdb --version
```

-----

## 1.5 Board Connectivity (Arch Linux / WSL2)

To allow your computer to talk to the STM32F429I-DISC1's built-in debugger (ST-LINK/V2-1) without special permissions, set up udev rules.

First, install helpful tools for identifying USB devices and working with ST-LINK:

On Arch Linux:

```bash
sudo pacman -S openocd stlink usbutils
```

On WSL2 (Ubuntu/Debian):

```bash
sudo apt update
sudo apt install openocd stlink-tools usbutils
```

Identify your ST-LINK device's Vendor ID (`idVendor`) and Product ID (`idProduct`). Connect your STM32F429I-DISC1 board via USB (use the port labeled "USB ST-LINK").

Run:

```bash
lsusb
```

Look for a line like:

```
Bus 001 Device 005: ID 0483:374b STMicroelectronics ST-LINK/V2-1
```

Here, `0483` is the `idVendor` and `374b` is the `idProduct`.

Create a new udev rule file:

```bash
sudo vim /etc/udev/rules.d/99-stlink.rules
```

Add (replace IDs if yours are different):

```udev
# STMicroelectronics ST-LINK/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE="0666"
```

Save and close the file. Reload udev rules:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Verify that `probe-rs` sees your board:

```bash
probe-rs list
```

You should see output like:

```
The following debug probes were found:
[0]: STLink V2-1 -- 0483:374b:066CFF495177514867242523 (ST-LINK)
```

-----

## Troubleshooting

- **Board not detected?**
  - Double-check your USB cable and connection.
  - Make sure you reloaded udev rules and used the correct Vendor/Product IDs.
  - Try reconnecting the board and running `probe-rs list` again.

-----

## Next Steps

If your ST-LINK probe is listed, your environment is all set! In the next chapter, you'll create your first bare-metal Rust project.

-----
