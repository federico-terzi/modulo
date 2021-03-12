# modulo

> A (very) basic Cross-platform GUI Toolkit for Any Language

Modulo is a simple, distributable binary that can be used to generate a variety of native GUI dialogs with ease:

macOS  |  Linux | Windows
:-------------------------:|:-------------------------:|:-------------------------:
![](/images/macform.png)  |  ![](/images/linuxform.png) | ![](/images/winform.png) 

## Table of Contents

* [Installation](#installation)
    * [Windows](#windows)
    * [macOS](#macos)
    * [Linux](#linux)
* [Getting Started](#getting-started)
* [Technology](#technology)

## Installation

modulo is still in its early days, so some of the details are work in progress.

### Windows

#### espanso users

If you use modulo as part of espanso, it's shipped by default with it since version 0.7.0.

#### Prebuilt release

For Windows x64 systems, you can find the prebuilt `modulo-win.exe` in the [Releases](https://github.com/federico-terzi/modulo/releases) page.

While you can simply download it in your favourite location, if you use modulo as part of espanso you should:

* Rename the executable from `modulo-win.exe` to `modulo.exe`
* Move it to a persistent location, such as `C:\modulo\modulo.exe`
* Add that location (`C:\modulo`) to the PATH environment variable.

**modulo also requires [Visual C++ Redistributable 2019](https://support.microsoft.com/en-us/help/2977003/the-latest-supported-visual-c-downloads) to run.**

#### Compile from source

To compile modulo on Windows, you need a recend Rust compiler, the MSVC C++ compiler and the LLVM compiler.

TODO

### macOS

#### espanso users

If you use modulo as part of espanso and used Homebrew to install it, modulo is automatically included since version 0.7.0.

#### Prebuilt release

For x64 macOS systems, you can find the prebuilt `modulo-mac` in the [Releases](https://github.com/federico-terzi/modulo/releases) page.

While you can simply download it in your favourite location, if you use modulo as part of espanso you should:

* Rename the executable from `modulo-mac` to `modulo`
* Place it in `/usr/local/bin`

#### Compile from source

Compiling from source on macOS requires a few steps:

1. Download the [wxWidgets source archive](https://www.wxwidgets.org/downloads/)
2. Extract the content of the archive in a known directory, such as `$USER/wxWidgets`.
3. Open a terminal, cd into the wxWidgets directory and type the follwing commands:

```
mkdir build-cocoa
cd build-cocoa
../configure --disable-shared --enable-macosx_arch=x86_64
make -j6
```

4. Install LLVM using Homebrew with: `brew install llvm`

5. Now open the `modulo` project directory in the Terminal and compile with: `WXMAC=$HOME/wxWidgets cargo build --release`

6. You will find the compiled binary in the `target/release` directory.

### Linux

#### AppImage

On Linux the easiest way to use modulo is the offical AppImage, that you can find in the [Releases](https://github.com/federico-terzi/modulo/releases) page.
 
#### Compile from source

Compiling modulo is not too difficult, but requires many tools so it's highly suggested to use the AppImage instead. If you still want to compile it:

1. Install the wxWidgets development packages for you platform, the Clang compiler and the Rust compiler. On Ubuntu/Debian, they can be installed with: `sudo apt install clang libwxgtk3.0-0v5 libwxgtk3.0-dev build-essential`.

2. In the project directory run: `cargo build --release`.
3. You will find the compiled binary in the `target/release` directory.

## Getting started

### Creating a Form

There are a variety of built-in dialogs that can be customized by feeding modulo with YAML (or JSON) descriptors:

1. Create a `form.yml` file with the following content:

```yaml
layout: |
  Hey {{name}},
  This form is built with modulo!
```

2. Invoke `modulo` with the command:

```
modulo form -i form.yml
```

3. The dialog will appear:

![Example](images/example1.png)

4. After clicking on `Submit` (or pressing CTRL+Enter), modulo will return to the `STDOUT` the values as JSON:

```json
{"name":"John"}
```

This was a very simple example to get you started, but its only the tip of the iceberg!

### Technology

Modulo is written in Rust and uses the [wxWidgets](https://www.wxwidgets.org/) GUI toolkit under the hoods. This allows modulo to use platform-specific controls that feel, look and behave much better than other solutions based on emulation (such as web-based technologies as Electron), with the additional benefit of a very small final size.

More info coming soon...