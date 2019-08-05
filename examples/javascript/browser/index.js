import * as jels from "jellyschema";

const initialValue = `
title: demo
version: 1
properties:
  - network:
      title: Network
      properties:
        - ssid:
            title: Network SSID
            type: string
            minLength: 1
            maxLength: 32
        - passphrase:
            title: Network Key
            type: password
            minLength: 8
  - advanced:
      title: Advanced
      properties:
        - hostname:
            title: Device Hostname
            type: string
        - persistentLogging:
            title: Do you want to enable persistent logging?
            type: boolean
            default: false
`;
const stringify = (value) => JSON.stringify(value, null, 2);
const $source = document.getElementById('source');
const $result = document.getElementById('result');

$source.value = initialValue;

const evaluate = () => {
    try {
        const value = $source.value;
        var schema = new jels.JellySchema(value);
        schema.validate({network:{ssid: "FOO", passphrase: "BAR"}});
        $result.innerText = stringify(schema.errors());
    } catch (error) {
        console.error(error)
    }
}

evaluate();

$source.addEventListener('input', evaluate, false)
