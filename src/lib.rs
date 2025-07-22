use anyhow;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
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

    pub fn get_options(&self) -> String {
        get_string(self.tk_ptr, bindings::vrvToolkit_getOptions)
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

    pub fn set_options(&self, options: impl AsRef<str>) -> anyhow::Result<bool> {
        get_primitive2(self.tk_ptr, options, bindings::vrvToolkit_setOptions)
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
