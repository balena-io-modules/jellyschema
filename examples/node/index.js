const jels = require('jellyschema');

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
`;

var schema = new jels.JellySchema(initialValue);
const result = schema.jsonAndUiSchema();
console.log(JSON.stringify(result, null, 2));
