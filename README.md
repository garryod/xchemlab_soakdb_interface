# XChemLab SoakDB Interface

An interface service, providing a mechanism to save & load data from the SoakDB format. This service is intended to be used during the transition period.

## Running

To run this service, simply execute `cargo run`.

The following environment variables are expected when running the `serve` subcommand:
- `OIDC_ISSUER_URL` - The URL of the OpenID Connect authentication service
- `OIDC_CLIENT_ID` - The issued OpenID Connect 'Client ID' for this application
- `OIDC_CLIENT_SECRET` - The issued OpenID Connect 'Client Secret' for this application
- `OIDC_REDIRECT_URL` - The URL of your frontend application
- `ACCESS_TOKEN_INTROSPECTION_URL` - The URL used to introspect access tokens.

## Test Data

A helper script, `collect_test_data.sh`, is included. This can be used to collect copies of all recent SoakDB databases into the `test_data` directory.
