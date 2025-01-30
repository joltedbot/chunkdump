# CHUNKDUMP

**_In active and very unstable development._**

**_"It works on my computer!"_**

A simple command line tool to read and display as many of the "chunks" of metadata as I can find and interpret in .wav
RIFF Wave files. Other file formats may follow starting with FLAC.

Chunks that I do not currently have a spec for to decode will be output as lossy text similar to the strings
command . . .
sort of.

To use just build as per Rust usual.
> cargo build -r

> cp target/release/chunkdump /your/executable/path/of/choice

&nbsp;

Then point it at your .wav file
> chunkdump file.wav

&nbsp;

Current List of Supported Chunks

- acid
- bext
- cart
- cue
- extra
- fact
- fmt
- id3
- ixml
- junk
- list
- resu
- xmp

** the data chunk is just skipped. See your media player of choice to read that part.