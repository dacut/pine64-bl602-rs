# pine64-bl602-sys
Rust bindings for the the Pine64 BL602 SDK

# Features
* `aws-iot`: Enables the AWS Internet of Things (IoT) library.
* `blecontroller`: Enables the Bluetooth Low Energy controller.
* `blemesh`: Enables the Bluetooth Low Energy mesh component.
  * `blemesh-friend`
  * `blemesh-low-power`
  * `blemesh-gen-server`
* `blestack`: Enables the Bluetooth Low Energy stack.
  * `blestack-bas`: Enables the Bluetooth Generic Attribute (GATT) Battery Service (BAS) client component.
  * `blestack-cli`: Enables Bluetooth CLI commands.
  * `blestack-dis`: Enables the Bluetooth Generic Attribute (GATT) Device Information Service (DIS) component.
  * `blestack-multiadv`: Enables the Bluetooth multiple advertisement sets feature component.
  * `blestack-oad-client`: Enables the Bluetooth Over the Air Download (OAD) profile client component.
  * `blestack-oad-server`: Enables the Bluetooth Over the Air Download (OAD) profile server component.
  * `blestack-scps`: Enables the Bluetooth Scan Parameters service component.
  * `blestack-smp`: Enables the Bluetooth Security Manager Protocol (SMP) component.
  * `blestack-tp`: Enables the Bluetooth Generic Attribute (GATT) Throughput Service (TP) component.
  * `blestack-wifiprov`: Enables the Bluetooth WiFi Provisioning Service component.
  * `blesync`: Enables the Bluetooth Low Energy Sync component.
* `blmtd`: Enables the Bouffalo Labs Memory Technology Device (MTD) (flash) component.
* `bltime`: Enables the Bouffalo Labs time library.
* `blog`: Enables the Bouffalo Labs logging component.
* `bloop`: Enables the Bouffalo Labs event loop.
  * `looprt`: Enables integration of the Bouffalo Labs event loop with FreeRTOS.
  * `loopset`: Enables device (I2C, IR, LED, PWM) integration with the Bouffalo Labs event loop.
* `cjson`: Enables the cJSON library.
* `cli`: Enables the Bouffalo Labs command line interface (over USB UART).
* `easyflash`: Enables the EasyFlash component.
* `hal`: Enables the Hardware Abstraction Layer (HAL) driver component.
* `lwip`: Enables the Lightweight TCP/IP stack.
  * `dns-server`: Enables the Domain Name Service (DNS) server component.
  * `httpc`: Enables the Hypertext Transfer Protocol (HTTP) client component.
  * `https`: Enables the Hypertext Transfer Protocol (HTTP) server component.
  * `lwip-altcp-tls-mbedtls`: Enables the application-layered TCP/TLS connection API.
  * `lwip-dhcpd`: Enables the lwIP Dynamic Host Configuration Protocol (DHCP) daemon.
  * `lwip-mdns`: Enables the lwIP multicast DNS (MDNS) responder component.
  * `mbedtls`: Enables the Mbed Transport Layer Security (TLS) encryption component.
  * `netutils`: Enables network utilities (iperf, netstat, ping, TCP clients and servers).
  * `sntp`: Enables the Simple Network Time Protocol (SNTP) client.
* `romfs`: Enables the ROM file system.
* `std`: Enables usage of the Rust `std` library.
* `utils`: Enables utilities such as CRC, DNS, HMAC, SHA.
* `vfs`: Enables the virtual file system.
* `wifi`: Enables the WiFi component.
* `yloop`: Enables the AliOS Things Yloop Event framework.

# Configuration options

The following options can be passed to your build to enable various build-time options. For example,
you can enable tickless mode on the command line with:
```
cargo build --target riscv32imc-unknown-none-elf --cfg freertos_tickless_mode
```

* `enable_psm_ef_size=4k` `enable_psm_ef_size=8k` `enable_psm_ef_size=16k`: Set the easyflash size
    to the specified value. Defaults to 4k.
* `dts_inapp`: Compiles a device-tree into the application for the HAL rather than loading it from
    the bootloader.
* `freertos_tickless_mode`: Configure FreeRTOS to use tickless mode.
* `sys_big_debug_buffer`: Expands the debug buffer to 2k (instead of 512 bytes).
