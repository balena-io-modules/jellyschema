const jels = require('jellyschema');

test('stateful JSON & UI schema generation', () => {
    schema = new jels.JellySchema({
        title: "Foo",
        properties: [
            {
                name: {
                    type: "string",
                    title: "Name",
                    description: "Your full name"
                }
            }
        ]
    });

    expected = {
        jsonSchema: {
            "$$order": [
                "name"
            ],
            "$schema": "http://json-schema.org/draft-04/schema#",
            additionalProperties: false,
            properties: {
                name: {
                    title: "Name",
                    type: "string",
                    description: "Your full name"
                }
            },
            required: [
                "name"
            ],
            title: "Foo",
            type: "object"
        },
        uiSchema: {
            "ui:order": [
                "name"
            ]
        }
    };

    expect(schema.jsonAndUiSchema()).toEqual(expected);
});

test('stateless JSON & UI schema generation', () => {
    schema = {
        title: "Foo",
        properties: [
            {
                name: {
                    type: "string",
                    title: "Name",
                    description: "Your full name"
                }
            }
        ]
    };

    expected = {
        jsonSchema: {
            "$$order": [
                "name"
            ],
            "$schema": "http://json-schema.org/draft-04/schema#",
            additionalProperties: false,
            properties: {
                name: {
                    title: "Name",
                    type: "string",
                    description: "Your full name"
                }
            },
            required: [
                "name"
            ],
            title: "Foo",
            type: "object"
        },
        uiSchema: {
            "ui:order": [
                "name"
            ]
        }
    };

    expect(jels.generateJsonAndUiSchema(schema)).toEqual(expected);
});
