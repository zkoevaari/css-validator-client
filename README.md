css-validator-client
====================

CLI client for the W3C CSS validator API.

This is an independent project without any affiliation to W3C. We would like
to express our gratitude to our colleagues at W3C for maintaining the service,
thus making this project possible.

__Important__: Please operate the tool responsibly, and observe any conditions
that may apply to using the service itself.

To quote the official documentation:

> Good manners and respectful usage of shared resources are of course
> customary.

> If you wish to call the validator programmatically for a batch of documents,
> please make sure that your script will sleep for at least 1 second between
> requests. The CSS Validation service is a free, public service for all, your
> respect is appreciated.


Links
-----

- [CSS Validation Service](https://jigsaw.w3.org/css-validator/)
- [CSS Validator User's Manual](
    https://jigsaw.w3.org/css-validator/manual.html
)
- [CSS Validator Web Service API documentation](
    https://jigsaw.w3.org/css-validator/api.html
)


Status
------

Version 0.1.0 fulfills the requirements for a minimally viable product:
- able to load a file or read the input from STDIN
- makes the API call
- prints the `text/plain` response body

### TODO:
- Add command line options to support API parameters, like output format,
    CSS profile, URI input.
- Support using a local JAR executable instead of the online service.
- Parse the SOAP output properly to provide a more concise output format.
- On large input files, do the slicing automatically.


Usage
-----

Invoking the program with the `--help` option, it prints the following:

```
CLI client for the W3C CSS validator API.

Important: Please operate the tool responsibly, with respectful usage of
shared resources. If you wish to call the validator programmatically for a
batch of documents, please make sure that your script will sleep for at least
1 second between requests.

Usage: css-validator-client [OPTIONS] <FILE>

Arguments:
  <FILE>
          Input file, or use '-' to read from STDIN

          Maximum supported percent-encoded length (which is to be used in the
          request URL) is 8KB. See the "--length" option to help if you need
          to slice your input.

Options:
  -l, --length
          Print the calculated length of input, then exit

          This can help with the 8KB payload size limitation, which seem to be
          related to the percent-encoded length.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Created by Zoltan Kovari, 2025. Licensed under the Apache License, Version 2.0
```
