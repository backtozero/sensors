extern crate i2cdev_bmp280;
extern crate i2cdev_lsm9ds0;
extern crate i2csensors;

use i2cdev_bmp280::*;
use i2cdev_lsm9ds0::*;
use i2csensors::{Accelerometer, Barometer, Gyroscope, Magnetometer, Thermometer};
use std::process;

fn main() {
    let i2c_device = i2cdev_bmp280::get_linux_bmp280_i2c_device_with_addr(0x76).unwrap();

    let settings = BMP280Settings
        {
            compensation: BMP280CompensationAlgorithm::B64,
            t_sb: BMP280Timing::ms0_5,
            iir_filter_coeff: BMP280FilterCoefficient::Medium,
            osrs_t: BMP280TemperatureOversampling::x1,
            osrs_p: BMP280PressureOversampling::StandardResolution,
            power_mode: BMP280PowerMode::NormalMode,
        };

    let bmp280 = BMP280::new(i2c_device, settings);

    match bmp280 {
        Ok(mut device) => {
            let temperature = device.temperature_celsius().unwrap();
            let pressure = device.pressure_kpa().unwrap();

            println!("Temperature: {}", temperature);
            println!("Pressure: {}", pressure);
        }
        Err(e) => {
            println!("Error while getting new BMP280 {:#?}", e);
            process::exit(1)
        }
    };


    let (gyro_dev,accel_dev) = get_default_lsm9ds0_linux_i2c_devices_with_addr(0x6B, 0x1E).unwrap();

    let gyro_settings = LSM9DS0GyroscopeSettings {
        DR: LSM9DS0GyroscopeDataRate::Hz190,
        BW: LSM9DS0GyroscopeBandwidth::BW1,
        power_mode: LSM9DS0PowerMode::Normal,
        zen: true,
        yen: true,
        xen: true,
        sensitivity: LSM9DS0GyroscopeFS::dps500,
        continuous_update: true,
        high_pass_filter_enabled: true,
        high_pass_filter_mode: Some(LSM9DS0GyroscopeHighPassFilterMode::NormalMode),
        high_pass_filter_configuration: Some(LSM9DS0HighPassFilterCutOffConfig::HPCF_0)
    };

    let accel_mag_settings = LSM9DS0AccelerometerMagnetometerSettings {
        continuous_update: true,
        accelerometer_data_rate: LSM9DS0AccelerometerUpdateRate::Hz100,
        accelerometer_anti_alias_filter_bandwidth: LSM9DS0AccelerometerFilterBandwidth::Hz50,
        azen: true,
        ayen: true,
        axen: true,
        accelerometer_sensitivity: LSM9DS0AccelerometerFS::g4,
        magnetometer_resolution: LSM9DS0MagnetometerResolution::Low,
        magnetometer_data_rate: LSM9DS0MagnetometerUpdateRate::Hz50,
        magnetometer_low_power_mode: false,
        magnetometer_mode: LSM9DS0MagnetometerMode::ContinuousConversion,
        magnetometer_sensitivity: LSM9DS0MagnetometerFS::gauss2
    };

    let mut lsm9ds0 = LSM9DS0::new(accel_dev, gyro_dev, gyro_settings, accel_mag_settings).unwrap();

    match  lsm9ds0.acceleration_reading() {
        Ok(v) =>  {
            println!("acceleration_reading: X:{}; y{}; z{}", v.x, v.y, v.z);
        },
        Err(e) => println!("acceleration_reading error: {}", e),
    }

    match  lsm9ds0.angular_rate_reading() {
        Ok(v) =>  {
            println!("angular_rate_reading: X:{}; y{}; z{}", v.x, v.y, v.z);
        },
        Err(e) => println!("angular_rate_reading error: {}", e),
    }
}
