#![no_main]
#![no_std]

use defmt::info;
// Needed to defmt rtt channel
use defmt_rtt as _;
use embassy_executor::Spawner;
// Sets the panic handler
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = embassy_stm32::Config::default();
    let _p = embassy_stm32::init(config);

    info!("Oh hi!");
}
