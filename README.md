# Code Printer
Code printer is a tool for scanning code and storing it in a docx file, written in Rust.

## Setup
Not much here except, in all directories you want to scan, make an 'assign_inc.txt' file. 
This tells code_printer to include all the contents in this directory in the final output file.

## Usage
Clone the repo onto your computer, then run the exe in the terminal with the following arguments:

 - Mandatory:
	 - the path to the directory. 
	 - the file type (with no .)
 - Optional:
	 - the path to save to. By default it will be where you ran the exe.

For example, to scan for rust files in a directory on macOS, run:
```console
path/to/code_printer path/to/my/directory rs path/to/save/to
```


## Issues
This project is in the first version phase, so expect bugs. Feel free to open an issue with any bugs you find and suggestions to make the codebase better, as this is one of my first projects for learning Rust.
