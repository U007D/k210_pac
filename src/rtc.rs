#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer date information"]
    pub date: DATE,
    #[doc = "0x04 - Timer time information"]
    pub time: TIME,
    #[doc = "0x08 - Alarm date information"]
    pub alarm_date: ALARM_DATE,
    #[doc = "0x0c - Alarm time information"]
    pub alarm_time: ALARM_TIME,
    #[doc = "0x10 - Timer counter initial value"]
    pub initial_count: INITIAL_COUNT,
    #[doc = "0x14 - Timer counter current value"]
    pub current_count: CURRENT_COUNT,
    #[doc = "0x18 - RTC interrupt settings"]
    pub interrupt_ctrl: INTERRUPT_CTRL,
    #[doc = "0x1c - RTC register settings"]
    pub register_ctrl: REGISTER_CTRL,
    _reserved8: [u8; 8usize],
    #[doc = "0x28 - Timer extended information"]
    pub extended: EXTENDED,
}
#[doc = "Timer date information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "Timer date information"]
pub mod date;
#[doc = "Timer time information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [time](time) module"]
pub type TIME = crate::Reg<u32, _TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME;
#[doc = "`read()` method returns [time::R](time::R) reader structure"]
impl crate::Readable for TIME {}
#[doc = "`write(|w| ..)` method takes [time::W](time::W) writer structure"]
impl crate::Writable for TIME {}
#[doc = "Timer time information"]
pub mod time;
#[doc = "Alarm date information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [alarm_date](alarm_date) module"]
pub type ALARM_DATE = crate::Reg<u32, _ALARM_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARM_DATE;
#[doc = "`read()` method returns [alarm_date::R](alarm_date::R) reader structure"]
impl crate::Readable for ALARM_DATE {}
#[doc = "`write(|w| ..)` method takes [alarm_date::W](alarm_date::W) writer structure"]
impl crate::Writable for ALARM_DATE {}
#[doc = "Alarm date information"]
pub mod alarm_date;
#[doc = "Alarm time information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [alarm_time](alarm_time) module"]
pub type ALARM_TIME = crate::Reg<u32, _ALARM_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARM_TIME;
#[doc = "`read()` method returns [alarm_time::R](alarm_time::R) reader structure"]
impl crate::Readable for ALARM_TIME {}
#[doc = "`write(|w| ..)` method takes [alarm_time::W](alarm_time::W) writer structure"]
impl crate::Writable for ALARM_TIME {}
#[doc = "Alarm time information"]
pub mod alarm_time;
#[doc = "Timer counter initial value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [initial_count](initial_count) module"]
pub type INITIAL_COUNT = crate::Reg<u32, _INITIAL_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INITIAL_COUNT;
#[doc = "`read()` method returns [initial_count::R](initial_count::R) reader structure"]
impl crate::Readable for INITIAL_COUNT {}
#[doc = "`write(|w| ..)` method takes [initial_count::W](initial_count::W) writer structure"]
impl crate::Writable for INITIAL_COUNT {}
#[doc = "Timer counter initial value"]
pub mod initial_count;
#[doc = "Timer counter current value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [current_count](current_count) module"]
pub type CURRENT_COUNT = crate::Reg<u32, _CURRENT_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURRENT_COUNT;
#[doc = "`read()` method returns [current_count::R](current_count::R) reader structure"]
impl crate::Readable for CURRENT_COUNT {}
#[doc = "`write(|w| ..)` method takes [current_count::W](current_count::W) writer structure"]
impl crate::Writable for CURRENT_COUNT {}
#[doc = "Timer counter current value"]
pub mod current_count;
#[doc = "RTC interrupt settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_ctrl](interrupt_ctrl) module"]
pub type INTERRUPT_CTRL = crate::Reg<u32, _INTERRUPT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CTRL;
#[doc = "`read()` method returns [interrupt_ctrl::R](interrupt_ctrl::R) reader structure"]
impl crate::Readable for INTERRUPT_CTRL {}
#[doc = "`write(|w| ..)` method takes [interrupt_ctrl::W](interrupt_ctrl::W) writer structure"]
impl crate::Writable for INTERRUPT_CTRL {}
#[doc = "RTC interrupt settings"]
pub mod interrupt_ctrl;
#[doc = "RTC register settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [register_ctrl](register_ctrl) module"]
pub type REGISTER_CTRL = crate::Reg<u32, _REGISTER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGISTER_CTRL;
#[doc = "`read()` method returns [register_ctrl::R](register_ctrl::R) reader structure"]
impl crate::Readable for REGISTER_CTRL {}
#[doc = "`write(|w| ..)` method takes [register_ctrl::W](register_ctrl::W) writer structure"]
impl crate::Writable for REGISTER_CTRL {}
#[doc = "RTC register settings"]
pub mod register_ctrl;
#[doc = "Timer extended information\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [extended](extended) module"]
pub type EXTENDED = crate::Reg<u32, _EXTENDED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTENDED;
#[doc = "`read()` method returns [extended::R](extended::R) reader structure"]
impl crate::Readable for EXTENDED {}
#[doc = "`write(|w| ..)` method takes [extended::W](extended::W) writer structure"]
impl crate::Writable for EXTENDED {}
#[doc = "Timer extended information"]
pub mod extended;
