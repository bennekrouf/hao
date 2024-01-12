# Hao


## Overview

This is a Rust-based Terminal User Interface (TUI) application using the ratatui and crossterm libraries. It dynamically creates a menu to execute .sh scripts from a specified directory, offering a straightforward and interactive script-running experience.
Prerequisites

- Rust Programming Language:
    
    Ensure you have Rust installed on your system. If not, you can install it from the official Rust website.

## Setup

    Scripts Directory: Create a scripts directory in the root of this application.
    Add Shell Scripts: Place your .sh shell scripts in the scripts folder. Each script file name becomes an option in the TUI menu.


## Building the Program

To build the program, follow these steps:

    Clone the Repository: Clone or download the source code to your local machine.
    Navigate to the Directory: Open a terminal and navigate to the directory containing the program's source code.
    Build with Cargo: Run the following command:

```sh
    cargo build --release
```


This command compiles the program and generates an executable in the target/release directory.

## Running the Program

After building, you can run the program directly from the command line:

    Navigate to the target/release directory.
    Run the executable:

```sh
    ./hao
```

### Using Hao

    Use the up and down arrow keys to navigate through the list of scripts.
    Press Enter to execute the selected script.
    Select "Exit" or press 'q' to quit the program.


## Features

    Automatically generates a menu from .sh scripts in the scripts folder.
    Easy navigation using arrow keys.
    Executes selected scripts directly from the TUI.
    Clean and user-friendly interface.

This tool is an excellent choice for users who regularly interact with shell scripts and prefer a consolidated and efficient script execution process.


### Create a shortcut to use it from anywhere



#### For Linux and macOS

Move the Executable to a Bin Directory:
    First, you need to move the hao executable to a directory that is in your system's PATH. Common choices are /usr/local/bin or ~/bin. If ~/bin does not exist, you can create it and add it to your PATH.

Creating ~/bin and Adding to PATH (if necessary):
    Create a bin directory in your home directory:

```sh
    mkdir ~/bin
```

Add this directory to your PATH. For bash, add the following line to your ~/.bashrc or ~/.bash_profile. For zsh, add it to ~/.zshrc:

```sh
    export PATH="$HOME/bin:$PATH"
```

Apply the changes (or restart your terminal):

```sh
    source ~/.bashrc  # or ~/.bash_profile or ~/.zshrc
```

#### Move the hao Executable:

    Move the hao executable to the chosen bin directory. If using /usr/local/bin, you might need sudo:

```sh
    mv ./target/release/hao /usr/local/bin
```

or for ~/bin:

```sh
    mv ./target/release/hao ~/bin
```

#### Running hao:

    Now, you can run hao from any directory by simply typing:

```sh
    hao
```

    Ensure that the scripts directory is reachable from the location you run hao from, or modify hao to look for scripts in a global location.

#### Additional Notes

    Permissions: Make sure that the hao executable has execute permissions. You can set this with chmod +x /path/to/hao.
    macOS Security: On macOS, you might encounter security warnings when running an unsigned executable. You can allow your application in the "Security & Privacy" settings if this happens.


