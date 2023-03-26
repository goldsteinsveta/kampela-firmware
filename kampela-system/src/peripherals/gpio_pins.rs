//! Map GPIO pins

use efm32pg23_fix::Peripherals;
use crate::visible_delay;

pub const FLASH_CS_PIN: u8 = 0;
pub const DISP_CS_PIN: u8 = 2;
pub const DISP_DC_PIN: u8 = 3;
pub const DISP_RES_PIN: u8 = 6;
pub const POW_PIN: u8 = 9;
pub const E_MISO_PIN: u8 = 1;
pub const E_MOSI_PIN: u8 = 2;
pub const E_SCK_PIN: u8 = 3;
pub const TOUCH_INT_PIN: u8 = 1;
pub const PSRAM_CS_PIN: u8 = 4; // at portC
pub const PSRAM_MISO_PIN: u8 = 5; // at portC
pub const PSRAM_MOSI_PIN: u8 = 6; // at portC
pub const PSRAM_SCK_PIN: u8 = 7; // at portC
pub const I2C_PIN: u8 = 4;
pub const SCL_PIN: u8 = 3;
pub const SDA_PIN: u8 = 5;
pub const SPI_BUSY_PIN: u8 = 4;
pub const NFC_PIN: u8 = 8; // at portA

/// Macro to switch a specific pin on a specific port.
///
/// At this point GPIO must be already clocked from elsewhere and port must be
/// in correct mode.
///
/// This does not change previously set bits.
macro_rules! gpio_pin {
    ($(#[$attr_set:meta] #[$attr_clear:meta] #[$attr_common:meta] $func_set: ident, $func_clear: ident, $port: tt, $pin: tt), *) => {
        $(
            #[$attr_set]
            #[$attr_common]
            pub fn $func_set(peripherals: &mut Peripherals) {
                peripherals
                    .GPIO_S
                    .$port
                    .modify(|r, w| w.dout().variant(r.dout().bits() | (1 << $pin)));
            }

            #[$attr_clear]
            #[$attr_common]
            pub fn $func_clear(peripherals: &mut Peripherals) {
                peripherals
                    .GPIO_S
                    .$port
                    .modify(|r, w| w.dout().variant(r.dout().bits() ^ (1 << $pin)));
            }
        )*
    }
}

// Prepare GPIO pins

gpio_pin!(
    /// Set flash chip select:
    /// Clear flash chip select:
    /// port C, pin [`FLASH_CS_PIN`].
    flash_chip_select_set,
    flash_chip_select_clear,
    portc_dout,
    FLASH_CS_PIN
);

gpio_pin!(
    /// Set display chip select:
    /// Clear display chip select:
    /// port D, pin [`DISP_CS_PIN`].
    display_chip_select_set,
    display_chip_select_clear,
    portd_dout,
    DISP_CS_PIN
);

gpio_pin!(
    /// Set display data/command:
    /// Clear display data/command:
    /// port D, pin [`DISP_DC_PIN`].
    display_data_command_set,
    display_data_command_clear,
    portd_dout,
    DISP_DC_PIN
);

gpio_pin!(
    /// Set display reset:
    /// Clear display reset:
    /// port A, pin [`DISP_RES_PIN`].
    display_res_set,
    display_res_clear,
    porta_dout,
    DISP_RES_PIN
);

gpio_pin!(
    /// i2c set:
    /// i2c clear:
    /// port A, pin [`I2C_PIN`].
    i2c_set,
    i2c_clear,
    porta_dout,
    I2C_PIN
);

gpio_pin!(
    /// scl set:
    /// scl clear:
    /// port A, pin [`SCL_PIN`].
    scl_set,
    scl_clear,
    porta_dout,
    SCL_PIN
);

gpio_pin!(
    /// sda set:
    /// sda clear:
    /// port A, pin [`SDA_PIN`].
    sda_set,
    sda_clear,
    porta_dout,
    SDA_PIN
);

gpio_pin!(
    /// Set power:
    /// Clear power:
    /// port A, pin [`POW_PIN`].
    pow_set,
    pow_clear,
    porta_dout,
    POW_PIN
);

gpio_pin!(
    /// Set MISO:
    /// Clear MISO:
    /// port A, pin [`E_MISO_PIN`].
    miso_set,
    miso_clear,
    portc_dout,
    E_MISO_PIN
);

gpio_pin!(
    /// Set MOSI:
    /// Clear MOSI:
    /// port A, pin [`E_MOSI_PIN`].
    mosi_set,
    mosi_clear,
    portc_dout,
    E_MOSI_PIN
);

gpio_pin!(
    /// Set SCK:
    /// Clear SCK:
    /// port A, pin [`E_SCK_PIN`].
    sck_set,
    sck_clear,
    portc_dout,
    E_SCK_PIN
);

gpio_pin!(
    /// Set PSRAM CS:
    /// Clear PSRAM CS:
    /// port C, pin [`PSRAM_CS_PIN`].
    psram_chip_select_set,
    psram_chip_select_clear,
    portc_dout,
    PSRAM_CS_PIN
);

gpio_pin!(
    /// Set PSRAM MISO:
    /// Clear PSRAM MISO:
    /// port C, pin [`PSRAM_MISO_PIN`].
    psram_miso_set,
    psram_miso_clear,
    portc_dout,
    PSRAM_MISO_PIN
);

gpio_pin!(
    /// Set PSRAM MOSI:
    /// Clear PSRAM MOSI:
    /// port C, pin [`PSRAM_MOSI_PIN`].
    psram_mosi_set,
    psram_mosi_clear,
    portc_dout,
    PSRAM_MOSI_PIN
);

gpio_pin!(
    /// Set PSRAM SCK:
    /// Clear PSRAM SCK:
    /// port C, pin [`PSRAM_SCK_PIN`].
    psram_sck_set,
    psram_sck_clear,
    portc_dout,
    PSRAM_SCK_PIN
);

gpio_pin!(
    /// Set NFC pin:
    /// Clear NFC pin:
    /// port A, pin [`NFC_PIN`].
    nfc_pin_set,
    nfc_pin_clear,
    porta_dout,
    NFC_PIN
);

/// GPIO initializations
pub fn init_gpio(peripherals: &mut Peripherals) {
    map_gpio(peripherals);
    set_gpio_pins(peripherals);
}

/// Map GPIO pins to their destinations
fn map_gpio(peripherals: &mut Peripherals) {
    peripherals
        .GPIO_S
        .porta_model
        .write(|w_reg| {
            w_reg
                .mode3().wiredandpullup() // SCL for USART (display)
                .mode4().pushpull() // I2C power
                .mode5().wiredandpullup() // SDA for USART (display)
                .mode6().pushpull() // Display reset
    });
    peripherals
        .GPIO_S
        .porta_modeh
        .write(|w_reg| {
            w_reg
                .mode0().inputpullfilter() // NFC
                .mode1().pushpull() // Power 2.8 V
    });
    peripherals
        .GPIO_S
        .portb_model
        .write(|w_reg| {
            w_reg
                .mode1().input() // interrupts from display sensor
                .mode4().input() // BUSY spi
    });
    peripherals
        .GPIO_S
        .portc_model
        .write(|w_reg| {
            w_reg
                .mode0().pushpull() // Flash chip select
                .mode1().inputpull() // Display MISO
                .mode2().pushpull() // Display MOSI
                .mode3().pushpull() //Display SCK
                .mode4().pushpull() // PSRAM chip select
                .mode5().inputpull() // PSRAM MISO
                .mode6().pushpull() // PSRAM MOSI
                .mode7().pushpull() // PSRAM SCK
    });
    peripherals
        .GPIO_S
        .portd_model
        .write(|w_reg| {
            w_reg
                .mode2().inputpull() // Display chip select
                .mode3().pushpull() // Display data/command
    });
}

/// Set GPIO pins to their starting values
fn set_gpio_pins(peripherals: &mut Peripherals) {
    pow_set(peripherals);
    i2c_set(peripherals);
    visible_delay(10); // wait after power set! (epaper manual for 2.8V setup)
    display_chip_select_set(peripherals);
    display_data_command_clear(peripherals);
    display_res_clear(peripherals);
    sda_set(peripherals);
    scl_set(peripherals);
    flash_chip_select_set(peripherals);
    miso_set(peripherals);
    mosi_set(peripherals);
    sck_clear(peripherals);
    psram_chip_select_set(peripherals);
    psram_miso_set(peripherals);
    psram_mosi_clear(peripherals);
    psram_sck_clear(peripherals);
    nfc_pin_clear(peripherals);
}