// Simple test that it works. Rest is tested in the Rust code.

const jels = require('jellyschema');

test('stateless', () => {
    schema = {
        properties: [
            {
                foo: {
                    type: "string",
                    default: "bar"
                }
            }
        ]
    };

    input = {};

    expected = {
        foo: "bar"
    };

    expect(jels.fillDefaultValues(schema, input)).toEqual(expected);
});

