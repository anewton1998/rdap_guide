# Conformance Tools

Conformance tools or validators evaluate the RDAP servers against RFCs
and extensions.

## Services

### RDAP.Org

* Service: <https://validator.rdap.org/>
* Related: [Software](#validatorrdaporg)
* Validations:
  * Vanilla (IETF STD 95)
  * gTLD registry (February 2024 gTLD RDAP profile)
  * gTLD registrar (February 2024 gTLD RDAP profile)
  * RIR (January 2021 NRO RDAP Profile)

## Software

### APNIC RDAP Conformance

> Tests an RDAP server for conformance with the published standards. See RFC7480 and http://datatracker.ietf.org/wg/weirds.

* License: BSD
* Repository: <https://github.com/APNIC-net/rdap-conformance>

### CentralNIC RDAP Conformance

> A script to validate the conformance of an RDAP server.

* License: Unknown
* Repository: <https://gitlab.centralnic.com/centralnic/rdap-conformance>
* Archived

### ICANN RDAP Conformance Tool

> The RDAP Conformance Tool is a stand-alone tool acting like a test suite that verifies 
> the conformity of an RDAP server against the specifications developed by the IETF 
> (RFC7481, RFC7482, RFC7483, RFC7484) and the ICANN gTLD RDAP profile
> <https://www.icann.org/gtld-rdap-profile>. It only tests RDAP servers related to domains. 
> Apart from generic RDAP tests, there are no specific tests for IP addresses and AS Numbers RDAP servers.

* License: Apache License, Version 2.0
* Repository: <https://github.com/icann/rdap-conformance-tool>
* Features:
  * Command-line module
  * RDAP validation library
* Validations:
  * Standard IETF RFCs
  * gTLD Registry (thin or thick)
  * gTLD Registrar
  * ICANN gTLD Profile February 2019

### Validator.RDAP.Org

> This repository contains:
> * JavaScript library which implements an RDAP validator;
> * CLI tool that uses the library;
> * single-page web application which provides a web frontend.

* License: Unknown Open Source
* Repository: <https://github.com/rdap-org/validator.rdap.org>
* Related: [Service](#rdaporg)
* Validations:
  * Vanilla (IETF STD 95)
  * gTLD registry (February 2024 gTLD RDAP profile)
  * gTLD registrar (February 2024 gTLD RDAP profile)
  * RIR (January 2021 NRO RDAP Profile)
