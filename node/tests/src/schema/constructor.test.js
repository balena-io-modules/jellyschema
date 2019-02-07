const jels = require('jellyschema');

test('can be instantiated from a string', () => {
    expect(
        new jels.JellySchema('title: Foo\nversion: 1\n')
    ).toBeDefined();
});

test('can be instantiated from an object', () => {
    expect(
        new jels.JellySchema({
            title: 'Foo',
            version: 1
        })
    ).toBeDefined();
});

test('throws in case of invalid schema (string)', () => {
    expect(
        () => {
            js = new jels.JellySchema('foo');
        }
    ).toThrow();
});

test('throws in case of invalid schema (boolean)', () => {
    expect(
        () => {
            js = new jels.JellySchema(true);
        }
    ).toThrow();
});

test('throws in case of invalid schema (undefined)', () => {
    expect(
        () => {
            js = new jels.JellySchema(undefined);
        }
    ).toThrow();
});

test('throws in case of invalid schema (null)', () => {
    expect(
        () => {
            js = new jels.JellySchema(null);
        }
    ).toThrow();
});

test('throws in case of invalid schema (number)', () => {
    expect(
        () => {
            js = new jels.JellySchema(10);
        }
    ).toThrow();
});
