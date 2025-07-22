use anyhow;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::path::Path;

mod bindings {
    #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct VerovioToolkit {
    tk_ptr: *mut ::std::os::raw::c_void,
}

impl Drop for VerovioToolkit {
    fn drop(&mut self) {
        unsafe {
            bindings::vrvToolkit_destructor(self.tk_ptr);
        }
    }
}

impl VerovioToolkit {
    pub fn new() -> VerovioToolkit {
        VerovioToolkit {
            tk_ptr: unsafe { bindings::vrvToolkit_constructor() },
        }
    }

    pub fn with_resource(path: impl AsRef<Path>) -> anyhow::Result<VerovioToolkit> {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = CString::new(path)?;
        Ok(VerovioToolkit {
            tk_ptr: unsafe { bindings::vrvToolkit_constructorResourcePath(path.as_ptr()) },
        })
    }

    pub fn enable_log(enable: bool) {
        unsafe {
            bindings::enableLog(enable);
        }
    }

    pub fn enable_log_to_buffer(enable: bool) {
        unsafe {
            bindings::enableLogToBuffer(enable);
        }
    }

    pub fn edit(&self, action: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(self.tk_ptr, action, bindings::vrvToolkit_edit)
    }

    pub fn edit_info(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_editInfo)
    }

    pub fn get_available_options(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getAvailableOptions)
    }

    pub fn get_default_options(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getDefaultOptions)
    }

    pub fn get_descriptive_features(&self, p: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, p, bindings::vrvToolkit_getDescriptiveFeatures)
    }

    pub fn get_element_attr(&self, p: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, p, bindings::vrvToolkit_getElementAttr)
    }

    pub fn get_expansion_ids_for_element(&self, p: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(
            self.tk_ptr,
            p,
            bindings::vrvToolkit_getExpansionIdsForElement,
        )
    }

    pub fn get_humdrum(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getHumdrum)
    }

    pub fn get_humdrum_file(&self, p: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(self.tk_ptr, p, bindings::vrvToolkit_getHumdrumFile)
    }

    pub fn get_elements_at_time(&self, p: i32) -> String {
        unsafe {
            let ret = bindings::vrvToolkit_getElementsAtTime(self.tk_ptr, p);
            CStr::from_ptr(ret).to_string_lossy().into_owned()
        }
    }

    pub fn get_id(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getID)
    }

    pub fn convert_humdrum_to_humdrum(&self, p: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, p, bindings::vrvToolkit_convertHumdrumToHumdrum)
    }

    pub fn convert_humdrum_to_midi(&self, p: impl AsRef<str>) -> anyhow::Result<Vec<u8>> {
        let ret = get_string2(self.tk_ptr, p, bindings::vrvToolkit_convertHumdrumToMIDI)
            .map(|s| BASE64_STANDARD.decode(s.as_bytes()))??;
        Ok(ret)
    }
    pub fn convert_midi_to_humdrum(&self, p: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, p, bindings::vrvToolkit_convertMEIToHumdrum)
    }

    pub fn get_log(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getLog)
    }

    pub fn get_mei(&self, options: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, options, bindings::vrvToolkit_getMEI)
    }

    pub fn get_midi_values_for_element(&self, p: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, p, bindings::vrvToolkit_getMIDIValuesForElement)
    }

    pub fn get_notated_id_for_element(&self, p: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, p, bindings::vrvToolkit_getNotatedIdForElement)
    }

    pub fn get_options(&self) -> anyhow::Result<AllOptions> {
        let s = get_string(self.tk_ptr, bindings::vrvToolkit_getOptions);
        let options = serde_json::from_str(&s)?;
        Ok(options)
    }

    pub fn get_option_usage(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getOptionUsageString)
    }

    pub fn get_page_count(&self) -> i32 {
        unsafe { bindings::vrvToolkit_getPageCount(self.tk_ptr) }
    }

    pub fn get_page_with_element(&self, p: impl AsRef<str>) -> anyhow::Result<i32> {
        get_primitive2(self.tk_ptr, p, bindings::vrvToolkit_getPageWithElement)
    }

    pub fn get_resource_path(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getResourcePath)
    }

    pub fn get_scale(&self) -> i32 {
        unsafe { bindings::vrvToolkit_getScale(self.tk_ptr) }
    }

    pub fn get_time_for_element(&self, p: impl AsRef<str>) -> anyhow::Result<f64> {
        get_primitive2(self.tk_ptr, p, bindings::vrvToolkit_getTimeForElement)
    }

    pub fn get_times_for_element(&self, p: impl AsRef<str>) -> anyhow::Result<ElementTime> {
        let ret = get_string2(self.tk_ptr, p, bindings::vrvToolkit_getTimesForElement)
            .map(|s| serde_json::from_str(&s))??;
        Ok(ret)
    }
    pub fn get_version(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getVersion)
    }

    pub fn load_data(&self, p: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(self.tk_ptr, p, bindings::vrvToolkit_loadData)
    }

    pub fn load_file(&self, path: impl AsRef<Path>) -> anyhow::Result<bool> {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = CString::new(path)?;
        unsafe { Ok(bindings::vrvToolkit_loadFile(self.tk_ptr, path.as_ptr())) }
    }

    pub fn load_zip_data(&self, p: impl AsRef<[u8]>) -> anyhow::Result<bool> {
        unsafe {
            let p = p.as_ref();
            Ok(bindings::vrvToolkit_loadZipDataBuffer(
                self.tk_ptr,
                p.as_ptr() as _,
                p.len() as _,
            ))
        }
    }

    pub fn redo_layout(&self, p: impl AsRef<str>) -> anyhow::Result<()> {
        get_primitive2(self.tk_ptr, p, bindings::vrvToolkit_redoLayout)
    }

    pub fn redo_page_pitch_pos_layout(&self) {
        unsafe {
            bindings::vrvToolkit_redoPagePitchPosLayout(self.tk_ptr);
        }
    }

    pub fn render_data(
        &self,
        data: impl AsRef<str>,
        option: impl AsRef<str>,
    ) -> anyhow::Result<String> {
        get_string3(self.tk_ptr, data, option, bindings::vrvToolkit_renderData)
    }

    pub fn render_to_expansion_map(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_renderToExpansionMap)
    }

    pub fn render_to_expansion_map_file(&self, p: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(
            self.tk_ptr,
            p,
            bindings::vrvToolkit_renderToExpansionMapFile,
        )
    }

    pub fn render_to_midi(&self) -> anyhow::Result<Vec<u8>> {
        let s = get_string(self.tk_ptr, bindings::vrvToolkit_renderToMIDI);
        let ret = BASE64_STANDARD.decode(s.as_bytes())?;
        Ok(ret)
    }

    pub fn render_to_midi_file(&self, p: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(self.tk_ptr, p, bindings::vrvToolkit_renderToMIDIFile)
    }

    pub fn render_to_pae(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_renderToPAE)
    }

    pub fn render_to_pae_file(&self, p: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(self.tk_ptr, p, bindings::vrvToolkit_renderToPAEFile)
    }

    pub fn render_to_svg(&self, page_no: i32, xml_declaration: bool) -> String {
        unsafe {
            let ret = bindings::vrvToolkit_renderToSVG(self.tk_ptr, page_no, xml_declaration);
            CStr::from_ptr(ret).to_string_lossy().into_owned()
        }
    }

    pub fn render_to_svg_file(&self, path: impl AsRef<Path>, page_no: i32) -> anyhow::Result<bool> {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = CString::new(path)?;
        unsafe {
            Ok(bindings::vrvToolkit_renderToSVGFile(
                self.tk_ptr,
                path.as_ptr(),
                page_no,
            ))
        }
    }

    pub fn render_to_timemap(&self, options: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, options, bindings::vrvToolkit_renderToTimemap)
    }

    pub fn render_to_timemap_file(
        &self,
        path: impl AsRef<Path>,
        options: impl AsRef<str>,
    ) -> anyhow::Result<bool> {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = CString::new(path)?;
        let options = CString::new(options.as_ref())?;
        unsafe {
            Ok(bindings::vrvToolkit_renderToTimemapFile(
                self.tk_ptr,
                path.as_ptr(),
                options.as_ptr(),
            ))
        }
    }

    pub fn reset_options(&self) {
        unsafe {
            bindings::vrvToolkit_resetOptions(self.tk_ptr);
        }
    }

    pub fn reset_xml_seed(&self, seed: i32) {
        unsafe {
            bindings::vrvToolkit_resetXmlIdSeed(self.tk_ptr, seed);
        }
    }

    pub fn save_file(
        &self,
        path: impl AsRef<Path>,
        options: impl AsRef<str>,
    ) -> anyhow::Result<bool> {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = CString::new(path)?;
        let options = CString::new(options.as_ref())?;
        unsafe {
            Ok(bindings::vrvToolkit_saveFile(
                self.tk_ptr,
                path.as_ptr(),
                options.as_ptr(),
            ))
        }
    }

    pub fn select(&self, select: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(self.tk_ptr, select, bindings::vrvToolkit_select)
    }

    pub fn set_input_from(&self, input: Format) -> anyhow::Result<bool> {
        let input: &str = input.into();
        get_primitive2(self.tk_ptr, input, bindings::vrvToolkit_setInputFrom)
    }

    pub fn set_options(&self, options: &AllOptions) -> anyhow::Result<bool> {
        let options = serde_json::to_string(options)?;
        get_primitive2(self.tk_ptr, &options, bindings::vrvToolkit_setOptions)
    }

    pub fn set_output_to(&self, format: Format) -> anyhow::Result<bool> {
        let format: &str = format.into();
        get_primitive2(self.tk_ptr, format, bindings::vrvToolkit_setOutputTo)
    }

    pub fn set_resource_path(&self, path: impl AsRef<Path>) -> anyhow::Result<bool> {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = CString::new(path)?;
        unsafe {
            Ok(bindings::vrvToolkit_setResourcePath(
                self.tk_ptr,
                path.as_ptr(),
            ))
        }
    }

    pub fn set_scale(&self, scale: i32) -> bool {
        unsafe { bindings::vrvToolkit_setScale(self.tk_ptr, scale) }
    }

    pub fn validate_pae(&self, pae: impl AsRef<str>) -> anyhow::Result<String> {
        get_string2(self.tk_ptr, pae, bindings::vrvToolkit_validatePAE)
    }

    pub fn validate_pae_file(&self, path: impl AsRef<Path>) -> anyhow::Result<String> {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = CString::new(path)?;
        unsafe {
            let ret = bindings::vrvToolkit_validatePAEFile(self.tk_ptr, path.as_ptr());
            Ok(CStr::from_ptr(ret).to_string_lossy().into_owned())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ElementTime {
    #[serde(rename = "qfracOn")]
    pub score_time_onset: Vec<[i32; 2]>,
    #[serde(rename = "qfracOff")]
    pub score_time_offset: Vec<[i32; 2]>,
    #[serde(rename = "qfracDuration")]
    pub score_time_duration: Vec<[i32; 2]>,
    #[serde(rename = "qfracTiedDuration")]
    pub score_time_tied_duration: Vec<[i32; 2]>,
    #[serde(rename = "tstampOn")]
    pub real_time_onset_milliseconds: Vec<f64>,
    #[serde(rename = "tstampOff")]
    pub real_time_offset_milliseconds: Vec<f64>,
}

pub enum Format {
    Abc,
    Pae,
    Darms,
    VolPiano,
    Cmme,
    Humdrum,
    Mei,
    MusicXml,
    MuseDataHum,
    MeiHum,
    Esac,
    Serialization,
    Auto,
    MusicxmlHum,
}

impl From<Format> for &'static str {
    fn from(input: Format) -> Self {
        match input {
            Format::Abc => "abc",
            Format::Pae => "pae",
            Format::Cmme => "cmme.xml",
            Format::Humdrum => "humdrum",
            Format::Mei => "mei",
            Format::MusicXml => "musicxml",
            Format::MuseDataHum => "musedata",
            Format::MeiHum => "mei-hum",
            Format::Esac => "esac",
            Format::Serialization => "mei-pb-serialized",
            Format::Auto => "auto",
            Format::MusicxmlHum => "musicxml-hum",
            Format::Darms => "darms",
            Format::VolPiano => "volpiano",
        }
    }
}

fn get_string(
    p: *mut ::std::os::raw::c_void,
    f: unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char,
) -> String {
    unsafe {
        let r = f(p);
        CStr::from_ptr(r).to_string_lossy().into_owned()
    }
}

fn get_string2(
    p: *mut ::std::os::raw::c_void,
    p1: impl AsRef<str>,
    f: unsafe extern "C" fn(
        *mut ::std::os::raw::c_void,
        *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,
) -> anyhow::Result<String> {
    unsafe {
        let p1 = CString::new(p1.as_ref())?;
        let r = f(p, p1.as_ptr());
        Ok(CStr::from_ptr(r).to_string_lossy().into_owned())
    }
}

fn get_string3(
    p: *mut ::std::os::raw::c_void,
    p1: impl AsRef<str>,
    p2: impl AsRef<str>,
    f: unsafe extern "C" fn(
        *mut ::std::os::raw::c_void,
        *const ::std::os::raw::c_char,
        *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,
) -> anyhow::Result<String> {
    unsafe {
        let p1 = CString::new(p1.as_ref())?;
        let p2 = CString::new(p2.as_ref())?;
        let r = f(p, p1.as_ptr(), p2.as_ptr());
        Ok(CStr::from_ptr(r).to_string_lossy().into_owned())
    }
}

fn get_primitive2<T>(
    p: *mut ::std::os::raw::c_void,
    p1: impl AsRef<str>,
    f: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *const ::std::os::raw::c_char) -> T,
) -> anyhow::Result<T> {
    unsafe {
        let p1 = CString::new(p1.as_ref())?;
        Ok(f(p, p1.as_ptr()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllOptions {
    #[serde(rename = "inputFrom")]
    pub input_from: Option<String>,
    #[serde(rename = "outputTo")]
    pub output_to: Option<String>,
    #[serde(rename = "scale")]
    pub scale: Option<i32>,
    #[serde(rename = "xmlIdSeed")]
    pub xml_id_seed: Option<i32>,
    #[serde(rename = "adjustPageHeight")]
    pub adjust_page_height: Option<bool>,
    #[serde(rename = "adjustPageWidth")]
    pub adjust_page_width: Option<bool>,
    #[serde(rename = "breaks")]
    pub breaks: Option<String>,
    #[serde(rename = "breaksSmartSb")]
    pub breaks_smart_sb: Option<f64>,
    #[serde(rename = "condense")]
    pub condense: Option<String>,
    #[serde(rename = "condenseFirstPage")]
    pub condense_first_page: Option<bool>,
    #[serde(rename = "condenseNotLastSystem")]
    pub condense_not_last_system: Option<bool>,
    #[serde(rename = "condenseTempoPages")]
    pub condense_tempo_pages: Option<bool>,
    #[serde(rename = "evenNoteSpacing")]
    pub even_note_spacing: Option<bool>,
    #[serde(rename = "expand")]
    pub expand: Option<String>,
    #[serde(rename = "footer")]
    pub footer: Option<String>,
    #[serde(rename = "header")]
    pub header: Option<String>,
    #[serde(rename = "humType")]
    pub hum_type: Option<bool>,
    #[serde(rename = "incip")]
    pub incip: Option<bool>,
    #[serde(rename = "justifyVertically")]
    pub justify_vertically: Option<bool>,
    #[serde(rename = "landscape")]
    pub landscape: Option<bool>,
    #[serde(rename = "minLastJustification")]
    pub min_last_justification: Option<f64>,
    #[serde(rename = "mmOutput")]
    pub mm_output: Option<bool>,
    #[serde(rename = "moveScoreDefinitionToStaff")]
    pub move_score_definition_to_staff: Option<bool>,
    #[serde(rename = "neumeAsNote")]
    pub neume_as_note: Option<bool>,
    #[serde(rename = "noJustification")]
    pub no_justification: Option<bool>,
    #[serde(rename = "openControlEvents")]
    pub open_control_events: Option<bool>,
    #[serde(rename = "outputFormatRaw")]
    pub output_format_raw: Option<bool>,
    #[serde(rename = "outputIndent")]
    pub output_indent: Option<i32>,
    #[serde(rename = "outputIndentTab")]
    pub output_indent_tab: Option<bool>,
    #[serde(rename = "outputSmuflXmlEntities")]
    pub output_smufl_xml_entities: Option<bool>,
    #[serde(rename = "pageHeight")]
    pub page_height: Option<i32>,
    #[serde(rename = "pageMarginBottom")]
    pub page_margin_bottom: Option<i32>,
    #[serde(rename = "pageMarginLeft")]
    pub page_margin_left: Option<i32>,
    #[serde(rename = "pageMarginRight")]
    pub page_margin_right: Option<i32>,
    #[serde(rename = "pageMarginTop")]
    pub page_margin_top: Option<i32>,
    #[serde(rename = "pageWidth")]
    pub page_width: Option<i32>,
    #[serde(rename = "pedalStyle")]
    pub pedal_style: Option<String>,
    #[serde(rename = "preserveAnalyticalMarkup")]
    pub preserve_analytical_markup: Option<bool>,
    #[serde(rename = "removeIds")]
    pub remove_ids: Option<bool>,
    #[serde(rename = "scaleToPageSize")]
    pub scale_to_page_size: Option<bool>,
    #[serde(rename = "setLocale")]
    pub set_locale: Option<bool>,
    #[serde(rename = "showRuntime")]
    pub show_runtime: Option<bool>,
    #[serde(rename = "shrinkToFit")]
    pub shrink_to_fit: Option<bool>,
    #[serde(rename = "smuflTextFont")]
    pub smufl_text_font: Option<String>,
    #[serde(rename = "staccatoCenter")]
    pub staccato_center: Option<bool>,
    #[serde(rename = "svgAdditionalAttribute")]
    pub svg_additional_attribute: Option<Vec<String>>,
    #[serde(rename = "svgBoundingBoxes")]
    pub svg_bounding_boxes: Option<bool>,
    #[serde(rename = "svgCss")]
    pub svg_css: Option<String>,
    #[serde(rename = "svgFormatRaw")]
    pub svg_format_raw: Option<bool>,
    #[serde(rename = "svgHtml5")]
    pub svg_html5: Option<bool>,
    #[serde(rename = "svgRemoveXlink")]
    pub svg_remove_xlink: Option<bool>,
    #[serde(rename = "svgViewBox")]
    pub svg_view_box: Option<bool>,
    #[serde(rename = "unit")]
    pub unit: Option<f64>,
    #[serde(rename = "useBraceGlyph")]
    pub use_brace_glyph: Option<bool>,
    #[serde(rename = "useFacsimile")]
    pub use_facsimile: Option<bool>,
    #[serde(rename = "usePgFooterForAll")]
    pub use_pg_footer_for_all: Option<bool>,
    #[serde(rename = "usePgHeaderForAll")]
    pub use_pg_header_for_all: Option<bool>,
    #[serde(rename = "xmlIdChecksum")]
    pub xml_id_checksum: Option<bool>,
    #[serde(rename = "barLineSeparation")]
    pub bar_line_separation: Option<f64>,
    #[serde(rename = "barLineWidth")]
    pub bar_line_width: Option<f64>,
    #[serde(rename = "beamFrenchStyle")]
    pub beam_french_style: Option<bool>,
    #[serde(rename = "beamMaxSlope")]
    pub beam_max_slope: Option<i32>,
    #[serde(rename = "beamMixedPreserve")]
    pub beam_mixed_preserve: Option<bool>,
    #[serde(rename = "beamMixedStemMin")]
    pub beam_mixed_stem_min: Option<f64>,
    #[serde(rename = "bracketThickness")]
    pub bracket_thickness: Option<f64>,
    #[serde(rename = "breaksNoWidow")]
    pub breaks_no_widow: Option<bool>,
    #[serde(rename = "dashedBarLineDashLength")]
    pub dashed_bar_line_dash_length: Option<f64>,
    #[serde(rename = "dashedBarLineGapLength")]
    pub dashed_bar_line_gap_length: Option<f64>,
    #[serde(rename = "dynamDist")]
    pub dynam_dist: Option<f64>,
    #[serde(rename = "dynamSingleGlyphs")]
    pub dynam_single_glyphs: Option<bool>,
    #[serde(rename = "extenderLineMinSpace")]
    pub extender_line_min_space: Option<f64>,
    #[serde(rename = "fingeringScale")]
    pub fingering_scale: Option<f64>,
    #[serde(rename = "font")]
    pub font: Option<String>,
    #[serde(rename = "fontAddCustom")]
    pub font_add_custom: Option<Vec<String>>,
    #[serde(rename = "fontFallback")]
    pub font_fallback: Option<String>,
    #[serde(rename = "fontLoadAll")]
    pub font_load_all: Option<bool>,
    #[serde(rename = "graceFactor")]
    pub grace_factor: Option<f64>,
    #[serde(rename = "graceRhythmAlign")]
    pub grace_rhythm_align: Option<bool>,
    #[serde(rename = "graceRightAlign")]
    pub grace_right_align: Option<bool>,
    #[serde(rename = "hairpinSize")]
    pub hairpin_size: Option<f64>,
    #[serde(rename = "hairpinThickness")]
    pub hairpin_thickness: Option<f64>,
    #[serde(rename = "handwrittenFont")]
    pub handwritten_font: Option<Vec<String>>,
    #[serde(rename = "harmDist")]
    pub harm_dist: Option<f64>,
    #[serde(rename = "justificationBraceGroup")]
    pub justification_brace_group: Option<f64>,
    #[serde(rename = "justificationBracketGroup")]
    pub justification_bracket_group: Option<f64>,
    #[serde(rename = "justificationMaxVertical")]
    pub justification_max_vertical: Option<f64>,
    #[serde(rename = "justificationStaff")]
    pub justification_staff: Option<f64>,
    #[serde(rename = "justificationSystem")]
    pub justification_system: Option<f64>,
    #[serde(rename = "ledgerLineExtension")]
    pub ledger_line_extension: Option<f64>,
    #[serde(rename = "ledgerLineThickness")]
    pub ledger_line_thickness: Option<f64>,
    #[serde(rename = "lyricElision")]
    pub lyric_elision: Option<String>,
    #[serde(rename = "lyricHeightFactor")]
    pub lyric_height_factor: Option<f64>,
    #[serde(rename = "lyricLineThickness")]
    pub lyric_line_thickness: Option<f64>,
    #[serde(rename = "lyricNoStartHyphen")]
    pub lyric_no_start_hyphen: Option<bool>,
    #[serde(rename = "lyricSize")]
    pub lyric_size: Option<f64>,
    #[serde(rename = "lyricTopMinMargin")]
    pub lyric_top_min_margin: Option<f64>,
    #[serde(rename = "lyricVerseCollapse")]
    pub lyric_verse_collapse: Option<bool>,
    #[serde(rename = "lyricWordSpace")]
    pub lyric_word_space: Option<f64>,
    #[serde(rename = "measureMinWidth")]
    pub measure_min_width: Option<i32>,
    #[serde(rename = "mnumInterval")]
    pub mnum_interval: Option<i32>,
    #[serde(rename = "multiRestStyle")]
    pub multi_rest_style: Option<String>,
    #[serde(rename = "multiRestThickness")]
    pub multi_rest_thickness: Option<f64>,
    #[serde(rename = "octaveAlternativeSymbols")]
    pub octave_alternative_symbols: Option<bool>,
    #[serde(rename = "octaveLineThickness")]
    pub octave_line_thickness: Option<f64>,
    #[serde(rename = "octaveNoSpanningParentheses")]
    pub octave_no_spanning_parentheses: Option<bool>,
    #[serde(rename = "pedalLineThickness")]
    pub pedal_line_thickness: Option<f64>,
    #[serde(rename = "repeatBarLineDotSeparation")]
    pub repeat_bar_line_dot_separation: Option<f64>,
    #[serde(rename = "repeatEndingLineThickness")]
    pub repeat_ending_line_thickness: Option<f64>,
    #[serde(rename = "slurCurveFactor")]
    pub slur_curve_factor: Option<f64>,
    #[serde(rename = "slurEndpointFlexibility")]
    pub slur_endpoint_flexibility: Option<f64>,
    #[serde(rename = "slurEndpointThickness")]
    pub slur_endpoint_thickness: Option<f64>,
    #[serde(rename = "slurMargin")]
    pub slur_margin: Option<f64>,
    #[serde(rename = "slurMaxSlope")]
    pub slur_max_slope: Option<i32>,
    #[serde(rename = "slurMidpointThickness")]
    pub slur_midpoint_thickness: Option<f64>,
    #[serde(rename = "slurSymmetry")]
    pub slur_symmetry: Option<f64>,
    #[serde(rename = "spacingBraceGroup")]
    pub spacing_brace_group: Option<i32>,
    #[serde(rename = "spacingBracketGroup")]
    pub spacing_bracket_group: Option<i32>,
    #[serde(rename = "spacingDurDetection")]
    pub spacing_dur_detection: Option<bool>,
    #[serde(rename = "spacingLinear")]
    pub spacing_linear: Option<f64>,
    #[serde(rename = "spacingNonLinear")]
    pub spacing_non_linear: Option<f64>,
    #[serde(rename = "spacingStaff")]
    pub spacing_staff: Option<i32>,
    #[serde(rename = "spacingSystem")]
    pub spacing_system: Option<i32>,
    #[serde(rename = "staffLineWidth")]
    pub staff_line_width: Option<f64>,
    #[serde(rename = "stemWidth")]
    pub stem_width: Option<f64>,
    #[serde(rename = "subBracketThickness")]
    pub sub_bracket_thickness: Option<f64>,
    #[serde(rename = "systemDivider")]
    pub system_divider: Option<String>,
    #[serde(rename = "systemMaxPerPage")]
    pub system_max_per_page: Option<i32>,
    #[serde(rename = "textEnclosureThickness")]
    pub text_enclosure_thickness: Option<f64>,
    #[serde(rename = "thickBarlineThickness")]
    pub thick_barline_thickness: Option<f64>,
    #[serde(rename = "tieEndpointThickness")]
    pub tie_endpoint_thickness: Option<f64>,
    #[serde(rename = "tieMidpointThickness")]
    pub tie_midpoint_thickness: Option<f64>,
    #[serde(rename = "tieMinLength")]
    pub tie_min_length: Option<f64>,
    #[serde(rename = "tupletAngledOnBeams")]
    pub tuplet_angled_on_beams: Option<bool>,
    #[serde(rename = "tupletBracketThickness")]
    pub tuplet_bracket_thickness: Option<f64>,
    #[serde(rename = "tupletNumHead")]
    pub tuplet_num_head: Option<bool>,
    #[serde(rename = "appXPathQuery")]
    pub app_xpath_query: Option<Vec<String>>,
    #[serde(rename = "choiceXPathQuery")]
    pub choice_xpath_query: Option<Vec<String>>,
    #[serde(rename = "loadSelectedMdivOnly")]
    pub load_selected_mdiv_only: Option<bool>,
    #[serde(rename = "mdivAll")]
    pub mdiv_all: Option<bool>,
    #[serde(rename = "mdivXPathQuery")]
    pub mdiv_xpath_query: Option<String>,
    #[serde(rename = "substXPathQuery")]
    pub subst_xpath_query: Option<Vec<String>>,
    #[serde(rename = "transpose")]
    pub transpose: Option<String>,
    #[serde(rename = "transposeMdiv")]
    pub transpose_mdiv: Option<TransposeMDiv>,
    #[serde(rename = "transposeSelectedOnly")]
    pub transpose_selected_only: Option<bool>,
    #[serde(rename = "transposeToSoundingPitch")]
    pub transpose_to_sounding_pitch: Option<bool>,
    #[serde(rename = "bottomMarginArtic")]
    pub bottom_margin_artic: Option<f64>,
    #[serde(rename = "bottomMarginHarm")]
    pub bottom_margin_harm: Option<f64>,
    #[serde(rename = "bottomMarginHeader")]
    pub bottom_margin_header: Option<f64>,
    #[serde(rename = "bottomMarginOctave")]
    pub bottom_margin_octave: Option<f64>,
    #[serde(rename = "defaultBottomMargin")]
    pub default_bottom_margin: Option<f64>,
    #[serde(rename = "defaultLeftMargin")]
    pub default_left_margin: Option<f64>,
    #[serde(rename = "defaultRightMargin")]
    pub default_right_margin: Option<f64>,
    #[serde(rename = "defaultTopMargin")]
    pub default_top_margin: Option<f64>,
    #[serde(rename = "leftMarginAccid")]
    pub left_margin_accid: Option<f64>,
    #[serde(rename = "leftMarginBarLine")]
    pub left_margin_bar_line: Option<f64>,
    #[serde(rename = "leftMarginBeatRpt")]
    pub left_margin_beat_rpt: Option<f64>,
    #[serde(rename = "leftMarginChord")]
    pub left_margin_chord: Option<f64>,
    #[serde(rename = "leftMarginClef")]
    pub left_margin_clef: Option<f64>,
    #[serde(rename = "leftMarginKeySig")]
    pub left_margin_key_sig: Option<f64>,
    #[serde(rename = "leftMarginLeftBarLine")]
    pub left_margin_left_bar_line: Option<f64>,
    #[serde(rename = "leftMarginMRest")]
    pub left_margin_m_rest: Option<f64>,
    #[serde(rename = "leftMarginMRpt2")]
    pub left_margin_m_rpt2: Option<f64>,
    #[serde(rename = "leftMarginMensur")]
    pub left_margin_mensur: Option<f64>,
    #[serde(rename = "leftMarginMeterSig")]
    pub left_margin_meter_sig: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransposeMDiv {}
