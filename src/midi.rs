const NOTE_NAMES_WITHOUT_OCTAVES: [&str; 12] =
    ["C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B"];

pub fn note_name_from_midi_note_number(midi_note_number: u32) -> String {
    let note_offset_from_c: usize = midi_note_number as usize % 12;
    let note_name = NOTE_NAMES_WITHOUT_OCTAVES[note_offset_from_c].to_string();
    let note_octave = ((midi_note_number as f32 - note_offset_from_c as f32) / 12.0) - 2.0;
    format!("{}{}", note_name, note_octave)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_note_c_minus_2_when_midi_note_number_is_0() {
        assert_eq!(note_name_from_midi_note_number(0), "C-2");
    }

    #[test]
    fn return_note_g_flat_3_when_midi_note_number_is_66() {
        assert_eq!(note_name_from_midi_note_number(66), "F#/Gb3");
    }

    #[test]
    fn return_note_g8_when_midi_note_number_is_127() {
        assert_eq!(note_name_from_midi_note_number(127), "G8");
    }
}
