use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use crate::err::*;
use crate::inch::*;
use crate::mrg::*;
use crate::sze::*;
use crate::unit::*;
use google_fonts::Font;
use serde::{Deserialize, Serialize};
use skia_safe::{
    pdf,
    textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, PlaceholderAlignment, PlaceholderStyle,
        TextAlign, TextBaseline, TextStyle, TypefaceFontProvider,
    },
    Document, FontMgr, FontStyle, Paint, Point,
};
use std::collections::hash_map::Entry::Vacant;

/// Creates an _8.5in x 11in_ [`Doc`].
pub fn new_ansi_letter() -> Doc {
    Doc::default()
        .set_sze(ANSI_LETTER)
        .set_mrg(MRG_IN_1)
        .set_ind(In(0.5))
}

/// A PDF document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doc {
    /// Size of the document.
    pub sze: Sze,
    /// Margin lengths of the document.
    pub mrg: Mrg,
    /// Indentation length of a paragraph's first line.
    pub ind: In,
    /// Font for the paragraph.
    pub fnt: Font,
    /// The size of the font in points.
    pub fnt_sze: f32,
    /// Font _style_ of the document.
    pub fnt_sty: Style,
    /// Text _alignment_ of the document.
    pub aln: Align,
    /// Line spacing of a document.
    pub spc_lne: LineSpace,
    /// Spacing _after_ a paragraph.
    pub spc_par_aft: LineSpace,
    /// Indicates whether the first line of a paragraph is _indented_.
    pub has_ind: bool,
    /// Paragraphs of text.
    pub elms: Vec<Elm>,
}

impl Default for Doc {
    fn default() -> Self {
        Doc {
            sze: Sze::default(),
            mrg: Mrg::default(),
            ind: In::default(),
            fnt: Font::DomineVariable,
            fnt_sze: 12.0,
            fnt_sty: Style::Normal,
            aln: Align::Justify,
            spc_lne: LineSpace::Custom(1.35),
            spc_par_aft: LineSpace::Custom(1.35),
            has_ind: true,
            elms: Vec::new(),
        }
    }
}

impl Doc {
    /// Save the document as a _JSON_ file.
    pub fn save_json<P>(&self, pth: P) -> Result<(), DocError>
    where
        P: AsRef<Path>,
    {
        // Serialize doc.
        let json_str = serde_json::to_string_pretty(self).map_err(DocError::from)?;

        // Append file suffix.
        let file_path = pth.as_ref().with_extension("json");

        // Create file.
        let mut file = File::create(file_path).map_err(DocError::from)?;

        // Write doc to disk.
        file.write_all(json_str.as_bytes())
            .map_err(DocError::FileError)?;

        Ok(())
    }

    /// Read a JSON file from disk.
    pub fn read_json<P>(&self, pth: P) -> Result<Doc, DocError>
    where
        P: AsRef<Path>,
    {
        // Append file suffix.
        let file_path = pth.as_ref().with_extension("json");

        // Load the file.
        let fle = File::open(file_path).map_err(DocError::from)?;
        let rdr = BufReader::new(fle);

        // Deserialize the JSON into a struct.
        let ret: Doc = serde_json::from_reader(rdr).map_err(DocError::from)?;

        Ok(ret)
    }

    /// Save the document as a _PDF_ file.
    pub fn save_pdf<P>(&self, pth: P) -> Result<(), DocError>
    where
        P: AsRef<Path>,
    {
        // Create a PDF document.
        let mut memory = Vec::new();
        let mut pdf = pdf::new_document(&mut memory, None);

        // Prepare font variables.
        let mut fnts: HashMap<Font, FontCollection> = HashMap::new();
        let font_mgr = FontMgr::new();

        // Segment document paragraphs into pages.
        let pags = self.seg_pags();

        // Write PDF pages.
        for pars in pags {
            pdf = self.wrt_pag(pars, pdf, &mut fnts, &font_mgr)?;
        }

        pdf.close();

        // Append file suffix.
        let file_path = pth.as_ref().with_extension("pdf");

        // Create file.
        let mut file = File::create(file_path).map_err(DocError::from)?;

        // Write doc to disk.
        file.write_all(&memory).map_err(DocError::FileError)?;

        Ok(())
    }

