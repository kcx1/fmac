[![Rust](https://github.com/kcx1/fmac/actions/workflows/rust.yml/badge.svg)](https://github.com/kcx1/fmac/actions/workflows/rust.yml)
# FMAC
### Fast and simple CLI tool to format MAC Addresses

## About

This is a fast and simple command line tool that allows you to quickly format a MAC address or list of MAC addresses. As a convenience, the last entered MAC address will be copied into the clipboard to make pasting it even faster.

## Usage
The default settings use canonical formatting: ```aa:bb:cc:12:34:56``` However, that can be modified by passing some simple options. 

The ```-c --caps``` option will change the format to use capital letters: ```AA:BB:CC:12:34:56``` 

You can also change the separator character by passing the ```-s --separator``` option followed by the character that you'd like to use. A hyphen could be a common choice: ```ab-cd-ef-12-34-56```. Version 1.1.0 allows you to also specify no separator by passing ```-s ""```

Beyond simple formatting **fmac** will also help you validate MAC addresses. If you try passing a MAC that contains an incorrect number of characters, you see a message explaining.

> [!NOTE]
> The way this application works is that it starts by removing any non-valid (non hex-digits) from the MAC address. Then it validates the length.
>So there are circumstances when the output message can seem misleading.
>
> For example, if you accidentally included the letter 'O' instead of the number '0', the program would first remove it as a non-valid character and then preform a length validation.
> The result would be a message suggesting the there is one character missing, Even though the original length was 12 +/- the previous separators.

## Install
Download from the releases tab. Move the application to a folder that is on your $PATH. I like to put it either somewhere like ```/usr/local/bin```, or ```~/.local/bin/```


> [!TIP]
> If you're on MacOS you'll also need to tell Mac to trust the software.
> 
> This is easiest done beofre you move the file. Simply open the zip in your downloads folder and right click "Open" on the fmac application. This will ask you if you trust the software, click ok and then it will run in your default terminal.
> 
> It will exit quickly, and that's okay. It just displays the help and exits.
> 
> Once it's ran it will be trusted and can be moved to somewhere that's already on $PATH like ```/usr/local/bin```. From here you can run it in your terminal like any other command. Here's an article if you get stuck: [Apple Help Guide](https://support.apple.com/guide/mac-help/apple-cant-check-app-for-malicious-software-mchleab3a043/mac)

## Motivation
This is actually a rust rewrite of a script that I had done previously in python. There were a few reasons for the rewrite. I had already been considering rewriting the project and moving it to it's own repo. I wanted it to be easily accessible and easier to install and use. Python comes with a lot of overhead in order to make it work as easy as other command line tools. (Setting up venv's and such) Additionally, since I'm starting to learn Rust, I thought this project would make a great candidate for a starting project. Plus it's fast! 

## Help
```
A fast and simple CLI tool for formatting mac addresses

Usage: fmac [OPTIONS] [MAC]...

Arguments:
  [MAC]...  Pass in any number of MAC addresses that you would like to format. If you're using spaces as a separator, wrap them in quotes

Options:
  -c, --caps                   Format the MAC addresses in uppercase; defaults to lowercase
  -s, --separator <SEPARATOR>  Use a custom separator character between each octet [default: :]
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```
