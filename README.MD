# CHUNKDUMP

A simple command line tool to read and display as many of the "chunks" of metadata as I can find and interpret in RIFF
Wave (.wav), AIFF/AIFF-C (.aif), FLAC (.flac), Ogg Vorbis (.ogg), MPEG-4 Audio (.m4a), Apple Core Audio Format File (
.caf)
and both RIFF RMID (.rmi) and standard SMF (.mid) MIDI files.

## Usage

To use just build as per Rust usual.
> cargo build -r

> cp target/release/chunkdump /your/executable/path/of/choice

&nbsp;

Then point it at your .wav, .aif, .flac, .ogg, .mp3, .m4a, .caf, .mid or .rmid file with
> chunkdump /path/to/file

or more fully

> chunkdump [-hmv] [-o <output-file>] <file>

#### Flags:

> -h - Display usage information
> -v - Print the version  
> -o /path/to/output/file - A file path to output the data to rather than stdout  
> -m - Only output mandatory chunks for each file type

## File Type Support Details TOC

- Flac
    - [Current List of Supported FLAC Blocks](#current-list-of-supported-flac-blocks)
    - [Special Skipped FLAC Blocks](#special-skipped-flac-blocks)
- Wave
    - [Current List of Supported Wave Chunks](#current-list-of-supported-wave-chunks)
    - [Special Skipped Wave Chunks](#special-skipped-wave-chunks)
- AIFF/AIFF-C
    - [Current List of Supported AIFF - AIFF C Chunks](#current-list-of-supported-aiff-and-aiff-c-chunks)
    - [Special Skipped AIFF - AIFF C Chunks](#special-skipped-aiff-and-aiff-c-chunks)
- Wave and AIFF Unsupported Chunks
    - [For Unsupported Wave and AIFF Chunks](#for-unsupported-wave-and-aiff-chunks)
- Ogg Vorbis
    - [Current List of Supported Ogg Vorbis Headers](#current-list-of-supported-ogg-vorbis-headers)
    - [Special Skipped Ogg Vorbis Headers](#special-skipped-ogg-vorbis-headers)
- MP3
    - [Current List of Supported MP3 Headers](#current-list-of-supported-mp3-headers)
    - [Special Skipped MP3 Headers](#special-skipped-mp3-headers)
- M4A
    - [Current List of Supported M4A Chunks](#current-list-of-supported-m4a-chunks)
    - [Special Skipped M4A Chunks](#special-skipped-m4a-chunks)
- CAF
    - [Current List of Supported CAF Chunks](#current-list-of-supported-caf-chunks)
    - [Special Skipped CAF Chunks](#special-skipped-caf-chunks)
- MIDI (SMF & RMID)
    - [Current List of Supported MIDI File Metadata](#current-list-of-supported-midi-file-metadata)

## File Type Chunk Details

### Current List of Supported FLAC Blocks

- **Application**
    - An optional block for use by third party applications for their own purposes
    - It contains an Application ID that is registered with IANA. See here: https://xiph.org/flac/id.html
- **Cuesheet**
    - An optional block similar to the seek table that provides index points to samples in the audio frames.
- **Padding**
    - An optional block generally used for padding the file size and is supposed to be ignored.
      Unlike with the wave files I haven't found any "secret" message stored in junk chunks for aiffs but you never
      know.
- **Picture**
    - An optional block for storing images, generally the album cover or similar.
- **Seek Table**
    - An optional list of seek points to various samples of interest in the audio frames.
    - A seek table is generally not usable for seeking in a FLAC file embedded in a container (see Section 10), as such
      containers usually interleave FLAC data with other data and the offsets used in seek points are those of an
      unmuxed FLAC stream.
- **Stream Info**
    - A mandatory Flac file block specifying the format of the data
- **Vorbis Comments**
    - An optional block storing key value pair metadata similar to Vorbis User Comment metadata hence the name.

### Special Skipped FLAC Blocks

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

### Special Skipped Wave Chunks

- **data**: This is the audio portion which is just skipped. See your media player of choice to read that chunk.
- **DISP**: This is a Windows Clipboard chunk and isn't useful here so it is skipped
- **LGWV**: A Logic Pro proprietary chunk. This is unsupported in chunkdump as I can't find any specs online.
  Since the output doesn't produce any usable string data it is just skipped.
- **DGDA, minf, elm1, regn**: Proprietary Pro Tools chunks that don't seem to have documentation or obvious formats.
  DGDA might eventually be reverse engineerable though we shall see what happens.

### Current List of Supported AIFF and AIFF C Chunks

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

### Special Skipped AIFF and AIFF C Chunks

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

### Current List of Supported Ogg Vorbis Headers

- **Identifications**
    - This describes the basic format data for the file. Like the "fmt " chunk in .wav files.
- **Comment**
    - A header storing an single Vendor Comment text string and a list of options key value metadata pairs about the
      file similar to FLAC's Vorbis Comment.

### Special Skipped Ogg Vorbis Headers

- **OggS**: This is the header to mark this as an Ogg container. It contains no metadata and is skipped.
- **vorbis**: The "Common" Header. This is just a header in each OggS container chunk that specifies that this is a
  Vorbis header.
  There is no metadata in it so it is skipped.
- **Setup**: While it is its own header it is largely just some variable inputs for the math used to read the audio
  section and doesn't provide
  any useful metadata so it is skipped.
- **Audio Packets**: This is the audio portion which is just skipped. See your media player of choice to read that
  data.

### Current List of Supported MP3 Headers

- **Frame Header**
    - MP3s are made up of multiple frames and each should have it's own frame header with all the information about
      data.
      While this could change in each frame, outside of variable bitrate schemes it generally shouldn't. As is ofter
      done
      I just read the first header to get the information. This should suffice but know that VBR bitrates might change
      per frame.
- **id3**
    - ID3 format metadata tags. MP3 doesn't really have its own metadata for things like artist and title so while ID3
      is
      optional and not expressly part of the MP3 standard, it does in practice play the part of the MP3 file's metadata
      chunks.

### Special Skipped MP3 Headers

- **Additional Frame Headers**: As mentioned above I do not check each frame's frame header to look for variable
  bitrates.
- **Audio Frames**: This is the audio portion which is just skipped. See your media player of choice to read that
  data.

### Current List of Supported M4A Chunks

- **Audio Info**
    - There are some general audio format chunks much like fmt or comm in the wave and aiff files respectively.
      In this case Channel Count, Duration, Sample Rate, and Average and Max Bitrates.
- **User Data**
    - Much like Ogg Vorbis or ID3 tags these user data chunk tags are key value pairs with information about the
      recording like track tile, artist, etc. There are also some off iTunes specific longer tags.

### Special Skipped M4A Chunks

- **Audio Chunks**: This is the audio portion which is just skipped. See your media player of choice to read that
  data.

### Current List of Supported CAF Chunks

- **chan**
    - The Channel Layout chunk describes the order and role of each channel in a CAF file.
- **desc**
    - The Audio Description describes the format of the audio data in the Audio Data chunk.
- **free**
    - The optional Free chunk is for reserving space, or providing padding, in a CAF file. Normally this is ignored but
      somethimes there is contents in it so I attempt to render it out as a string.
- **id3**
    - ID3 format metadata tags. MP3 doesn't really have its own metadata for things like artist and title so while ID3
      is optional and not expressly part of the MP3 standard, it does in practice play the part of the MP3 file's
      metadata chunks.
- **info**
    - Information chunk to contain any number of human-readable text strings.
- **mark**
    - The Marker chunk can contain any number of marker descriptions, each of which marks a particular sample location
      in the file.
- **midi**
    - The MIDI chunk allows you to embed MIDI SMF file data in the CAF container including the SMF metadata which is
      extrated by chunkdump
- **ovvw**
    - You can use the optional Overview chunk to hold sample descriptions that you can use to draw a graphical view of
      the audio data in a CAF file.
    - This section can get very long and might require writing it to a file and removing this piece. I might need to
      find a better output format for this.
- **regn**
    - The Region chunks containers region descriptions for the audio data chunk. Each region description includes
      starting and ending marker
      descriptions that delineate a span of sample frames in
      the audio data. See Marker Descriptions for more information about markers.
- **strg**
    - The optional Strings chunk contains any number of textual strings, along with an index for accessing them. These
      strings serve as labels for other chunks, such as Marker or Region chunks.
    - This is a very strange feature of CAF files. The Marker and Region chunks do not contain the names of each of the
      markers and regions.
      Those names are stored in the stings chunk and you have to infer from the order of markers and regions and strings
      which is which.
    - Note that markers in the Marker chunk have names in the string chunk and in the same order. Regions can also have
      markers
      within them and those markers seem, at least some of the time, to get the same name as the region they are in. It
      is . . . irritating.
- **uuid**
    - The CAF format lets you extend it with user defined chunks. This UUID header is used to define those chunks.
    - Because those user define chunks are generally created by the maker of the audio application that created file,
      they are not
      included in the file specification, they could be anything and so I have no way of identifying and properly
      parsing them,
      I print them out as strings in the hope that there is something useful there for you to parse through manually.

### Special Skipped CAF Chunks

- **data**: This is the audio portion which is just skipped. See your media player of choice to read that
  data.
- **pakt**:
- **kuki**

**These chunks are part of the spec for CAF specification but I can not find examples in the wild and can't find an app
that will create files that use them.
Rather than release support for them untested or create files myself from scratch and trick myself into believing it is
connect enough
to count as valid testing.
Once I find a CAF file from a valid source that has these metadata chunks I will add support**

- **instr**
- **edct**
- **peak**
- **umid**

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