    /// Write a PDF page.
    pub fn wrt_pag<'a>(
        &'a self,
        pars: Vec<Par>,
        pdf: Document<'a>,
        fnts: &mut HashMap<Font, FontCollection>,
        font_mgr: &FontMgr,
    ) -> Result<Document<'a>, DocError> {
        let mut pdf_pag = pdf.begin_page(self.sze.pt(), None);

        // Write paragraphs.
        let par_wid = self.sze.width - self.mrg.width();
        let mut y: f32 = self.mrg.top.pt();
        for par in pars {
            // Determine paragraph font collection.
            let fnt = par.fnt.unwrap_or(self.fnt);
            if let Vacant(e) = fnts.entry(fnt) {
                e.insert(create_fnt_col(fnt, font_mgr)?);
            }
            let cur_fnt_col = fnts.get(&fnt).unwrap().clone();

            // Determine paragraph text style.
            let fnt_sze = par.fnt_sze.unwrap_or(self.fnt_sze);
            let mut cur_ts = TextStyle::new();
            cur_ts.set_font_families(&[par.fnt.unwrap_or(self.fnt).to_string()]);
            cur_ts.set_font_size(fnt_sze);
            cur_ts.set_height(par.spc_lne.unwrap_or(self.spc_lne).val());
            cur_ts.set_height_override(true);
            cur_ts.set_foreground_paint(&Paint::default());
            par.fnt_sty.unwrap_or(self.fnt_sty).set(&mut cur_ts);

            // Determine paragraph style.
            let mut cur_par_sty = ParagraphStyle::new();
            par.aln.unwrap_or(self.aln).set(&mut cur_par_sty);

            // Build paragraph.
            let mut par_bld = ParagraphBuilder::new(&cur_par_sty, &cur_fnt_col);
            par_bld.push_style(&cur_ts);

            // Determine paragraph first line indentation.
            if par.has_ind.unwrap_or(self.has_ind) {
                let ind = par.ind.as_ref().unwrap_or(&self.ind);
                par_bld.add_placeholder(&PlaceholderStyle {
                    width: ind.pt(),
                    height: 0.0,
                    alignment: PlaceholderAlignment::Baseline,
                    baseline_offset: 0.0,
                    baseline: TextBaseline::Alphabetic,
                });
            }

            // Add paragraph text.
            par_bld.add_text(&par.txt);

            // Layout paragraph on canvas.
            let mut paragraph = par_bld.build();

            paragraph.layout(par_wid.pt());

            // Paint paragraph to canvas.
            paragraph.paint(
                pdf_pag.canvas(),
                Point {
                    x: self.mrg.lft.pt(),
                    y,
                },
            );

            // Determine space after paragraph.
            let par_spc_aft = par.spc_aft.unwrap_or(self.spc_par_aft);
            y += paragraph.get_line_metrics_at(0).unwrap().height as f32 * par_spc_aft.val();

            // Prepare for layout of next paragraph.
            y += paragraph.height();
        }

        Ok(pdf_pag.end_page())
    }

    /// Segments `elms` into pages of paragraphs.
    pub fn seg_pags(&self) -> Vec<Vec<Par>> {
        let mut pages: Vec<Vec<Par>> = vec![];
        let mut current_page: Vec<Par> = vec![];

        for elm in &self.elms {
            match elm {
                Elm::Par(par) => current_page.push(par.clone()),
                Elm::PagBrk => {
                    // Start a new page
                    if !current_page.is_empty() {
                        pages.push(current_page);
                        current_page = vec![];
                    }
                }
            }
        }

        // Add the last page if it has any paragraphs
        if !current_page.is_empty() {
            pages.push(current_page);
        }

        pages
    }

    /// Copies and appends paragraphs from another document.
    pub fn copy_pars(&mut self, doc: Doc) {
        self.elms.extend(doc.elms.iter().cloned())
    }

    /// Adds a _paragraph_ to the end of the document.
    pub fn add_par(&mut self, par: Par) {
        self.elms.push(Elm::Par(par));
    }

    /// Adds a _page break_ to the end of the document.
    pub fn add_pag_brk(&mut self) {
        self.elms.push(Elm::PagBrk);
    }

    /// Replace text within a paragraph.
    pub fn replace_par_at(&mut self, idx: usize, from: &str, to: &str) {
        if let Some(Elm::Par(ref mut par)) = self.elms.get_mut(idx) {
            par.txt = par.txt.replace(from, to);
        }
    }

    /// Sets the _size_ of the document.
    ///
    /// ### Arguments
    ///
    /// * `sze` - The new size of the document.
    ///
    /// ### Returns
    ///
    /// Self with updated size.
    pub fn set_sze(mut self, sze: Sze) -> Self {
        self.sze = sze;
        self
    }

    /// Sets the _margin_ lengths of the document.
    ///
    /// ### Arguments
    ///
    /// * `mrg` - The new margin lengths.
    ///
    /// ### Returns
    ///
    /// Self with updated margins.
    pub fn set_mrg(mut self, mrg: Mrg) -> Self {
        self.mrg = mrg;
        self
    }

    /// Sets the _indentation_ length of a paragraph's first line.
    ///
    /// ### Arguments
    ///
    /// * `ind` - The new indentation length.
    ///
    /// ### Returns
    ///
    /// Self with updated indentation length.
    pub fn set_ind(mut self, ind: In) -> Self {
        self.ind = ind;
        self
    }

    /// Sets the _font_ for the paragraph.
    ///
    /// ### Arguments
    ///
    /// * `fnt` - The new font.
    ///
    /// ### Returns
    ///
    /// Self with updated font.
    pub fn set_fnt(mut self, fnt: Font) -> Self {
        self.fnt = fnt;
        self
    }

    /// Sets the _font size_ in points.
    ///
    /// ### Arguments
    ///
    /// * `fnt_sze` - The new font size.
    ///
    /// ### Returns
    ///
    /// Self with updated font size.
    pub fn set_fnt_sze(mut self, fnt_sze: f32) -> Self {
        self.fnt_sze = fnt_sze;
        self
    }

    /// Sets the _font style_ of the document.
    ///
    /// ### Arguments
    ///
    /// * `sty` - The new text style.
    ///
    /// ### Returns
    ///
    /// Self with updated text style.
    pub fn set_fnt_sty(mut self, sty: Style) -> Self {
        self.fnt_sty = sty;
        self
    }

    /// Sets the _text alignment_ of the document.
    ///
    /// ### Arguments
    ///
    /// * `aln` - The new text alignment.
    ///
    /// ### Returns
    ///
    /// Self with updated text alignment.
    pub fn set_aln(mut self, aln: Align) -> Self {
        self.aln = aln;
        self
    }

    /// Sets the _line spacing_ of the document.
    ///
    /// ### Arguments
    ///
    /// * `spc_lne` - The new line spacing.
    ///
    /// ### Returns
    ///
    /// Self with updated line spacing.
    pub fn set_spc_lne(mut self, spc_lne: LineSpace) -> Self {
        self.spc_lne = spc_lne;
        self
    }

    /// Sets the _spacing after_ paragraphs in the document.
    ///
    /// ### Arguments
    ///
    /// * `spc_par_aft` - The new line spacing after paragraphs. Can be set using the `LineSpace` enum.
    ///
    /// ### Returns
    ///
    /// Self with updated spacing after paragraphs.
    pub fn set_spc_par_aft(mut self, spc_par_aft: LineSpace) -> Self {
        self.spc_par_aft = spc_par_aft;
        self
    }

    /// Sets whether the first line of a paragraph is _indented_.
    ///
    /// ### Arguments
    ///
    /// * `has_ind` - `true` if the first line is indented, `false` otherwise.
    ///
    /// ### Returns
    ///
    /// Self with updated indentation setting.
    pub fn set_has_ind(mut self, has_ind: bool) -> Self {
        self.has_ind = has_ind;
        self
    }
}

