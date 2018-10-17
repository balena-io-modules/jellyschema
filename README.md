# balena-configuration-dsl

This library serves as an entry point to the reconfiguration ecosystem.

For the main usecase the API surface needs to be similar to (work in progress):

* [ ] Schema compiler (parse & validate YAML schema)
  * Input - YAML schema
  * Output - compiled schema + some internal caching structures probably - temporary named as `XYZ` type

* [ ] UI descriptions generator
  * Input - XYZ
  * Output - XYZ, JSON Schema & uiObject

* [ ] Targets extraction
  * Input - XYZ
  * Output - XYZ, list of targets (= config files/folders) to read

* [ ] Mapping from config files
  * Input - raw file contents, XYZ, target names
  * Output - XYZ, dry JSON (form data)

* [ ] Mapping to config files
  * Input - dry JSON (form data), XYZ
  * Output - XYZ, raw file contents, target names

* [ ] Data validation (balena-data-validation)
  * Input - dry JSON (form data), XYZ
  * Output - XYZ, list of errors

* [ ] Templating -> balena-templating
  * Input - dry JSON (context), XYZ
  * Output - XYZ, dry JSON [&& list of errors]