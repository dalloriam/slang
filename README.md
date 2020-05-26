# slang
[![codecov](https://codecov.io/gh/dalloriam/slang/branch/master/graph/badge.svg)](https://codecov.io/gh/dalloriam/slang)

Language VM from https://blog.subnetzero.io/post/building-language-vm-part-01/

# Executable Header Structure
* Total Header Length: 64 bytes
    * 4 bytes for the magic number (`45 4C 49 43`)
    * 4 bytes for the length of the data section
    * 56 bytes padding (reserved for future use)
