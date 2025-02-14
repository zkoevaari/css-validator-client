css-validator-client
====================

CLI client for the W3C CSS validator API.

This is an independent project without any affiliation to W3C. We would like to express our gratitude to our colleagues at W3C for maintaining the service, thus making this project possible.

__Important__: Please operate the tool responsibly, and observe any conditions that may apply to using the service itself.

To quote the official documentation:

> Good manners and respectful usage of shared resources are of course customary.

> If you wish to call the validator programmatically for a batch of documents, please make sure that your script will sleep for at least 1 second between requests.
> The CSS Validation service is a free, public service for all, your respect is appreciated.


Links
-----

- [CSS Validation Service](https://jigsaw.w3.org/css-validator/)
- [CSS Validator User's Manual](https://jigsaw.w3.org/css-validator/manual.html)
- [CSS Validator Web Service API documentation](https://jigsaw.w3.org/css-validator/api.html)


Status
------

The project is under development.

Version 0.1.0 will be able to load a file or read the input from STDIN, make the API call and print the response body.

### TODO:
- Add command line options to support API parameters, like output format, CSS profile, URI input.
- Support using a local JAR executable instead of the online service
- Parse the SOAP output properly to provide a more concise output format
