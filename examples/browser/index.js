import * as cdsl from "balena-cdsl";

const initialValue = `
title: resin-cli demo
version: 1
definitions:
  bootPartition: &BOOT_PARTITION
    partition: 1
mapping:
  targets:
    config_json:
      type: file
      format: json
      location:
        << : *BOOT_PARTITION
        path: /config.json
    resin_wifi:
      type: file
      format: ini
      location:
        << : *BOOT_PARTITION
        path: /system-connections/resin-wifi
properties:
  - network:
      title: Network
      properties:
        - ssid:
            title: Network SSID
            type: string
            minLength: 1
            maxLength: 32
            mapping:
              target: resin_wifi
              path: wifi.ssid
        - passphrase:
            title: Network Key
            type: password
            minLength: 8
            mapping:
              target: resin_wifi
              path: wifi-security.psk
      mapping:
        target: resin_wifi
        template:
          connection:
            type: wifi
          wifi:
            hidden: true
            mode: infrastructure
          wifi-security:
            auth-alg: open
            key-mgmt: wpa-psk
          ipv4:
            method: auto
          ipv6:
            addr-gen-mode: stable-privacy
            method: auto
  - advanced:
      title: Advanced
      properties:
        - hostname:
            title: Device Hostname
            type: string
            mapping:
              target: config_json
              path: hostname
        - persistentLogging:
            title: Do you want to enable persistent logging?
            type: boolean
            default: false
            mapping:
              target: config_json
              path: persistentLogging
`;
const stringify = (value) => JSON.stringify(value, null, 2);
const $source = document.getElementById('source');
const $result = document.getElementById('result');

$source.value = initialValue;

const evaluate = () => {
    try {
        const value = $source.value;
        const result = cdsl.generate_ui(value);
        $result.innerText = stringify(result)
    } catch (error) {
        console.error(error)
    }
}

evaluate();

$source.addEventListener('input', evaluate, false)
