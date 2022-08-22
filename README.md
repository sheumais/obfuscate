# obfuscate
Simple encoding-based obfuscation generator

## How does it work?
This obfuscation system works by prepending the text with a UTF-16 (BE) [Byte order mark](https://en.wikipedia.org/wiki/Byte_order_mark).
For example:
```
62 79 74 65      FE FF 62 79 74 65
B  Y  T  E   ->        批    瑥
```