/// Determines the style of text in a paragraph.
///
/// - `Normal`: The default, unstyled text.
/// - `Italic`: Text that's styled with an italic font.
/// - `Bold`: Text that's styled with a bold font.
/// - `BoldItalic`: Text that's styled with both bold and italic fonts.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// The default, unstyled text.
    #[default]
    Normal,
    /// Text that's styled with an italic font.
    Italic,
    /// Text that's styled with a bold font.
    Bold,
    /// Text that's styled with both bold and italic fonts.
    BoldItalic,
}

impl Style {
    pub fn set(self, ts: &mut TextStyle) {
        match self {
            Style::Normal => ts.set_font_style(FontStyle::normal()),
            Style::Italic => ts.set_font_style(FontStyle::italic()),
            Style::Bold => ts.set_font_style(FontStyle::bold()),
            Style::BoldItalic => ts.set_font_style(FontStyle::bold_italic()),
        };
    }
}

/// Determines _horizontal_ text alignment of a paragraph.
///
/// - `Left`: Aligns text to the left edge of the paragraph.
/// - `Right`: Aligns text to the right edge of the paragraph.
/// - `Center`: Centers the text horizontally within the paragraph.
/// - `Justify`: Stretches the text to ensure that each line has
///     equal width. The last line is aligned to the left.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    /// Aligns text to the left edge of the paragraph.
    #[default]
    Left,
    /// Aligns text to the right edge of the paragraph.
    Right,
    /// Centers the text horizontally within the paragraph.
    Center,
    /// Stretches the text to ensure that each line has
    /// equal width.
    ///
    /// The last line is aligned to the left.
    Justify,
}

