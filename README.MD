# CHUNKDUMP

**_In active and very unstable development._**

A simple command line tool to read and display as many of the "chunks" of metadata as I can find and interpret in .wav
Resource Interchange File Format (RIFF) Wave files as well as the standard metadata from Vorbis Flac files.

To use just build as per Rust usual.
> cargo build -r

> cp target/release/chunkdump /your/executable/path/of/choice

&nbsp;

Then point it at your .wav or .flac file with
> chunkdump /path/to/file

#### Usage

> chunkdump [-v] [-o <output-file>] <file>

#### Flags:

> -v - Print the version  
> -o /path/to/output/file - A file path to output the data to rather than stdout  
> -h - Display usage information


&nbsp;

### Flac File Metadata Supported

Flac files have much more standardized metadata than wave files so there shouldn't be a lot of unexpected metadata
blocks.
I am just extracting some basic file information and the StreamInfo and Vorbis Comment metadata blocks. Padding and Data
blocks are skipped.

### Current List of Supported Wave Chunks

- **acid**
    - For "Acidized" wave files (*note: Meter seems to be set to 4/4 regardless of the meter of the file audio)
- **bext**
    - Broadcast Wave Format (BWF)
    - European Broadcast Union EBU - Tech 3285
    - A format for audio data files in broadcasting
- **cart**
    - Audio Engineering Society - AES46
    - Radio Traffic Data - Broadcast-specific cartridge-labeling information
- **cue**
    - Stores time markers (i.e. cues) for the contents of the wave file data chunk
- **fact**
    - This chunk stores file-dependent information about the contents of the WAVE file. It currently specifies the
      length of the file in samples.
    - The FACT chunk is related to the bext chunk.
- **fmt**
    - A mandatory RIFF Wave file chunk specifying the format of the data
- **id3**
    - ID3 format metadata tags
- **iXML**
    - iXML is an open standard for providing metadata to broadcast wave files.
      It was intended to supercede the bext chunk metadata.
- **junk**
    - An optional RIFF wave chunk generally used for padding the file size and is supposed to be ignored.
      Occasionally though as in the case of Ableton Live created waves it will contain some interesting text data.
- **LIST**
    - An ordered list of other sub-chunks that contain various metadata.
    - INFO and ADTL, the most common LIST sub-chunks for wave files are supported
- **ResU**
    - A Logic Pro X metadata format in the form of a JSON document
- **_PMX** (XMP)
    - The XML based Adobe XMP metadata standard
- **smpl**
    - Metadata to allow samplers to use the wave file as a collection of samples.

### Special Skipped Wave Chunks:

- **data**: This is the audio portion which is just skipped. See your media player of choice to read that chunk.
- **DISP**: This is a Windows Clipboard chunk and isn't useful here so it is skipped
- **LGWV**: A Logic Pro proprietary chunk. This is unsupported in chunkdump as I can't find any specs online.
  Since the output doesn't produce any usable string data it is just skipped.

### For Unsupported Wave Chunks:

For any chunk that is not expressly supported, chunkdump will try and output the contents as a string.
This is somewhat similar to running the "strings" command on the file.