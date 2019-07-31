Contributions by: @zrzka @jviotti @alexandrosm @petrosagg

# Introduction

Key words to indicate requirement levels (MUST, SHALL, SHOULD, ...) are described in the
[RFC 2119](https://tools.ietf.org/html/rfc2119) document.

This document covers the Configuration Schema specification. This document DOES NOT
cover transformation from / to configuration files.

# Current State

JSON Schema is being used across many tools / libraries. It can be very complicated and
it's not human friendly at all. We have to provide different way how to write
the Configuration Schemas.

# Target State

A human friendly, easy to write, Configuration Schema. This Configuration Schema will
act as a single source of truth and we will use it to generate JSON Schema, user
interfaces and to validate data.

# Benefits

An easy to write, human friendly, Configuration Schema. No one has to learn how to
write the JSON Schema.

# Implementation Approach

## YAML Format

Configuration Schema will be written in the YAML format, due to its simplicity
and expressiveness.

* [YAML Ain’t Markup Language (YAML™) Version 1.2](http://yaml.org/spec/1.2/spec.html)
* [Reference card](http://yaml.org/refcard.html)

### Style

Configuration Schema is a human friendly, easy to edit, format. The following
style rules are introduced to maintain clarity and readability.

#### Indentation

Indentation should be set to two spaces.

#### Flow Mappings

The Configuration Schema MUST not use [flow mappings](http://yaml.org/spec/1.2/spec.html#id2790832)
unless they're explicitly allowed in this specification.

```yaml
# Flow mapping (NOT allowed)
phone: { prefix: +420, number: 123456 }

# Block mapping (allowed)
another-phone:
  prefix: +420
  number: 123456
```

#### Flow Sequences

The Configuration Schema MUST not use [flow sequences](http://yaml.org/spec/1.2/spec.html#id2790320)
unless they're explicitly allowed in this specification.

```yaml
# Flow sequence (NOT allowed)
flow: [a, b]

# Block sequence (allowed)
block:
  - a
  - b
```

#### Block Sequence Values Indentation

The Configuration Schema MUST indent [block sequence values](http://yaml.org/spec/1.2/spec.html#id2797382).

```yaml
# Not indented sequence values (NOT allowed)
first:
- a
- b

# Indented sequence values (allowed)
second:
  - a
  - b
```

#### Multi Line Strings

YAML provides a couple of ways how to write a string. Except plain string,
[block style](http://yaml.org/spec/1.2/spec.html#id2795688) and
[block chomping indicator](http://yaml.org/spec/1.2/spec.html#id2794534) can be used. If you're
unsure, visit [Find the right syntax for your YAML multiline strings](https://yaml-multiline.info/) site
for more info.

When using a style or chomping indicator, always choose one that maintains readability and clarity.

Block styles:

* `>` - folded - replace new lines with spaces
* `|` - literal - keep new lines

Block chomping indicators:

* N/A - clip - single new line at the end
* `-` - strip - no new line at the end
* `+` - keep - all new lines from end

Plain string example:

```yaml
# "Hallo world!"
plain: Hallo world!
```

Literal string example:

```yaml
# "Hallo\nworld!\nI'm here!"
literal: |
  Hallo
  world!
  
  I'm here!
```

**WARNING:** Be aware of the additional new line at the end ...

```yaml
# "Hallo\nworld!\nI'm here!\n"   <- \n at the end
literal: |
  Hallo
  world!
  
  I'm here!

```

... because of the empty line at the end.

Stripped literal string example:

```yaml
# "Hallo\nworld!\nI'm here!"    <- No \n at the end because of -
literal: |-
    Hallo
    world!

    I'm here!

```

Folded string example:

```yaml
# "Hallo world!\nI'm here!"
folded: >
    Hallo
    world!

    I'm here!
```

**WARNING:** Be aware of the additional new line at the end ...

```yaml
# "Hallo world!\nI'm here!\n"   <- \n at the end
literal: >
  Hallo
  world!
  
  I'm here!

```

... because of the empty line at the end.

Stripped folded string example:

```yaml
# "Hallo world!\nI'm here!"    <- No \n at the end because of -
literal: >-
    Hallo
    world!

    I'm here!

```

#### Aliasing

[YAML aliases](http://yaml.org/spec/1.2/spec.html#alias//) MUST NOT be used.

## Meta Schema

The current version for the configuration schema validation is `1`. Every schema
SHOULD reference version via `version` keyword. Omitting this keyword has the
same behavior as a value of `1`.

Example:

```yaml
# Version is specified, MUST be 1, no other versions are supported
version: 1
title: Raspberry Pi 3B+ Configuration
```

```yaml
# Version omitted, implies `version: 1`
title: Raspberry Pi 3B+ Configuration
```

## Validation Keywords

### Validation Keywords for Any Instance Type

#### type

The value of this keyword MUST be a string.

String values MUST be one of the seven primitive types `null`, `boolean`,
`object`, `array`, `number`, `string`, `integer` or one of the
abstract data types.

An instance validates if and only if the instance is in any of the
sets listed for this keyword.

Omitting this keyword has the same behavior as a value of `object`.

Example:

```yaml
properties:
  - networking:
      type: object   # The type has been explicitly declared as "object"
      title: Networking
```

```yaml
properties:
  - networking:
      # The type keyword has been omitted, so the type defaults to "object"
      title: Networking
```

##### Optional Type

All properties are required by default. Type suffix `?` can be used to mark
a property as optional.

Example:

```yaml
properties:
  - ssid:
      # Property ssid is required
      type: string
  - name:
      # Property name is optional (? type suffix)
      type: string?
```

Optional properties can be missing or their values can be `null`.

##### Abstract Data Types

###### password

* Value is of type `string`
* Validation keywords for `string` can be used
* It implies that the input should be masked

Example:

```yaml
properties:
  - psk:
      title: Passphrase
      type: password
```

##### Dates and Times

The following profile of [ISO 8601](https://tools.ietf.org/html/rfc3339#ref-ISO8601) dates MUST be used. This is
specified using the syntax description notation defined in [ABNF](https://tools.ietf.org/html/rfc3339#ref-ABNF).

```text
date-fullyear   = 4DIGIT
date-month      = 2DIGIT  ; 01-12
date-mday       = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on
                         ; month/year
time-hour       = 2DIGIT  ; 00-23
time-minute     = 2DIGIT  ; 00-59
time-second     = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second
                         ; rules
time-secfrac    = "." 1*DIGIT
time-numoffset  = ("+" / "-") time-hour ":" time-minute
time-offset     = "Z" / time-numoffset

partial-time    = time-hour ":" time-minute ":" time-second
                 [time-secfrac]
full-date       = date-fullyear "-" date-month "-" date-mday
full-time       = partial-time time-offset

date-time       = full-date "T" full-time
```

###### date-time

`date-time` - A string instance is valid against this attribute if it
is a valid representation according to the `date-time` production
([ABNF](https://tools.ietf.org/html/rfc3339#ref-ABNF)).

Example:

```yaml
properties:
  - expiresAt:
      type: datetime
      title: Subscription expiration date
```

Valid value examples:

* `2018-10-24T10:20:30Z`
* `2018-10-24T10:20:30.5Z`
* `2018-10-24T10:20:30+02:00`

###### date

`date` - A string instance is valid against this attribute if it is a
valid representation according to the `full-date` production
([ABNF](https://tools.ietf.org/html/rfc3339#ref-ABNF)).

Example:

```yaml
properties:
  - birthday:
      type: date
      title: Enter your birth date
```

Valid value examples:

* `2018-10-20`

###### time

`time` - A string instance is valid against this attribute if it is a
valid representation according to the `full-time` production
([ABNF](https://tools.ietf.org/html/rfc3339#ref-ABNF)).

Example:

```yaml
title: Roomba Schedule
properties:
  - schedule:
      properties:
        - monday:
            title: Monday
            properties:
              - startAt:
                  type: time
                  title: Start cleaning at
```

Valid value examples:

* `10:20:30`
* `10:20:30.5`
* `10:20:30.5Z`
* `10:20:30.5+02:00`

##### email

`email` - As defined by [RFC 5322, section 3.4.1](https://tools.ietf.org/html/rfc5322#section-3.4.1).

Example:

```yaml
properties:
  - email:
      type: email
      title: Enter your email
```

Valid value examples:

* `robert@resin.io`
* `robert+shopping@resin.io`

##### hostname

`hostname` - As defined by [RFC 1034, section 3.1](https://tools.ietf.org/html/rfc1034#section-3.1),
including host names produced using the Punycode algorithm specified in
[RFC 5891](https://tools.ietf.org/html/rfc5891), [section 4.4](https://tools.ietf.org/html/rfc5891#section-4.4).

To simplify implementations, the total number of octets that represent a domain name (i.e., the sum
of all label octets and label lengths) is limited to 255.

Example:

```yaml
title: VPN
properties:
  - hostname:
      type: hostname
      title: Enter your VPN server hostname
```

Valid value examples:

* `zrzka.robertvojta.com`
* `zrzka`

##### port

`port` - Represents port number. `port` is effectively an `integer` with
`min` set to `0` and `max` to `65535`.

Example:

```yaml
# Port number in the 0 - 65535 range
properties:
  - port:
      title: Port number
      type: port
```

```yaml
# Only unprivileged ports in the 1024 - 65535 range
properties:
  - unprivilegedPort:
      title: Port number
      type: port
      min: 1024
```

##### IP Addresses

###### ip-address

`ip-address` - An IPv4 address according to the "dotted-quad" ABNF syntax as
defined in [RFC 2673, section 3.2](https://tools.ietf.org/html/rfc2673#section-3.2)
or an IPv6 address as defined in [RFC 4291, section 2.2](https://tools.ietf.org/html/rfc4291#section-2.2).

Example:

```yaml
title: VPN
properties:
  - host:
      type: ip-address
      title: Enter your VPN server
```

Valid value examples:

* `208.116.0.0`
* `208.116.0.0/14`
* `ABCD:EF01:2345:6789:ABCD:EF01:2345:6789`

###### ipv4-address

`ipv4-address` - An IPv4 address according to the "dotted-quad" ABNF syntax as
defined in [RFC 2673, section 3.2](https://tools.ietf.org/html/rfc2673#section-3.2).

Example:

```yaml
title: VPN
properties:
  - host:
      type: ipv4-address
      title: Enter your VPN server IPv4 address
```

Valid value examples:

* `208.116.0.0`
* `208.116.0.0/14`

###### ipv6-address

`ipv6-address` - An IPv6 address as defined in
[RFC 4291, section 2.2](https://tools.ietf.org/html/rfc4291#section-2.2).

Example:

```yaml
title: VPN
properties:
  - host:
      type: ipv6-address
      title: Enter your VPN server IPv4 address
```

Valid value examples:

* `ABCD:EF01:2345:6789:ABCD:EF01:2345:6789`
* `2001:DB8:0:0:8:800:200C:417A` (or `2001:DB8::8:800:200C:417A`)
* `FF01:0:0:0:0:0:0:101` (or `FF01::101`)
* `0:0:0:0:0:0:0:1` (or `::1`)
* `0:0:0:0:0:0:0:0` (or `::`)

##### binary

`binary` - A string instance is valid against this attribute if it is a valid
base 64 encoded string, according to [RFC4648](https://tools.ietf.org/html/rfc4648#section-4).

**NOTE:** This `binary` type SHOULD be used for small binary files like logo. It SHOULD NOT
be used for big files. Raise a question in the `p/configuration` flow if you do require
support for big files.

Example:

```yaml
title: Assets
properties:
  - logo:
      type: binary
      title: Select image with your logo
```

Valid value example::

* `SGFsbG9Gcm9tQmFzZTY0` (encoded `HalloFromBase64`)

#### enum

The value of this keyword MUST be an array.  This array SHOULD have
at least one element. Elements in the array SHOULD be unique.

Every element of the array MUST be either a simple type (`string`, `number`, ...)
or an object.

Example:

```yaml
# String array
deviceType:
  type: string
  enum:
    - fincm3        # Value & title
    - raspberrypi3  # Value & title
```

```yaml
# Object array
deviceType:
  type: string
  default: raspberrypi3
  enum:
    - value: fincm3
      title: Balena Fin (CM3)
    - value: raspberrypi3
      title: Raspberry Pi 3
```

An instance validates successfully against this keyword if its value
is equal to:

* one of the elements in this keyword's array value (string array) or
* one of the elements `value` in this keyword's array value (object array).

#### const

The value of this keyword MAY be of any type, including null.

An instance validates successfully against this keyword if its value
is equal to the value of the keyword.

Example:

```yaml
# `mode` is of type string and must always contain `auto` value
mode:
  type: string
  const: auto
```

```yaml
# Equals to the previous example, enum keyword is used instead of const
mode:
  type: string
  enum:
    - auto
```

### Validation Keywords for Numeric Instances (number and integer)

#### multipleOf

The value of `multipleOf` MUST be a positive number greater than 0.
`multipleOf` MUST be used with `type: integer` only. Floating point
numbers are not supported.

A numeric instance is valid only if division by this keyword's value
results in an integer.

Example:

```yaml
updateInterval:
  type: integer
  multipleOf: 10
```

`updateInterval` value MUST be divisible by 10: 10, 20, 30, 40, etc.

#### max

The value of `max` MUST be of the same type as the property itself,
representing an inclusive upper limit.

If the instance is a number (and the property is of type `number`), then this keyword
validates only if the instance is lower than or exactly equal to `max`.

If the instance is a string (and the property is of type `date`), then this keyword
validates only if the date is lower than or exactly equal to `exclusiveMin`.

Example:

```yaml
updateInterval:
  type: integer
  max: 100
  multipleOf: 10
```

`updateInterval` value MUST be divisible by 10 and lower or equal to 100:
10, 20, ..., 100.

#### exclusiveMax

The value of `exclusiveMax` MUST be of the same type as the property itself,
representing an exclusive upper limit.

If the instance is a number (and property is of type `number`), then this keyword
validates only if the instance is lower than (not equal to) `exclusiveMax`.

If the instance is a string (and the property is of type `date`), then this keyword
validates only if the date is lower than (not equal to) `exclusiveMax`.

Either `max` or `exclusiveMax` MUST be used, but not both.

Example:

```yaml
updateInterval:
  type: integer
  exclusiveMax: 100
  multipleOf: 10
```

`updateInterval` value MUST be divisible by 10 and lower then 100:
10, 20, ..., 90.

#### min

The value of `min` MUST be of the same type as the property itself,
representing an inclusive lower limit.

If the instance is a number (and the property is of type `number`), then this keyword
validates only if the instance is greater than or exactly equal to `min`.

If the instance is a string (and the property is of type `date`), then this keyword
validates only if the date is greater than or exactly equal to `min`.

Example:

```yaml
updateInterval:
  type: integer
  min: 10
  multipleOf: 10
```

`updateInterval` value MUST be divisible by 10 and greater or equal to 10:
10, 20, 30, etc.

#### exclusiveMin

The value of `exclusiveMin` MUST be of the same type as the property itself,
representing an exclusive lower limit.

If the instance is a number (and property is of type `number`), then this keyword
validates only if the instance is greater than (not equal to) `exclusiveMin`.

If the instance is a string (and the property is of type `date`), then this keyword
validates only if the date is greater than (not equal to) `exclusiveMin`.

Either `min` or `exclusiveMin` MUST be used, but not both.

Example:

```yaml
updateInterval:
  type: integer
  exclusiveMin: 10
  multipleOf: 10
```

`updateInterval` value MUST be divisible by 10 and greater than 10:
20, 30, 40, etc.

### Validation Keywords for Strings

#### maxLength

The value of this keyword MUST be a non-negative integer.

A string instance is valid against this keyword if its length is less
than, or equal to, the value of this keyword.

The length of a string instance is defined as the number of its
UTF-8 characters.

Example:

```yaml
password:
  type: string
  maxLength: 32
```

`password` length MUST be lower or equal to 32.

#### minLength

The value of this keyword MUST be a non-negative integer.

A string instance is valid against this keyword if its length is
greater than, or equal to, the value of this keyword.

The length of a string instance is defined as the number of its
UTF-8 characters.

Example:

```yaml
password:
  type: string
  minLength: 8
```

`password` length MUST be greater or equal to 8.

#### pattern

The value of this keyword MUST be a string. This string SHOULD be a
valid regular expression, according to the
[ECMA 262 regular expression dialect](https://www.ecma-international.org/ecma-262/5.1/#sec-15.10.1).

A string instance is valid against this keyword if the pattern matches the string.

Example:

```yaml
ssid:
  type: string
  pattern: "^[a-zA-Z]{8,}$"
```

`ssid` MUST consist of eight (or more) a-z or A-Z characters.

### Validation Keywords for Arrays

#### items

The value of `items` MUST be either a valid Configuration Schema or an array
of valid Configuration Schemas.

This keyword determines how child instances validate for arrays, and
does not directly validate the immediate instance itself.

If `items` is a schema, validation succeeds if all elements in the
array successfully validate against that schema.

If `items` is an array of schemas, validation succeeds if every
instance element in any position validates against any `items` schema.

Omitting this keyword has the same behavior as an empty schema.

Example:

```yaml
networks:
  type: array
  items:
    properties:
      - ssid:
          type: string
```

All `networks` items MUST validate against given schema (MUST contain `ssid` property
of type `string`).

```yaml
networks:
  type: array
  items:
    - properties:
        - ssid:
            type: string
    - properties:
        - name:
            type: string
```

#### maxItems

The value of this keyword MUST be a non-negative integer.

An array instance is valid against `maxItems` if its size is less
than, or equal to, the value of this keyword.

#### minItems

The value of this keyword MUST be a non-negative integer.

An array instance is valid against `minItems` if its size is greater
than, or equal to, the value of this keyword.

Omitting this keyword has the same behavior as a value of 0.

#### uniqueItems

The value of this keyword MUST be a boolean or an array of strings
(relative property paths).

If this keyword has boolean value `false`, the instance validates
successfully. If it has boolean value `true`, the instance validates
successfully if all of its elements are unique.

Omitting this keyword has the same behavior as a value of `false`.

Example:

```yaml
names:
  type: array
  uniqueItems: true
  items:
    type: string
```

In this example, all provided `names` MUST be strings and they MUST be unique.

If the `uniqueItems` keyword value is an array of strings, the instance
validates successfully if selected properties are unique.

Example:

```yaml
networks:
  type: array
  uniqueItems:
    - $.wifi.ssid
  items:
    properties:
      - wifi:
          properties:
            - ssid:
                type: string
      - wifi-security:
          properties:
            - psk:
                type: password
```

An array instance is valid if all items are valid against given schema
and `$.wifi.ssid` property values are unique.

### Validation Keywords for Objects

#### properties

The value of `properties` MUST be an array of objects. Each value of this
object MUST be a valid Configuration Schema.

This keyword determines how child instances validate for objects, and
does not directly validate the immediate instance itself.

Validation succeeds if, for each name that appears in both the
instance and as a name within this keyword's value, the child
instance for that name successfully validates against the
corresponding schema.

Omitting this keyword has the same behavior as an empty array.

Example:

```yaml
properties:
  - ssid:
      type: string
  - password:
      type: string
```

## Schema Annotations

These general-purpose annotation keywords provide commonly used
information for documentation and user interface display purposes.

### title

The value MUST be a string. Defaults to `null`.

This keyword can be used to decorate a user interface with
information about the data produced by this user interface.

A title should be short.

### description

The value MUST be a string. Defaults to `null`.

This keyword can be used to decorate a user interface with
information about the data produced by this user interface.

Provides explanation about the purpose of the instance described by
this schema.

The value can be written in plain text or Markdown. The user interface SHOULD attempt
to render markdown values.

### help

The value MUST be a string. Defaults to `null`.

This keyword can be used to decorate a user interface with
information about the data produced by this user interface.

Sometimes it's convenient to add instructions how to fill a field.

The value can be written in plain text or Markdown. The user interface SHOULD attempt
to render markdown values.

### warning

The value MUST be a string. Defaults to `null`.

Sometimes it's convenient to add warning next to a field.

This keyword can be used to decorate a user interface with
information about the data produced by this user interface.

The value can be written in plain text or Markdown. The user interface SHOULD attempt
to render markdown values.

### default

There are no restrictions placed on the value of this keyword.  When
multiple occurrences of this keyword are applicable to a single
subinstance, implementations SHOULD remove duplicates.

This keyword can be used to supply a default property value associated
with a particular schema. A default value MUST be valid against the
associated schema.

Example:

```yaml
properties:
  - id:
      type: string
      default: resin-wifi
```

### readOnly and writeOnly

The value of these keywords MUST be a boolean. When multiple
occurrences of these keywords are applicable to a single sub-
instance, the resulting value MUST be true if any occurrence
specifies a true value, and MUST be false otherwise.

Both `readOnly` and `writeOnly` default to `false`.

If `readOnly` has a value of boolean true, it indicates that the
value of the instance is managed exclusively by the owning authority,
and attempts by an application to modify the value of this property
are expected to be ignored or rejected by that owning authority.

If `writeOnly` has a value of boolean true, it indicates that the
value is never present when the instance is retrieved from the owning
authority.  It can be present when sent to the owning authority to
update or create the document (or the resource it represents), but it
will not be included in any updated or newly created version of the
instance.

For example, `readOnly` would be used to mark a database-generated
serial number as read-only, while `writeOnly` would be used to mark a
password input field.

### hidden

The value MUST be a boolean. Defaults to `false`.

### collapsed

The value MUST be a boolean. Defaults to `false`.

Specifies if a group (object) is collapsed by default. All groups are
expanded by default.

Any object (`type: object` or `type: array`) is considered as a group. Groups can be
named (`title` set) or anonymous (`title` omitted).

### collapsible

The value MUST be a boolean. Defaults to `true`.

Specifies if a group can be collapsed.

### orderable, addable, removable

The value of these keywords MUST be a boolean. Defaults to
`true` for all these keywords.

It's valid for `array` types only and it specifies if array
items can be reordered, new items added or existing items removed.

### placeholder

The value MUST be a string.

You can add placeholder text to an input by using the `placeholder`
keyword.
