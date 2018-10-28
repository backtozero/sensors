extern crate i2cdev_bmp280;
extern crate i2cdev_l3gd20;
extern crate i2cdev_lsm9ds0;
extern crate i2csensors;

use i2cdev_bmp280::*;
use i2cdev_l3gd20::*;
use i2cdev_lsm9ds0::*;
use i2csensors::{Accelerometer, Barometer, Gyroscope, Magnetometer, Thermometer, Vec3};
use std::{process, thread, time};

fn main() {
    // bmp280
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

    let mut bmp280 = BMP280::new(i2c_device, settings).unwrap();


    // lsm9ds0
    let (gyro_dev, accel_dev) = get_default_lsm9ds0_linux_i2c_devices_with_addr(0x6B, 0x1E).unwrap();

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
        high_pass_filter_configuration: Some(LSM9DS0HighPassFilterCutOffConfig::HPCF_0),
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
        magnetometer_sensitivity: LSM9DS0MagnetometerFS::gauss2,
    };

    let mut lsm9ds0 = LSM9DS0::new(accel_dev, gyro_dev, gyro_settings, accel_mag_settings).unwrap();


    // L3GD20

    let settings = L3GD20GyroscopeSettings {
        DR: L3GD20GyroscopeDataRate::Hz190,
        BW: L3GD20GyroscopeBandwidth::BW1,
        power_mode: L3GD20PowerMode::Normal,
        zen: true,
        yen: true,
        xen: true,
        sensitivity: L3GD20GyroscopeFS::dps500,
        continuous_update: true,
        high_pass_filter_enabled: true,
        high_pass_filter_mode: Some(L3GD20GyroscopeHighPassFilterMode::NormalMode),
        high_pass_filter_configuration: Some(L3GD20HighPassFilterCutOffConfig::HPCF_0),
    };

    let mut i2cdev = get_linux_l3gd20_i2c_device().unwrap();

    let mut l3gd20_gyro = L3GD20::new(i2cdev, settings).unwrap();


    // loop

    let temperature = bmp280.temperature_celsius().unwrap();
    let pressure = bmp280.pressure_kpa().unwrap();

    println!("bmp280 Temperature: {}", temperature);
    println!("bmp280 Pressure: {}", pressure);

    match lsm9ds0.acceleration_reading() {
        Ok(v) => {
            println!("lsm9ds0 acceleration_reading: X: {}; y: {}; z: {}", v.x, v.y, v.z);
        }
        Err(e) => println!("lsm9ds0 acceleration_reading error: {}", e),
    }

    match lsm9ds0.angular_rate_reading() {
        Ok(v) => {
            println!("lsm9ds0 angular_rate_reading: X: {}; y: {}; z: {}", v.x, v.y, v.z);
        }
        Err(e) => println!(" lsm9ds0 angular_rate_reading error: {}", e),
    }


    let angular_rate = l3gd20_gyro.angular_rate_reading().unwrap();
    println!("L3GD20 angular_rate_reading: X: {}; y: {}; z: {}", angular_rate.x, angular_rate.y, angular_rate.z);

    thread::sleep(time::Duration::from_millis(300));
}