impl Align {
    pub fn set(self, ps: &mut ParagraphStyle) {
        match self {
            Align::Left => ps.set_text_align(TextAlign::Left),
            Align::Right => ps.set_text_align(TextAlign::Right),
            Align::Center => ps.set_text_align(TextAlign::Center),
            Align::Justify => ps.set_text_align(TextAlign::Justify),
        };
    }
}

/// Determines the amount of space between lines of a paragraph.
///
/// - `Single`: Single line spacing.
/// - `Double`: Double line spacing.
/// - `Custom(f32)`: Custom line spacing specified by a floating-point value.
///
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum LineSpace {
    #[default]
    Single,
    Double,
    Custom(f32),
}

impl LineSpace {
    pub fn val(self) -> f32 {
        match self {
            LineSpace::Single => 1.0,
            LineSpace::Double => 2.0,
            LineSpace::Custom(val) => val,
        }
    }
}

/// A _paragraph_ with formatting options.
///
/// Formatting options are inherited from the document.
///
/// Setting a paragraph option overrides a doc setting.
///
/// ### Fields
///
/// - `fnt`: Optional font for the paragraph. This is specified as a `Font` type.
/// - `fnt_sze`: Optional size of the font in points. This is specified as a `f32`.
/// - `sty`: Optional text _style_ of the paragraph. Possible values are defined in the `Style` enum.
/// - `aln`: Optional text _alignment_ of the paragraph. Possible values are defined in the `Align` enum.
/// - `lne_spc`: Optional line spacing of the paragraph. Possible values are defined in the `LineSpace` enum.
/// - `has_spc_bfr`: Indicates whether there is _space before_ the paragraph. `Some(true)` if there is space before, `Some(false)` otherwise, or `None` if not specified.
/// - `has_spc_aft`: Indicates whether there is _space after_ the paragraph. `Some(true)` if there is space after, `Some(false)` otherwise, or `None` if not specified.
/// - `has_ind`: Indicates whether the first line is _indented_. `Some(true)` if the first line is indented, `Some(false)` otherwise, or `None` if not specified.
/// - `txt`: Text _content_ of the paragraph, specified as a `String`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Par {
    /// Indentation length of the first line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ind: Option<In>,
    /// Font for the paragraph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnt: Option<Font>,
    /// The size of the font in points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnt_sze: Option<f32>,
    /// Font _style_ of the paragraph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnt_sty: Option<Style>,
    /// Text _alignment_ of the paragraph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aln: Option<Align>,
    /// Line spacing of a paragraph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spc_lne: Option<LineSpace>,
    /// Spacing _after_ the paragraph.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spc_aft: Option<LineSpace>,
    /// Indicates whether the first line is _indented_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ind: Option<bool>,
    /// Text _content_ of the paragraph.
    pub txt: String,
}

/// Creates a paragraph with the given text.
pub fn par(txt: &str) -> Par {
    Par::default().set_txt(txt.into())
}

impl Par {
    /// Replaces all matches of a pattern with another string.
    pub fn replace(&mut self, from: &str, to: &str) {
        self.txt = self.txt.replace(from, to)
    }

