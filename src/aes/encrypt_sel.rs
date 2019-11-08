#[doc = "Reader of register encrypt_sel"]
pub type R = crate::R<u32, super::ENCRYPT_SEL>;
#[doc = "Writer for register encrypt_sel"]
pub type W = crate::W<u32, super::ENCRYPT_SEL>;
#[doc = "Register encrypt_sel `reset()`'s with value 0"]
impl crate::ResetValue for super::ENCRYPT_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select encryption or decryption mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCRYPT_SEL_A {
    #[doc = "0: Sets encryption mode"]
    ENCRYPTION,
    #[doc = "1: Sets decryption mode"]
    DECRYPTION,
}
impl From<ENCRYPT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ENCRYPT_SEL_A) -> Self {
        match variant {
            ENCRYPT_SEL_A::ENCRYPTION => false,
            ENCRYPT_SEL_A::DECRYPTION => true,
        }
    }
}
#[doc = "Reader of field `encrypt_sel`"]
pub type ENCRYPT_SEL_R = crate::R<bool, ENCRYPT_SEL_A>;
impl ENCRYPT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCRYPT_SEL_A {
        match self.bits {
            false => ENCRYPT_SEL_A::ENCRYPTION,
            true => ENCRYPT_SEL_A::DECRYPTION,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPTION`"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == ENCRYPT_SEL_A::ENCRYPTION
    }
    #[doc = "Checks if the value of the field is `DECRYPTION`"]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == ENCRYPT_SEL_A::DECRYPTION
    }
}
#[doc = "Write proxy for field `encrypt_sel`"]
pub struct ENCRYPT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCRYPT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENCRYPT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sets encryption mode"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut W {
        self.variant(ENCRYPT_SEL_A::ENCRYPTION)
    }
    #[doc = "Sets decryption mode"]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut W {
        self.variant(ENCRYPT_SEL_A::DECRYPTION)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select encryption or decryption mode"]
    #[inline(always)]
    pub fn encrypt_sel(&self) -> ENCRYPT_SEL_R {
        ENCRYPT_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select encryption or decryption mode"]
    #[inline(always)]
    pub fn encrypt_sel(&mut self) -> ENCRYPT_SEL_W {
        ENCRYPT_SEL_W { w: self }
    }
}
