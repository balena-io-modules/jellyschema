// Simple test that it works. Rest is tested in the Rust code.

const jels = require('jellyschema');

test('stateless', () => {
    schema = new jels.JellySchema({
        properties: [
            {
                foo: {
                    type: "string",
                    default: "bar"
                }
            }
        ]
    });

    input = {};

    expected = {
        foo: "bar"
    };

    expect(schema.fillDefaultValues(input)).toEqual(expected);
});

