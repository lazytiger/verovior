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

    pub fn set_options(&self, options: impl AsRef<AllOptions>) -> anyhow::Result<bool> {
        let options = serde_json::to_string(options.as_ref())?;
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
    #[serde(rename = "allPages")]
    all_pages: Option<bool>,
    #[serde(rename = "help")]
    help: Option<String>,
    #[serde(rename = "inputFrom")]
    input_from: Option<String>,
    #[serde(rename = "logLevel")]
    log_level: Option<String>,
    #[serde(rename = "outfile")]
    outfile: Option<String>,
    #[serde(rename = "outputTo")]
    output_to: Option<String>,
    #[serde(rename = "page")]
    page: Option<i32>,
    #[serde(rename = "resourcePath")]
    resource_path: Option<String>,
    #[serde(rename = "scale")]
    scale: Option<i32>,
    #[serde(rename = "stdin")]
    stdin: Option<bool>,
    #[serde(rename = "version")]
    version: Option<bool>,
    #[serde(rename = "xmlIdSeed")]
    xml_id_seed: Option<i32>,
    #[serde(rename = "adjustPageHeight")]
    adjust_page_height: Option<bool>,
    #[serde(rename = "adjustPageWidth")]
    adjust_page_width: Option<bool>,
    #[serde(rename = "breaks")]
    breaks: Option<String>,
    #[serde(rename = "breaksSmartSb")]
    breaks_smart_sb: Option<f64>,
    #[serde(rename = "condense")]
    condense: Option<String>,
    #[serde(rename = "condenseFirstPage")]
    condense_first_page: Option<bool>,
    #[serde(rename = "condenseNotLastSystem")]
    condense_not_last_system: Option<bool>,
    #[serde(rename = "condenseTempoPages")]
    condense_tempo_pages: Option<bool>,
    #[serde(rename = "evenNoteSpacing")]
    even_note_spacing: Option<bool>,
    #[serde(rename = "expand")]
    expand: Option<String>,
    #[serde(rename = "footer")]
    footer: Option<String>,
    #[serde(rename = "header")]
    header: Option<String>,
    #[serde(rename = "humType")]
    hum_type: Option<bool>,
    #[serde(rename = "incip")]
    incip: Option<bool>,
    #[serde(rename = "justifyVertically")]
    justify_vertically: Option<bool>,
    #[serde(rename = "landscape")]
    landscape: Option<bool>,
    #[serde(rename = "minLastJustification")]
    min_last_justification: Option<f64>,
    #[serde(rename = "mmOutput")]
    mm_output: Option<bool>,
    #[serde(rename = "moveScoreDefinitionToStaff")]
    move_score_definition_to_staff: Option<bool>,
    #[serde(rename = "neumeAsNote")]
    neume_as_note: Option<bool>,
    #[serde(rename = "noJustification")]
    no_justification: Option<bool>,
    #[serde(rename = "openControlEvents")]
    open_control_events: Option<bool>,
    #[serde(rename = "outputFormatRaw")]
    output_format_raw: Option<bool>,
    #[serde(rename = "outputIndent")]
    output_indent: Option<i32>,
    #[serde(rename = "outputIndentTab")]
    output_indent_tab: Option<bool>,
    #[serde(rename = "outputSmuflXmlEntities")]
    output_smufl_xml_entities: Option<bool>,
    #[serde(rename = "pageHeight")]
    page_height: Option<i32>,
    #[serde(rename = "pageMarginBottom")]
    page_margin_bottom: Option<i32>,
    #[serde(rename = "pageMarginLeft")]
    page_margin_left: Option<i32>,
    #[serde(rename = "pageMarginRight")]
    page_margin_right: Option<i32>,
    #[serde(rename = "pageMarginTop")]
    page_margin_top: Option<i32>,
    #[serde(rename = "pageWidth")]
    page_width: Option<i32>,
    #[serde(rename = "pedalStyle")]
    pedal_style: Option<String>,
    #[serde(rename = "preserveAnalyticalMarkup")]
    preserve_analytical_markup: Option<bool>,
    #[serde(rename = "removeIds")]
    remove_ids: Option<bool>,
    #[serde(rename = "scaleToPageSize")]
    scale_to_page_size: Option<bool>,
    #[serde(rename = "setLocale")]
    set_locale: Option<bool>,
    #[serde(rename = "showRuntime")]
    show_runtime: Option<bool>,
    #[serde(rename = "shrinkToFit")]
    shrink_to_fit: Option<bool>,
    #[serde(rename = "smuflTextFont")]
    smufl_text_font: Option<String>,
    #[serde(rename = "staccatoCenter")]
    staccato_center: Option<bool>,
    #[serde(rename = "svgAdditionalAttribute")]
    svg_additional_attribute: Option<Vec<String>>,
    #[serde(rename = "svgBoundingBoxes")]
    svg_bounding_boxes: Option<bool>,
    #[serde(rename = "svgCss")]
    svg_css: Option<String>,
    #[serde(rename = "svgFormatRaw")]
    svg_format_raw: Option<bool>,
    #[serde(rename = "svgHtml5")]
    svg_html5: Option<bool>,
    #[serde(rename = "svgRemoveXlink")]
    svg_remove_xlink: Option<bool>,
    #[serde(rename = "svgViewBox")]
    svg_view_box: Option<bool>,
    #[serde(rename = "unit")]
    unit: Option<f64>,
    #[serde(rename = "useBraceGlyph")]
    use_brace_glyph: Option<bool>,
    #[serde(rename = "useFacsimile")]
    use_facsimile: Option<bool>,
    #[serde(rename = "usePgFooterForAll")]
    use_pg_footer_for_all: Option<bool>,
    #[serde(rename = "usePgHeaderForAll")]
    use_pg_header_for_all: Option<bool>,
    #[serde(rename = "xmlIdChecksum")]
    xml_id_checksum: Option<bool>,
    #[serde(rename = "barLineSeparation")]
    bar_line_separation: Option<f64>,
    #[serde(rename = "barLineWidth")]
    bar_line_width: Option<f64>,
    #[serde(rename = "beamFrenchStyle")]
    beam_french_style: Option<bool>,
    #[serde(rename = "beamMaxSlope")]
    beam_max_slope: Option<i32>,
    #[serde(rename = "beamMixedPreserve")]
    beam_mixed_preserve: Option<bool>,
    #[serde(rename = "beamMixedStemMin")]
    beam_mixed_stem_min: Option<f64>,
    #[serde(rename = "bracketThickness")]
    bracket_thickness: Option<f64>,
    #[serde(rename = "breaksNoWidow")]
    breaks_no_widow: Option<bool>,
    #[serde(rename = "dashedBarLineDashLength")]
    dashed_bar_line_dash_length: Option<f64>,
    #[serde(rename = "dashedBarLineGapLength")]
    dashed_bar_line_gap_length: Option<f64>,
    #[serde(rename = "dynamDist")]
    dynam_dist: Option<f64>,
    #[serde(rename = "dynamSingleGlyphs")]
    dynam_single_glyphs: Option<bool>,
    #[serde(rename = "extenderLineMinSpace")]
    extender_line_min_space: Option<f64>,
    #[serde(rename = "fingeringScale")]
    fingering_scale: Option<f64>,
    #[serde(rename = "font")]
    font: Option<String>,
    #[serde(rename = "fontAddCustom")]
    font_add_custom: Option<Vec<String>>,
    #[serde(rename = "fontFallback")]
    font_fallback: Option<String>,
    #[serde(rename = "fontLoadAll")]
    font_load_all: Option<bool>,
    #[serde(rename = "graceFactor")]
    grace_factor: Option<f64>,
    #[serde(rename = "graceRhythmAlign")]
    grace_rhythm_align: Option<bool>,
    #[serde(rename = "graceRightAlign")]
    grace_right_align: Option<bool>,
    #[serde(rename = "hairpinSize")]
    hairpin_size: Option<f64>,
    #[serde(rename = "hairpinThickness")]
    hairpin_thickness: Option<f64>,
    #[serde(rename = "handwrittenFont")]
    handwritten_font: Option<Vec<String>>,
    #[serde(rename = "harmDist")]
    harm_dist: Option<f64>,
    #[serde(rename = "justificationBraceGroup")]
    justification_brace_group: Option<f64>,
    #[serde(rename = "justificationBracketGroup")]
    justification_bracket_group: Option<f64>,
    #[serde(rename = "justificationMaxVertical")]
    justification_max_vertical: Option<f64>,
    #[serde(rename = "justificationStaff")]
    justification_staff: Option<f64>,
    #[serde(rename = "justificationSystem")]
    justification_system: Option<f64>,
    #[serde(rename = "ledgerLineExtension")]
    ledger_line_extension: Option<f64>,
    #[serde(rename = "ledgerLineThickness")]
    ledger_line_thickness: Option<f64>,
    #[serde(rename = "lyricElision")]
    lyric_elision: Option<String>,
    #[serde(rename = "lyricHeightFactor")]
    lyric_height_factor: Option<f64>,
    #[serde(rename = "lyricLineThickness")]
    lyric_line_thickness: Option<f64>,
    #[serde(rename = "lyricNoStartHyphen")]
    lyric_no_start_hyphen: Option<bool>,
    #[serde(rename = "lyricSize")]
    lyric_size: Option<f64>,
    #[serde(rename = "lyricTopMinMargin")]
    lyric_top_min_margin: Option<f64>,
    #[serde(rename = "lyricVerseCollapse")]
    lyric_verse_collapse: Option<bool>,
    #[serde(rename = "lyricWordSpace")]
    lyric_word_space: Option<f64>,
    #[serde(rename = "measureMinWidth")]
    measure_min_width: Option<i32>,
    #[serde(rename = "mnumInterval")]
    mnum_interval: Option<i32>,
    #[serde(rename = "multiRestStyle")]
    multi_rest_style: Option<String>,
    #[serde(rename = "multiRestThickness")]
    multi_rest_thickness: Option<f64>,
    #[serde(rename = "octaveAlternativeSymbols")]
    octave_alternative_symbols: Option<bool>,
    #[serde(rename = "octaveLineThickness")]
    octave_line_thickness: Option<f64>,
    #[serde(rename = "octaveNoSpanningParentheses")]
    octave_no_spanning_parentheses: Option<bool>,
    #[serde(rename = "pedalLineThickness")]
    pedal_line_thickness: Option<f64>,
    #[serde(rename = "repeatBarLineDotSeparation")]
    repeat_bar_line_dot_separation: Option<f64>,
    #[serde(rename = "repeatEndingLineThickness")]
    repeat_ending_line_thickness: Option<f64>,
    #[serde(rename = "slurCurveFactor")]
    slur_curve_factor: Option<f64>,
    #[serde(rename = "slurEndpointFlexibility")]
    slur_endpoint_flexibility: Option<f64>,
    #[serde(rename = "slurEndpointThickness")]
    slur_endpoint_thickness: Option<f64>,
    #[serde(rename = "slurMargin")]
    slur_margin: Option<f64>,
    #[serde(rename = "slurMaxSlope")]
    slur_max_slope: Option<i32>,
    #[serde(rename = "slurMidpointThickness")]
    slur_midpoint_thickness: Option<f64>,
    #[serde(rename = "slurSymmetry")]
    slur_symmetry: Option<f64>,
    #[serde(rename = "spacingBraceGroup")]
    spacing_brace_group: Option<i32>,
    #[serde(rename = "spacingBracketGroup")]
    spacing_bracket_group: Option<i32>,
    #[serde(rename = "spacingDurDetection")]
    spacing_dur_detection: Option<bool>,
    #[serde(rename = "spacingLinear")]
    spacing_linear: Option<f64>,
    #[serde(rename = "spacingNonLinear")]
    spacing_non_linear: Option<f64>,
    #[serde(rename = "spacingStaff")]
    spacing_staff: Option<i32>,
    #[serde(rename = "spacingSystem")]
    spacing_system: Option<i32>,
    #[serde(rename = "staffLineWidth")]
    staff_line_width: Option<f64>,
    #[serde(rename = "stemWidth")]
    stem_width: Option<f64>,
    #[serde(rename = "subBracketThickness")]
    sub_bracket_thickness: Option<f64>,
    #[serde(rename = "systemDivider")]
    system_divider: Option<String>,
    #[serde(rename = "systemMaxPerPage")]
    system_max_per_page: Option<i32>,
    #[serde(rename = "textEnclosureThickness")]
    text_enclosure_thickness: Option<f64>,
    #[serde(rename = "thickBarlineThickness")]
    thick_barline_thickness: Option<f64>,
    #[serde(rename = "tieEndpointThickness")]
    tie_endpoint_thickness: Option<f64>,
    #[serde(rename = "tieMidpointThickness")]
    tie_midpoint_thickness: Option<f64>,
    #[serde(rename = "tieMinLength")]
    tie_min_length: Option<f64>,
    #[serde(rename = "tupletAngledOnBeams")]
    tuplet_angled_on_beams: Option<bool>,
    #[serde(rename = "tupletBracketThickness")]
    tuplet_bracket_thickness: Option<f64>,
    #[serde(rename = "tupletNumHead")]
    tuplet_num_head: Option<bool>,
    #[serde(rename = "appXPathQuery")]
    app_xpath_query: Option<Vec<String>>,
    #[serde(rename = "choiceXPathQuery")]
    choice_xpath_query: Option<Vec<String>>,
    #[serde(rename = "loadSelectedMdivOnly")]
    load_selected_mdiv_only: Option<bool>,
    #[serde(rename = "mdivAll")]
    mdiv_all: Option<bool>,
    #[serde(rename = "mdivXPathQuery")]
    mdiv_xpath_query: Option<String>,
    #[serde(rename = "substXPathQuery")]
    subst_xpath_query: Option<Vec<String>>,
    #[serde(rename = "transpose")]
    transpose: Option<String>,
    #[serde(rename = "transposeMdiv")]
    transpose_mdiv: Option<String>,
    #[serde(rename = "transposeSelectedOnly")]
    transpose_selected_only: Option<bool>,
    #[serde(rename = "transposeToSoundingPitch")]
    transpose_to_sounding_pitch: Option<bool>,
    #[serde(rename = "bottomMarginArtic")]
    bottom_margin_artic: Option<f64>,
    #[serde(rename = "bottomMarginHarm")]
    bottom_margin_harm: Option<f64>,
    #[serde(rename = "bottomMarginHeader")]
    bottom_margin_header: Option<f64>,
    #[serde(rename = "bottomMarginOctave")]
    bottom_margin_octave: Option<f64>,
    #[serde(rename = "defaultBottomMargin")]
    default_bottom_margin: Option<f64>,
    #[serde(rename = "defaultLeftMargin")]
    default_left_margin: Option<f64>,
    #[serde(rename = "defaultRightMargin")]
    default_right_margin: Option<f64>,
    #[serde(rename = "defaultTopMargin")]
    default_top_margin: Option<f64>,
    #[serde(rename = "leftMarginAccid")]
    left_margin_accid: Option<f64>,
    #[serde(rename = "leftMarginBarLine")]
    left_margin_bar_line: Option<f64>,
    #[serde(rename = "leftMarginBeatRpt")]
    left_margin_beat_rpt: Option<f64>,
    #[serde(rename = "leftMarginChord")]
    left_margin_chord: Option<f64>,
    #[serde(rename = "leftMarginClef")]
    left_margin_clef: Option<f64>,
    #[serde(rename = "leftMarginKeySig")]
    left_margin_key_sig: Option<f64>,
    #[serde(rename = "leftMarginLeftBarLine")]
    left_margin_left_bar_line: Option<f64>,
    #[serde(rename = "leftMarginMRest")]
    left_margin_m_rest: Option<f64>,
    #[serde(rename = "leftMarginMRpt2")]
    left_margin_m_rpt2: Option<f64>,
    #[serde(rename = "leftMarginMensur")]
    left_margin_mensur: Option<f64>,
    #[serde(rename = "leftMarginMeterSig")]
    left_margin_meter_sig: Option<f64>,
}
