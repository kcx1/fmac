# FMAC
### Fast and simple CLI tool to format MAC Addresses

## About

This is a fast and simple command line tool that allows you to quickly format a MAC address or list of MAC addresses. As a convenience, the last entered MAC address will be copied into the clipboard to make pasting it even faster.

## Usage
The default settings use canonical formatting: ```aa:bb:cc:12:34:56``` However, that can be modified by passing some simple options. 

The ```-c --caps``` option will change the format to use capital letters: ```AA:BB:CC:12:34:56``` 

You can also change the seperator character by passing the ```-s --separator``` option followed by the character that you'd like to use. A hyphen could be a common choice: ```ab-cd-ef-12-34-56```.

Beyond simple formatting **fmac** will also help you validate MAC addresses. If you try passing a MAC that contains an incorrect number of characters, you see a message explaining.

> [!NOTE]
> The way this application works is that it starts by removing any nonvalid (non hexdigits) from the MAC address. Then it validates the length.
>So there are circumstances when the output message can seem misleading.
>
> For example, if you accidentally included the letter 'O' instead of the number '0', the program would first remove it as a nonvalid character and then preform a length validation.
> The result would be a message suggesting the there is one character missing, Even though the original length was 12 +/- the previous seperators.

## Install
Download from the releases tab. Move the application to a folder that is on your $PATH. If you chose to put in somewhere like ```/usr/local/bin```, make sure that you update the ownership and the permissions. Defaults are typically something like: ```chown root:wheel path/to/file && chmod 755 path/to/file```

If you're on Mac you'll also need to tell Mac to trust the software: 

[Apple Help Guide](https://support.apple.com/guide/mac-help/apple-cant-check-app-for-malicious-software-mchleab3a043/mac)

Navigate to the application in Finder and right click the app. Then select "open". You may be prompted to select open once more in the terminal. Once you okay it, you should be good to go and can run the commands from the terminal now. 

## Motivation
This is actually a rust rewrite of a script that I had done previously in python. There were a few reasons for the rewrite. I had already been considering rewriting the project and moving it to it's own repo. I wanted it to be easily accessible and easier to install and use. Python comes with a lot of overhead in order to make it work as easy as other command line tools. (Setting up venv's and such) Addionally, since I'm starting to learn Rust, I thought this project would make a great canidate for a starting project. Plus it's fast! 

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