    /// Sets the _indentation_ length of the first line.
    ///
    /// ### Arguments
    ///
    /// * `ind` - The new indentation length.
    ///
    /// ### Returns
    ///
    /// Self with updated indentation length.
    pub fn set_ind(mut self, ind: Option<In>) -> Self {
        self.ind = ind;
        self
    }

    /// Sets the _font_ for the paragraph.
    ///
    /// ### Arguments
    ///
    /// * `fnt` - The new font.
    ///
    /// ### Returns
    ///
    /// Self with updated font.
    pub fn set_fnt(mut self, fnt: Option<Font>) -> Self {
        self.fnt = fnt;
        self
    }

    /// Sets the _size_ of the font in points.
    ///
    /// ### Arguments
    ///
    /// * `fnt_sze` - The new font size.
    ///
    /// ### Returns
    ///
    /// Self with updated font size.
    pub fn set_fnt_sze(mut self, fnt_sze: Option<f32>) -> Self {
        self.fnt_sze = fnt_sze;
        self
    }

    /// Sets the _font style_ of the paragraph.
    ///
    /// ### Arguments
    ///
    /// * `sty` - The new style.
    ///
    /// ### Returns
    ///
    /// Self with updated style.
    pub fn set_fnt_sty(mut self, sty: Option<Style>) -> Self {
        self.fnt_sty = sty;
        self
    }

    /// Sets the _alignment_ of the paragraph.
    ///
    /// ### Arguments
    ///
    /// * `aln` - The new alignment.
    ///
    /// ### Returns
    ///
    /// Self with updated alignment.
    pub fn set_aln(mut self, aln: Option<Align>) -> Self {
        self.aln = aln;
        self
    }

    /// Sets the _line spacing_ of the paragraph.
    ///
    /// ### Arguments
    ///
    /// * `spc_lne` - The new line spacing.
    ///
    /// ### Returns
    ///
    /// Self with updated line spacing.
    pub fn set_spc_lne(mut self, spc_lne: Option<LineSpace>) -> Self {
        self.spc_lne = spc_lne;
        self
    }

    /// Sets the _spacing after_ the paragraph.
    //
    /// ### Arguments
    //
    /// * `spc_aft` - The new spacing after the paragraph. Can be set using the `LineSpace` enum.
    //
    /// ### Returns
    ///
    /// Self with updated spacing after the paragraph.
    pub fn set_spc_aft(mut self, spc_aft: Option<LineSpace>) -> Self {
        self.spc_aft = spc_aft;
        self
    }

    /// Sets whether the first line of the paragraph is _indented_.
    ///
    /// ### Arguments
    ///
    /// * `has_ind` - `true` if the first line is indented, `false` otherwise.
    ///
    /// ### Returns
    ///
    /// Self with updated indentation setting.
    pub fn set_has_ind(mut self, has_ind: Option<bool>) -> Self {
        self.has_ind = has_ind;
        self
    }

    /// Sets the _text content_ of the paragraph.
    ///
    /// ### Arguments
    ///
    /// * `txt` - The new text content.
    ///
    /// ### Returns
    ///
    /// Self with updated text content.
    pub fn set_txt(mut self, txt: String) -> Self {
        self.txt = txt;
        self
    }
}

pub fn create_fnt_col(font: Font, font_mgr: &FontMgr) -> Result<FontCollection, DocError> {
    // Get font data from network or cache.
    let font_data = font.get_with_cache().map_err(DocError::from)?;

    // Load typeface from font data.
    if let Some(typeface) = font_mgr.new_from_data(&font_data, None) {
        // Create a font collection.
        let mut tfp = TypefaceFontProvider::new();
        tfp.register_typeface(typeface, Some(font.to_string().as_str()));
        let mut fnt_col = FontCollection::new();
        fnt_col.set_default_font_manager(Some(tfp.into()), None);
        return Ok(fnt_col);
    }

    Err(DocError::from(
        format!("Unable to parse font `{}`.", font).as_str(),
    ))
}

/// Elements of a [`Doc`].
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Elm {
    /// A _paragraph_ element.
    Par(Par),
    /// A _page break_ element.
    PagBrk,
}
