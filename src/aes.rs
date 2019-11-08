#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 1st-4th word of key"]
    pub key: [KEY; 4],
    #[doc = "0x10 - Encryption or decryption select"]
    pub encrypt_sel: ENCRYPT_SEL,
    #[doc = "0x14 - AES mode register"]
    pub mode_ctl: MODE_CTL,
    #[doc = "0x18 - Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
    pub iv: [IV; 4],
    #[doc = "0x28 - Endian control"]
    pub endian: ENDIAN,
    #[doc = "0x2c - Finished status"]
    pub finish: FINISH,
    #[doc = "0x30 - DMA select"]
    pub dma_sel: DMA_SEL,
    #[doc = "0x34 - GCM additional authenticated data count in bytes, minus one"]
    pub aad_num: AAD_NUM,
    _reserved8: [u8; 4usize],
    #[doc = "0x3c - Plaintext/ciphertext input data count in bytes, minus one"]
    pub pc_num: PC_NUM,
    #[doc = "0x40 - Plaintext/ciphertext input data"]
    pub text_data: TEXT_DATA,
    #[doc = "0x44 - Additional authenticated data"]
    pub aad_data: AAD_DATA,
    #[doc = "0x48 - Tag check status"]
    pub tag_chk: TAG_CHK,
    #[doc = "0x4c - Data can input flag"]
    pub data_in_flag: DATA_IN_FLAG,
    #[doc = "0x50 - GCM input tag for comparison with the calculated tag"]
    pub gcm_in_tag: [GCM_IN_TAG; 4],
    #[doc = "0x60 - Plaintext/ciphertext output data"]
    pub out_data: OUT_DATA,
    #[doc = "0x64 - AES module enable"]
    pub en: EN,
    #[doc = "0x68 - Data can output flag"]
    pub data_out_flag: DATA_OUT_FLAG,
    #[doc = "0x6c - Can input tag (when using GCM)"]
    pub tag_in_flag: TAG_IN_FLAG,
    #[doc = "0x70 - Tag clear (a write to this register clears the tag_chk status)"]
    pub tag_clear: TAG_CLEAR,
    #[doc = "0x74 - Computed GCM output tag"]
    pub gcm_out_tag: [GCM_OUT_TAG; 4],
    #[doc = "0x84 - 5th-8th word of key"]
    pub key_ext: [KEY_EXT; 4],
}
#[doc = "1st-4th word of key\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key](key) module"]
pub type KEY = crate::Reg<u32, _KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY;
#[doc = "`read()` method returns [key::R](key::R) reader structure"]
impl crate::Readable for KEY {}
#[doc = "`write(|w| ..)` method takes [key::W](key::W) writer structure"]
impl crate::Writable for KEY {}
#[doc = "1st-4th word of key"]
pub mod key;
#[doc = "Encryption or decryption select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [encrypt_sel](encrypt_sel) module"]
pub type ENCRYPT_SEL = crate::Reg<u32, _ENCRYPT_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENCRYPT_SEL;
#[doc = "`read()` method returns [encrypt_sel::R](encrypt_sel::R) reader structure"]
impl crate::Readable for ENCRYPT_SEL {}
#[doc = "`write(|w| ..)` method takes [encrypt_sel::W](encrypt_sel::W) writer structure"]
impl crate::Writable for ENCRYPT_SEL {}
#[doc = "Encryption or decryption select"]
pub mod encrypt_sel;
#[doc = "AES mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode_ctl](mode_ctl) module"]
pub type MODE_CTL = crate::Reg<u32, _MODE_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE_CTL;
#[doc = "`read()` method returns [mode_ctl::R](mode_ctl::R) reader structure"]
impl crate::Readable for MODE_CTL {}
#[doc = "`write(|w| ..)` method takes [mode_ctl::W](mode_ctl::W) writer structure"]
impl crate::Writable for MODE_CTL {}
#[doc = "AES mode register"]
pub mod mode_ctl;
#[doc = "Initialisation Vector (96 bit for GCM, 128 bit for CBC)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iv](iv) module"]
pub type IV = crate::Reg<u32, _IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV;
#[doc = "`read()` method returns [iv::R](iv::R) reader structure"]
impl crate::Readable for IV {}
#[doc = "`write(|w| ..)` method takes [iv::W](iv::W) writer structure"]
impl crate::Writable for IV {}
#[doc = "Initialisation Vector (96 bit for GCM, 128 bit for CBC)"]
pub mod iv;
#[doc = "Endian control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [endian](endian) module"]
pub type ENDIAN = crate::Reg<u32, _ENDIAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDIAN;
#[doc = "`read()` method returns [endian::R](endian::R) reader structure"]
impl crate::Readable for ENDIAN {}
#[doc = "`write(|w| ..)` method takes [endian::W](endian::W) writer structure"]
impl crate::Writable for ENDIAN {}
#[doc = "Endian control"]
pub mod endian;
#[doc = "Finished status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [finish](finish) module"]
pub type FINISH = crate::Reg<u32, _FINISH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FINISH;
#[doc = "`read()` method returns [finish::R](finish::R) reader structure"]
impl crate::Readable for FINISH {}
#[doc = "`write(|w| ..)` method takes [finish::W](finish::W) writer structure"]
impl crate::Writable for FINISH {}
#[doc = "Finished status"]
pub mod finish;
#[doc = "DMA select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_sel](dma_sel) module"]
pub type DMA_SEL = crate::Reg<u32, _DMA_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SEL;
#[doc = "`read()` method returns [dma_sel::R](dma_sel::R) reader structure"]
impl crate::Readable for DMA_SEL {}
#[doc = "`write(|w| ..)` method takes [dma_sel::W](dma_sel::W) writer structure"]
impl crate::Writable for DMA_SEL {}
#[doc = "DMA select"]
pub mod dma_sel;
#[doc = "GCM additional authenticated data count in bytes, minus one\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aad_num](aad_num) module"]
pub type AAD_NUM = crate::Reg<u32, _AAD_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AAD_NUM;
#[doc = "`read()` method returns [aad_num::R](aad_num::R) reader structure"]
impl crate::Readable for AAD_NUM {}
#[doc = "`write(|w| ..)` method takes [aad_num::W](aad_num::W) writer structure"]
impl crate::Writable for AAD_NUM {}
#[doc = "GCM additional authenticated data count in bytes, minus one"]
pub mod aad_num;
#[doc = "Plaintext/ciphertext input data count in bytes, minus one\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc_num](pc_num) module"]
pub type PC_NUM = crate::Reg<u32, _PC_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_NUM;
#[doc = "`read()` method returns [pc_num::R](pc_num::R) reader structure"]
impl crate::Readable for PC_NUM {}
#[doc = "`write(|w| ..)` method takes [pc_num::W](pc_num::W) writer structure"]
impl crate::Writable for PC_NUM {}
#[doc = "Plaintext/ciphertext input data count in bytes, minus one"]
pub mod pc_num;
#[doc = "Plaintext/ciphertext input data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [text_data](text_data) module"]
pub type TEXT_DATA = crate::Reg<u32, _TEXT_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEXT_DATA;
#[doc = "`read()` method returns [text_data::R](text_data::R) reader structure"]
impl crate::Readable for TEXT_DATA {}
#[doc = "`write(|w| ..)` method takes [text_data::W](text_data::W) writer structure"]
impl crate::Writable for TEXT_DATA {}
#[doc = "Plaintext/ciphertext input data"]
pub mod text_data;
#[doc = "Additional authenticated data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aad_data](aad_data) module"]
pub type AAD_DATA = crate::Reg<u32, _AAD_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AAD_DATA;
#[doc = "`read()` method returns [aad_data::R](aad_data::R) reader structure"]
impl crate::Readable for AAD_DATA {}
#[doc = "`write(|w| ..)` method takes [aad_data::W](aad_data::W) writer structure"]
impl crate::Writable for AAD_DATA {}
#[doc = "Additional authenticated data"]
pub mod aad_data;
#[doc = "Tag check status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tag_chk](tag_chk) module"]
pub type TAG_CHK = crate::Reg<u32, _TAG_CHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_CHK;
#[doc = "`read()` method returns [tag_chk::R](tag_chk::R) reader structure"]
impl crate::Readable for TAG_CHK {}
#[doc = "`write(|w| ..)` method takes [tag_chk::W](tag_chk::W) writer structure"]
impl crate::Writable for TAG_CHK {}
#[doc = "Tag check status"]
pub mod tag_chk;
#[doc = "Data can input flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_in_flag](data_in_flag) module"]
pub type DATA_IN_FLAG = crate::Reg<u32, _DATA_IN_FLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN_FLAG;
#[doc = "`read()` method returns [data_in_flag::R](data_in_flag::R) reader structure"]
impl crate::Readable for DATA_IN_FLAG {}
#[doc = "`write(|w| ..)` method takes [data_in_flag::W](data_in_flag::W) writer structure"]
impl crate::Writable for DATA_IN_FLAG {}
#[doc = "Data can input flag"]
pub mod data_in_flag;
#[doc = "GCM input tag for comparison with the calculated tag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gcm_in_tag](gcm_in_tag) module"]
pub type GCM_IN_TAG = crate::Reg<u32, _GCM_IN_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCM_IN_TAG;
#[doc = "`read()` method returns [gcm_in_tag::R](gcm_in_tag::R) reader structure"]
impl crate::Readable for GCM_IN_TAG {}
#[doc = "`write(|w| ..)` method takes [gcm_in_tag::W](gcm_in_tag::W) writer structure"]
impl crate::Writable for GCM_IN_TAG {}
#[doc = "GCM input tag for comparison with the calculated tag"]
pub mod gcm_in_tag;
#[doc = "Plaintext/ciphertext output data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_data](out_data) module"]
pub type OUT_DATA = crate::Reg<u32, _OUT_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_DATA;
#[doc = "`read()` method returns [out_data::R](out_data::R) reader structure"]
impl crate::Readable for OUT_DATA {}
#[doc = "`write(|w| ..)` method takes [out_data::W](out_data::W) writer structure"]
impl crate::Writable for OUT_DATA {}
#[doc = "Plaintext/ciphertext output data"]
pub mod out_data;
#[doc = "AES module enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`read()` method returns [en::R](en::R) reader structure"]
impl crate::Readable for EN {}
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "AES module enable"]
pub mod en;
#[doc = "Data can output flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_out_flag](data_out_flag) module"]
pub type DATA_OUT_FLAG = crate::Reg<u32, _DATA_OUT_FLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_OUT_FLAG;
#[doc = "`read()` method returns [data_out_flag::R](data_out_flag::R) reader structure"]
impl crate::Readable for DATA_OUT_FLAG {}
#[doc = "`write(|w| ..)` method takes [data_out_flag::W](data_out_flag::W) writer structure"]
impl crate::Writable for DATA_OUT_FLAG {}
#[doc = "Data can output flag"]
pub mod data_out_flag;
#[doc = "Can input tag (when using GCM)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tag_in_flag](tag_in_flag) module"]
pub type TAG_IN_FLAG = crate::Reg<u32, _TAG_IN_FLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_IN_FLAG;
#[doc = "`read()` method returns [tag_in_flag::R](tag_in_flag::R) reader structure"]
impl crate::Readable for TAG_IN_FLAG {}
#[doc = "`write(|w| ..)` method takes [tag_in_flag::W](tag_in_flag::W) writer structure"]
impl crate::Writable for TAG_IN_FLAG {}
#[doc = "Can input tag (when using GCM)"]
pub mod tag_in_flag;
#[doc = "Tag clear (a write to this register clears the tag_chk status)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tag_clear](tag_clear) module"]
pub type TAG_CLEAR = crate::Reg<u32, _TAG_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAG_CLEAR;
#[doc = "`read()` method returns [tag_clear::R](tag_clear::R) reader structure"]
impl crate::Readable for TAG_CLEAR {}
#[doc = "`write(|w| ..)` method takes [tag_clear::W](tag_clear::W) writer structure"]
impl crate::Writable for TAG_CLEAR {}
#[doc = "Tag clear (a write to this register clears the tag_chk status)"]
pub mod tag_clear;
#[doc = "Computed GCM output tag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gcm_out_tag](gcm_out_tag) module"]
pub type GCM_OUT_TAG = crate::Reg<u32, _GCM_OUT_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCM_OUT_TAG;
#[doc = "`read()` method returns [gcm_out_tag::R](gcm_out_tag::R) reader structure"]
impl crate::Readable for GCM_OUT_TAG {}
#[doc = "`write(|w| ..)` method takes [gcm_out_tag::W](gcm_out_tag::W) writer structure"]
impl crate::Writable for GCM_OUT_TAG {}
#[doc = "Computed GCM output tag"]
pub mod gcm_out_tag;
#[doc = "5th-8th word of key\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key_ext](key_ext) module"]
pub type KEY_EXT = crate::Reg<u32, _KEY_EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY_EXT;
#[doc = "`read()` method returns [key_ext::R](key_ext::R) reader structure"]
impl crate::Readable for KEY_EXT {}
#[doc = "`write(|w| ..)` method takes [key_ext::W](key_ext::W) writer structure"]
impl crate::Writable for KEY_EXT {}
#[doc = "5th-8th word of key"]
pub mod key_ext;
