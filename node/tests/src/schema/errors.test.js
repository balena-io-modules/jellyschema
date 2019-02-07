const jels = require('jellyschema');

test('no errors after instantiation', () => {
    expect(
        new jels.JellySchema({
            title: 'Foo',
            version: 1
        }).errors()
    ).toEqual([]);
});

test('errors are cleared if validation succeeds', () => {
    schema = new jels.JellySchema({
        type: 'integer'
    });
    expect(schema.validate('foo')).toBeFalsy();
    expect(schema.errors().length).toBeGreaterThan(0);
    expect(schema.validate(10)).toBeTruthy();
    expect(schema.errors().length).toBe(0);
});

test('errors are not stacked up for consequent validation fails', () => {
    schema = new jels.JellySchema({
        type: 'integer'
    });
    expect(schema.validate('foo')).toBeFalsy();
    errors_count = schema.errors().length;
    expect(errors_count).toBeGreaterThan(0);
    expect(schema.validate('foo')).toBeFalsy();
    expect(errors_count).toBe(schema.errors().length);
});
