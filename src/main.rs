use bitreader::BitReader;
// alternative: https://github.com/BurntSushi/byteorder.

const FONT : &'static [u8] = include_bytes!("./FreeMono-30P.bin");


/// Decode the font file made by the specifications from here:
/// https://github.com/littlevgl/lv_font_conv/blob/master/doc/font_spec.md
fn main() -> Result<(), bitreader::BitReaderError> {
    let mut reader = BitReader::new(FONT);

    let record_size = reader.read_u32(32)?;
    dbg!(record_size);
    let head /* table marker */ = reader.read_u32(32)?;
    dbg!(head);
    let version = reader.read_u16(16)?;
    dbg!(version);
    let additional_tables = reader.read_u16(16)?;
    dbg!(additional_tables);
    let font_size = reader.read_u16(16)?;
    dbg!(font_size);
    let ascent  = reader.read_u16(16)?;
    dbg!(ascent);
    let descent = reader.read_i16(16)?;
    dbg!(descent);
    let typo_ascent = reader.read_u16(16)?;
    dbg!(typo_ascent);
    let typo_decent = reader.read_i16(16)?;
    dbg!(typo_decent);
    let typo_line_gap = reader.read_i16(16)?;
    dbg!(typo_line_gap);
    // Min Y (used to quick check line intersections with other objects).
    let min_y = reader.read_u16(16)?;
    dbg!(min_y);
    let max_y = reader.read_u16(16)?;
    dbg!(max_y);
    let advance_width = reader.read_i16(16)?;
    dbg!(advance_width);
    //FP12,4
    //todo: read this the right way.
    let kerning_scale = reader.read_u16(16)?;
    dbg!(kerning_scale);
    // indexToLocFormat in loca table (0 - Offset16, 1 - Offset32).
    let index_to_loc_format = reader.read_u8(8)?;
    dbg!(index_to_loc_format);
    // glyphIdFormat (0 - 1 byte, 1 - 2 bytes).
    let glyph_id_format = reader.read_u8(8)?;
    dbg!(glyph_id_format);
    let advance_width_format = reader.read_u8(8)?;
    dbg!(advance_width_format);
    let bits_per_pixel = reader.read_u8(8)?;
    dbg!(bits_per_pixel);
    // Glyph BBox x/y bits length (signed value).
    let glyph_b_box_x_y = reader.read_i8(8)?;
    dbg!(glyph_b_box_x_y);
    // Glyph BBox w/h bits length (unsigned).
    let glyph_b_box_w_h = reader.read_u8(8)?;
    dbg!(glyph_b_box_w_h);
    // Glyph advanceWidth bits length (unsigned, may be FP4).
    let glyph_advanced_width_bits_length = reader.read_u8(8)?;
    dbg!(glyph_advanced_width_bits_length);
    let compression_algorithm_id = reader.read_u8(8)?;
    dbg!(compression_algorithm_id);

    Result::Ok(())
}
