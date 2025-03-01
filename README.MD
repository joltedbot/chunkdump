# CHUNKDUMP

**Status: v0.2.0 - _Stable but currently only tested on macOS with Apple Silicon._**

A simple command line tool to read and display as many of the "chunks" of metadata as I can find and interpret in .wav
Resource Interchange File Format (RIFF) Wave files and Audio Interchange File Format AIFF/AIFF-C .aif files as well as
the standard metadata from Flac files.

To use just build as per Rust usual.
> cargo build -r

> cp target/release/chunkdump /your/executable/path/of/choice

&nbsp;

Then point it at your .wav, .aif, or .flac file with
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
- **AXML**
    - EBU standard XML based metadata format for storing core metadata for BWF format files.
    - Used for the transfer of core metadata according to either or both EBU documents Tech 3293 and Tech 3295.
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
- **pad**
- Technically not a valid chunk. Padding should be accomplished with the JUNK chunk but some audio applications don't
  play by the rules. I treat this as if it was the proper JUNK chunk.
- **ResU**
    - A Logic Pro X metadata format in the form of a JSON document
- **_PMX** (XMP)
    - The XML based Adobe XMP metadata standard
- **smpl**
    - Metadata to allow samplers to use the wave file as a collection of samples.
- **SNDM**
    - I am not sure what this is and I haven't found a spec for it but I worked out how to read it to generate what
      seems like correct output based on the bytes present and the output I fields that I found in the sole reference
      to the the chunk online. I hope it is transferable as I only have a couple files to test with that chunk
- **umid**
    - A basic UMID filed added by Pro Tools

### Special Skipped Wave Chunks:

- **data**: This is the audio portion which is just skipped. See your media player of choice to read that chunk.
- **DISP**: This is a Windows Clipboard chunk and isn't useful here so it is skipped
- **LGWV**: A Logic Pro proprietary chunk. This is unsupported in chunkdump as I can't find any specs online.
  Since the output doesn't produce any usable string data it is just skipped.
- **DGDA, minf, elm1, regn**: Proprietary Pro Tools chunks that don't seem to have documentation or obvious formats.
  DGDA might eventually be reverse engineerable though we shall see what happens.

### Current List of Supported AIFF/AIFF-C Chunks

- **ANNO**
    - A simple text chunk storing a free form text comment or annotation about the recorded sound.
- **APPL**
    - The Application Chunk. This is text data specific to application that created or modified the aiff/aiff-c file.
- **AUTH**
    - A simple text chunk storing the author of the recorded sound.
- **COMM**
    - Common Chunk. This describes the basic format data for the file. Like the "fmt " chunk in .wav files.
- **COMT**
    - Comment Chunk. This stores text comments for any provided markers in the MARK chunk. Marker IDs in this chuck
      reference the same Marker ID from the MARK chunk.
- **(c) **
    - A simple text chunk storing a copyright notice for the recorded sound.
- **FVER**
    - Format Version Chunk. The Format Version Chunk contains a date field to indicate the format rules for an AIFF-C
      specification.
    - The idea is apparently to compare this timestamp against that of the AIFF spec your app supports to see if it is  
      compatible.
    - I have converted it to a human readable data for you the humans.
- **id3**
    - ID3 format metadata tags
- **junk**
    - An optional chunk generally used for padding the file size and is supposed to be ignored.
      Unlike with the wave files I haven't found any "secret" message stored in junk chunks for aiffs but you never
      know.
- **MARK**
    - Marker Chunk. The Marker Chunk provides positional markers, like cues or cuepoints in other formats, that
      reference
      specific positions in the SSND audio section.
- **NAME**
    - A simple text chunk storing the name of the recorded sound.

### Special Skipped AIFF/AIFF-C Chunks:

- **SSND**: This is the audio portion which is just skipped. See your media player of choice to read that chunk.
- **LGWV**: A Logic Pro proprietary chunk. This is unsupported in chunkdump as I can't find any specs online.
  Since the output doesn't produce any usable string data it is just skipped.
- **CHAN**: I haven't worked out a format for this one and most apps just skip it so for now that is what I am doing.

### For Unsupported Wave and AIFF Chunks:

For any chunk that is not expressly supported, chunkdump will try and output the contents as a string.
This is somewhat similar to running the "strings" command on the file.

For AIFF/AIFF-C these unsupported (or not yet supported) chunks include the following. They are in the formal spec but
I can't find files that contain them or an app to create files that contain to test an implementation.
For now they will show up in "Extra Chunks" until I actually have something to work with.

- **Audio Recording Chunk - AESD**
- **Instrument Chunk - INST**
- **MIDI Data Chunk - MIDI**
    
    