# obfuscate
Simple encoding-based obfuscation generator


## Usage
Interaction with the compiled program is done through command line arguments.
```
> executable (string|"file") [File to be read, path] [File to modify, path]
```
The first argument read is assumed to be self.
The second argument (first given by user) is either a string encased with quotation marks or the word file.
- string: encodes the string into a file named `export.txt` which is created in the same file location as the executable
- file: requires either a path as the third argument for reading, or a path as the third argument for reading AND a path as the fourth argument as an export location. This also allows the customisation of the exported files name.

Example commands
```
> obfuscate.exe "obfuscate me"
> obfuscate.exe "file" ".\path\to\file"
> obfuscate.exe "file" ".\path\to\file" ".\another\path\output.txt"
```

## How does it work?
This obfuscation system works by prepending the text with a UTF-16 (BE) [Byte order mark](https://en.wikipedia.org/wiki/Byte_order_mark).
For example:
```
62 79 74 65      FE FF 62 79 74 65
B  Y  T  E   ->        批    瑥
```
