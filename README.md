# Ofin
A hotkey scripting language.

This is currently a work in progress.

## Installation

The only method available as of now to install Ofin is from source.  
Before installing, if you are running linux you will need some packages, these can be installed on ubuntu with:
```
sudo apt-get install libdbus-glib-1-dev libsdl-pango-dev libatk1.0-dev libgtk-3-dev
```
Assuming you have [rust](https://www.rust-lang.org/) (and cargo) installed you should be able to simply run:  
```
cargo install --git https://github.com/Nigecat/Ofin
```  
This adds the `ofin` binary to your PATH. For usage details run `ofin --help`
