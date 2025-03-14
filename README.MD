# CHUNKDUMP

**Status: v0.5.0 - _Stable but currently only tested on macOS with Apple Silicon._**

A simple command line tool to read and display as many of the "chunks" of metadata as I can find and interpret in .wav
Resource Interchange File Format (RIFF) Wave files and Audio Interchange File Format AIFF/AIFF-C .aif files as well as
the standard metadata from Flac files.

To use just build as per Rust usual.
> cargo build -r

> cp target/release/chunkdump /your/executable/path/of/choice

&nbsp;

Then point it at your .wav, .aif, .flac, .mid or .rmid file with
> chunkdump /path/to/file

#### Usage

> chunkdump [-v] [-o <output-file>] <file>

#### Flags:

> -v - Print the version  
> -o /path/to/output/file - A file path to output the data to rather than stdout  
> -h - Display usage information


&nbsp;

### Current List of Supported FLAC Blocks

- **application**
    - An optional block for use by third party applications for their own purposes
    - It contains an Application ID that is registered with IANA. See here: https://xiph.org/flac/id.html
- **cuesheet**
    - An optional block similar to the seek table that provides index points to samples in the audio frames.
- **padding**
    - An optional block generally used for padding the file size and is supposed to be ignored.
      Unlike with the wave files I haven't found any "secret" message stored in junk chunks for aiffs but you never
      know.
- **picture**
    - An optional block for storing images, generally the album cover or similar.
- **seek_table**
    - An optional list of seek points to various samples of interest in the audio frames.
    - A seek table is generally not usable for seeking in a FLAC file embedded in a container (see Section 10), as such
      containers usually interleave FLAC data with other data and the offsets used in seek points are those of an
      unmuxed FLAC stream.
- **stream_info**
    - A mandatory Flac file block specifying the format of the data
- **Vorbis Comments**
    - An optional block storing key value pair metadata similar to Vorbis codex metadata hence the name.

### Special Skipped FLAC Blocks:

- **Audio Frames**: This is the audio portion which is just skipped. See your media player of choice to read that block.

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

### Current List of Supported MIDI File Metadata

There are 2 kinds of MIDI files "Standard MIDI Files" SMF and the somewhat less common and odd "RIFF MIDI" RMID files.
RMID files are contained inside a RIFF container like WAVE files but their DATA chunk contains an complete SMF midi
file.
RMID files should have a .rmid extension apparently but sometimes they just have a .mid extension. \
Chunkdump supports either option.

The idea it seems is for RMID is to provide for richer metadata to be layered ontop of standard midi files.
As such chunkdump supports all the WAVE file chunks listed above and all the SMF MIDI metadata described below.

SMF MIDI files contain one or more tracks any of which can contain metadata. The first track is the header track which
gives you metadata about the format of the file like WAVE fmt or AIFF COMM chunks. And that is followed by one or more
MIDI tracks which can, but don't have to, contain "Meta Events" containing metadata about the track contents.

Chunkdump supports all the standard Header metadata and what should be all Meta Events but this is an insane protocol
from the 80s so it is hard to know I found them all.

Meta Events will show the Track Number the even occurs in, the "Delta Time", an "Event" type as per the list below and
a value for the event which will vary in type depending on the event.

Delta Time is the point in the track where the event occurs. The Delta Time is presented as pure value for each event.  
The units that that value use is represented in the Header Info section for the file either as a "Division: " meaning
ticks per quarter note or as a SMPTE/Midi Time Code that has both a frames per second and ticks per frame value.
All of which will be clearly stated in the header info.

- **0x00 - Sequence Number**
- **0x01 - Text**
- **0x02 - Copyright Notice**
- **0x03 - Track Name**
- **0x04 - Instrument Name**
- **0x05 - Lyrics**
- **0x06 - Marker**
- **0x07 - Cue Point**
- **0x09 - Device Name**
- **0x20 - Channel Prefix**
    - A midi channel. If this is present it means the next meta event applies to that channel.
- **0x21 - MIDI Port**
- **0x2F - End of Track**
    - This will not show up in the output. I just put it here for completeness.
- **0x51 - Set Tempo**
    - This occurs to set or change the tempo of the
- **0x54 - SMPTE Offset**
- **0x58 - Time Signature**
    - In MIDI as in life this defaults to 4/4 time if nothing is specified to the contrary.
    - If it does say 4/4 it may be specified as such or it might just be the default. Good luck.
- **0x59 - Key Signature**
    - C Major is the default and in some cases it can show C Major even if it is not.
    - Generally if this event shows up it will be properly specified but take C Major with a grain of salt.
- **0x7F - Sequencer Specific**
    - This contains a byte representing the manufacture and then data representing what ever the manufacturer wanted to
      store.
    - The idea was to provide instructions for hardware sequencers. Perhaps to save its internal state in some way.
    - The data will be a string of raw hex bytes as there is no way to know what it should be. The format is:

> manufacturer: data bytes

If a meta event that isn't supported does show up chunkdump will note that and display the byte representing that event
id
and try and render the contents as a string as with the other file types above.
