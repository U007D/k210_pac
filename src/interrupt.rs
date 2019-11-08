#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "1 - SPI0"]
    SPI0,
    #[doc = "2 - SPI1"]
    SPI1,
    #[doc = "3 - SPI_SLAVE"]
    SPI_SLAVE,
    #[doc = "4 - SPI3"]
    SPI3,
    #[doc = "5 - I2S0"]
    I2S0,
    #[doc = "6 - I2S1"]
    I2S1,
    #[doc = "7 - I2S2"]
    I2S2,
    #[doc = "8 - I2C0"]
    I2C0,
    #[doc = "9 - I2C1"]
    I2C1,
    #[doc = "10 - I2C2"]
    I2C2,
    #[doc = "11 - UART1"]
    UART1,
    #[doc = "12 - UART2"]
    UART2,
    #[doc = "13 - UART3"]
    UART3,
    #[doc = "14 - TIMER0 channel 0 or 1 interrupt"]
    TIMER0A,
    #[doc = "15 - TIMER0 channel 2 or 3 interrupt"]
    TIMER0B,
    #[doc = "16 - TIMER1 channel 0 or 1 interrupt"]
    TIMER1A,
    #[doc = "17 - TIMER1 channel 2 or 3 interrupt"]
    TIMER1B,
    #[doc = "18 - TIMER2 channel 0 or 1 interrupt"]
    TIMER2A,
    #[doc = "19 - TIMER2 channel 2 or 3 interrupt"]
    TIMER2B,
    #[doc = "20 - RTC"]
    RTC,
    #[doc = "21 - WDT0"]
    WDT0,
    #[doc = "22 - WDT1"]
    WDT1,
    #[doc = "23 - APB_GPIO"]
    APB_GPIO,
    #[doc = "24 - DVP"]
    DVP,
    #[doc = "25 - KPU"]
    KPU,
    #[doc = "26 - FFT"]
    FFT,
    #[doc = "27 - DMA0"]
    DMA0,
    #[doc = "28 - DMA1"]
    DMA1,
    #[doc = "29 - DMA2"]
    DMA2,
    #[doc = "30 - DMA3"]
    DMA3,
    #[doc = "31 - DMA4"]
    DMA4,
    #[doc = "32 - DMA5"]
    DMA5,
    #[doc = "33 - UARTHS"]
    UARTHS,
    #[doc = "34 - GPIOHS0"]
    GPIOHS0,
    #[doc = "35 - GPIOHS1"]
    GPIOHS1,
    #[doc = "36 - GPIOHS2"]
    GPIOHS2,
    #[doc = "37 - GPIOHS3"]
    GPIOHS3,
    #[doc = "38 - GPIOHS4"]
    GPIOHS4,
    #[doc = "39 - GPIOHS5"]
    GPIOHS5,
    #[doc = "40 - GPIOHS6"]
    GPIOHS6,
    #[doc = "41 - GPIOHS7"]
    GPIOHS7,
    #[doc = "42 - GPIOHS8"]
    GPIOHS8,
    #[doc = "43 - GPIOHS9"]
    GPIOHS9,
    #[doc = "44 - GPIOHS10"]
    GPIOHS10,
    #[doc = "45 - GPIOHS11"]
    GPIOHS11,
    #[doc = "46 - GPIOHS12"]
    GPIOHS12,
    #[doc = "47 - GPIOHS13"]
    GPIOHS13,
    #[doc = "48 - GPIOHS14"]
    GPIOHS14,
    #[doc = "49 - GPIOHS15"]
    GPIOHS15,
    #[doc = "50 - GPIOHS16"]
    GPIOHS16,
    #[doc = "51 - GPIOHS17"]
    GPIOHS17,
    #[doc = "52 - GPIOHS18"]
    GPIOHS18,
    #[doc = "53 - GPIOHS19"]
    GPIOHS19,
    #[doc = "54 - GPIOHS20"]
    GPIOHS20,
    #[doc = "55 - GPIOHS21"]
    GPIOHS21,
    #[doc = "56 - GPIOHS22"]
    GPIOHS22,
    #[doc = "57 - GPIOHS23"]
    GPIOHS23,
    #[doc = "58 - GPIOHS24"]
    GPIOHS24,
    #[doc = "59 - GPIOHS25"]
    GPIOHS25,
    #[doc = "60 - GPIOHS26"]
    GPIOHS26,
    #[doc = "61 - GPIOHS27"]
    GPIOHS27,
    #[doc = "62 - GPIOHS28"]
    GPIOHS28,
    #[doc = "63 - GPIOHS29"]
    GPIOHS29,
    #[doc = "64 - GPIOHS30"]
    GPIOHS30,
    #[doc = "65 - GPIOHS31"]
    GPIOHS31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SPI0 => 1,
            Interrupt::SPI1 => 2,
            Interrupt::SPI_SLAVE => 3,
            Interrupt::SPI3 => 4,
            Interrupt::I2S0 => 5,
            Interrupt::I2S1 => 6,
            Interrupt::I2S2 => 7,
            Interrupt::I2C0 => 8,
            Interrupt::I2C1 => 9,
            Interrupt::I2C2 => 10,
            Interrupt::UART1 => 11,
            Interrupt::UART2 => 12,
            Interrupt::UART3 => 13,
            Interrupt::TIMER0A => 14,
            Interrupt::TIMER0B => 15,
            Interrupt::TIMER1A => 16,
            Interrupt::TIMER1B => 17,
            Interrupt::TIMER2A => 18,
            Interrupt::TIMER2B => 19,
            Interrupt::RTC => 20,
            Interrupt::WDT0 => 21,
            Interrupt::WDT1 => 22,
            Interrupt::APB_GPIO => 23,
            Interrupt::DVP => 24,
            Interrupt::KPU => 25,
            Interrupt::FFT => 26,
            Interrupt::DMA0 => 27,
            Interrupt::DMA1 => 28,
            Interrupt::DMA2 => 29,
            Interrupt::DMA3 => 30,
            Interrupt::DMA4 => 31,
            Interrupt::DMA5 => 32,
            Interrupt::UARTHS => 33,
            Interrupt::GPIOHS0 => 34,
            Interrupt::GPIOHS1 => 35,
            Interrupt::GPIOHS2 => 36,
            Interrupt::GPIOHS3 => 37,
            Interrupt::GPIOHS4 => 38,
            Interrupt::GPIOHS5 => 39,
            Interrupt::GPIOHS6 => 40,
            Interrupt::GPIOHS7 => 41,
            Interrupt::GPIOHS8 => 42,
            Interrupt::GPIOHS9 => 43,
            Interrupt::GPIOHS10 => 44,
            Interrupt::GPIOHS11 => 45,
            Interrupt::GPIOHS12 => 46,
            Interrupt::GPIOHS13 => 47,
            Interrupt::GPIOHS14 => 48,
            Interrupt::GPIOHS15 => 49,
            Interrupt::GPIOHS16 => 50,
            Interrupt::GPIOHS17 => 51,
            Interrupt::GPIOHS18 => 52,
            Interrupt::GPIOHS19 => 53,
            Interrupt::GPIOHS20 => 54,
            Interrupt::GPIOHS21 => 55,
            Interrupt::GPIOHS22 => 56,
            Interrupt::GPIOHS23 => 57,
            Interrupt::GPIOHS24 => 58,
            Interrupt::GPIOHS25 => 59,
            Interrupt::GPIOHS26 => 60,
            Interrupt::GPIOHS27 => 61,
            Interrupt::GPIOHS28 => 62,
            Interrupt::GPIOHS29 => 63,
            Interrupt::GPIOHS30 => 64,
            Interrupt::GPIOHS31 => 65,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::SPI0),
            2 => Ok(Interrupt::SPI1),
            3 => Ok(Interrupt::SPI_SLAVE),
            4 => Ok(Interrupt::SPI3),
            5 => Ok(Interrupt::I2S0),
            6 => Ok(Interrupt::I2S1),
            7 => Ok(Interrupt::I2S2),
            8 => Ok(Interrupt::I2C0),
            9 => Ok(Interrupt::I2C1),
            10 => Ok(Interrupt::I2C2),
            11 => Ok(Interrupt::UART1),
            12 => Ok(Interrupt::UART2),
            13 => Ok(Interrupt::UART3),
            14 => Ok(Interrupt::TIMER0A),
            15 => Ok(Interrupt::TIMER0B),
            16 => Ok(Interrupt::TIMER1A),
            17 => Ok(Interrupt::TIMER1B),
            18 => Ok(Interrupt::TIMER2A),
            19 => Ok(Interrupt::TIMER2B),
            20 => Ok(Interrupt::RTC),
            21 => Ok(Interrupt::WDT0),
            22 => Ok(Interrupt::WDT1),
            23 => Ok(Interrupt::APB_GPIO),
            24 => Ok(Interrupt::DVP),
            25 => Ok(Interrupt::KPU),
            26 => Ok(Interrupt::FFT),
            27 => Ok(Interrupt::DMA0),
            28 => Ok(Interrupt::DMA1),
            29 => Ok(Interrupt::DMA2),
            30 => Ok(Interrupt::DMA3),
            31 => Ok(Interrupt::DMA4),
            32 => Ok(Interrupt::DMA5),
            33 => Ok(Interrupt::UARTHS),
            34 => Ok(Interrupt::GPIOHS0),
            35 => Ok(Interrupt::GPIOHS1),
            36 => Ok(Interrupt::GPIOHS2),
            37 => Ok(Interrupt::GPIOHS3),
            38 => Ok(Interrupt::GPIOHS4),
            39 => Ok(Interrupt::GPIOHS5),
            40 => Ok(Interrupt::GPIOHS6),
            41 => Ok(Interrupt::GPIOHS7),
            42 => Ok(Interrupt::GPIOHS8),
            43 => Ok(Interrupt::GPIOHS9),
            44 => Ok(Interrupt::GPIOHS10),
            45 => Ok(Interrupt::GPIOHS11),
            46 => Ok(Interrupt::GPIOHS12),
            47 => Ok(Interrupt::GPIOHS13),
            48 => Ok(Interrupt::GPIOHS14),
            49 => Ok(Interrupt::GPIOHS15),
            50 => Ok(Interrupt::GPIOHS16),
            51 => Ok(Interrupt::GPIOHS17),
            52 => Ok(Interrupt::GPIOHS18),
            53 => Ok(Interrupt::GPIOHS19),
            54 => Ok(Interrupt::GPIOHS20),
            55 => Ok(Interrupt::GPIOHS21),
            56 => Ok(Interrupt::GPIOHS22),
            57 => Ok(Interrupt::GPIOHS23),
            58 => Ok(Interrupt::GPIOHS24),
            59 => Ok(Interrupt::GPIOHS25),
            60 => Ok(Interrupt::GPIOHS26),
            61 => Ok(Interrupt::GPIOHS27),
            62 => Ok(Interrupt::GPIOHS28),
            63 => Ok(Interrupt::GPIOHS29),
            64 => Ok(Interrupt::GPIOHS30),
            65 => Ok(Interrupt::GPIOHS31),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
