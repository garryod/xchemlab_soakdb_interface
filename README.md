# XChemLab SoakDB Interface

An interface service, providing a mechanism to save & load data from the SoakDB format. This service is intended to be used during the transition period.

## Running

To run this service, simply execute `cargo run`.

An [Open Policy Agent](https://www.openpolicyagent.org/) instance serving the polcies in `/policies` is required for authentication & authorization.

## Test Data

A helper script, `collect_test_data.sh`, is included. This can be used to collect copies of all recent SoakDB databases into the `test_data` directory.
