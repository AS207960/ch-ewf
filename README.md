# Companies House eFilling Client

A client for the Companies House XML filling API, exposing the forms as gRPC services.

## How to use

Pass the pass to the settings file using the command line option `-s`. 
The file can be JSON, TOML, or YAML. A PostgreSQL (or compatible) database is required.

```toml
presenter_id = "<your presenter ID>"
presenter_code = "<your presenter authentication code>"
presenter_email = "<last resort contact email>"
package_reference = "<assigned by companies house>"
database_url = "postgres://user:pass@host/db_name"
documents_path = "<path to store response PDFs>"
```

### Presenter account 

You will need to apply for a presenter account from Companies House, either with a credit account
to file fee bearing forms, or without if you don't require that. The application forms are
[available here](https://www.gov.uk/government/publications/apply-for-a-companies-house-online-filing-presenter-account).

### Package reference

You will need to email [xml@companieshouse.gov.uk](mailto:xml@companieshouse.gov.uk) to request a test
package reference and complete the required testing to recieve a live package reference.
We're unable to openly share our own live package reference.

### gRPC spec

The gRPC protobuf files are available in [`proto/`](proto/).

### Building

```shell
cargo build --release
```

## Contributing

Contributions are very welcome and appreciated. We'd like to know if you use this
(purely for bragging rights)!

## Feature requests

Feature requests can be made by submitting a GitHub issue.

## Wanting to use this commercially?

The source code is provided fully free of charge and with no strings attached.
You're welcome to use it however you want!

If however you'd like to discuss a support contract or need help using this project
drop us an email at [hello@glauca.digital](mailto:hello@glauca.digital).

## Supported companies

* Public limited companies
* Companies limited by shares
* Companies limited by guarantee
* Unlimited companies
* Limited Liability Partnerships

## Supported forms

* AD01 – Change of Registered Office
* AD02 – Notification of Single Alternative Inspection Location
* AD03 – Change of location of company records to SAIL
* AD04 – Change of location of company records to RO
* AP01 – Appointment of Natural Director
* AP02 – Appointment of Corporate Director
* AP03 – Appointment of Natural Secretary
* AP04 – Appointment of Corporate Secretary
* TM01 – Termination of Director
* TM02 – Termination of Secretary
* CH01 – Change of Natural Director
* CH02 – Change of Corporate Director
* CH03 – Change of Natural Secretary
* CH04 – Change of Corporate Secretary
* SH01 – Allotment of Shares
* AA01 – Change of Accounting Reference Date
* EINC – Incorporation
* NM01 – Change of name with resolution
* NM04 – Change of name by means provided for in the articles
* LLAR01 – Annual Return
* LLAD01 – Change of Registered Office
* LLAD02 – Notification of Single Alternative Inspection Location
* LLAD03 – Change of location of company records to SAIL
* LLAD04 – Change of location of company records to RO
* LLAP01 – Appointment of Natural Member
* LLAP02 – Appointment of Corporate Member
* LLTM01 – Termination of Member
* LLCH01 – Change of Natural Member
* LLCH02 – Change of Corporate Member
* LLAA01 – Change of Accounting Reference Date
* LLIN01 – Incorporation
* LLNM01 – Change of name
* MR01 – Registration of a Charge
* MR02 – Registration of an Acquisition of a Charge
* MR04 – Satisfaction of a Charge
* MR05 – Release or Cease Property From a Charge
* LLMR01 – Registration of a Charge
* LLMR02 – Registration of an Acquisition of a Charge
* LLMR04 – Satisfaction of a Charge
* CS01 – Confirmation Statement and LLCS01 (LLP equivalent form)
* PSC01 - Notice of individual person with significant control
* PSC02 - Notice of relevant legal entity (RLE)
* PSC03 - Notice of other registrable person
* PSC04 - Change of individual person with significant control (PSC) details
* PSC05 - Change of relevant legal entity (RLE) details
* PSC06 - Change of other registrable person with significant control (PSC) details
* PSC07 - Notice of ceasing to be a person with significant control (PSC)
* PSC08 - Notification of additional matters
* PSC09 - Notification of end date of additional matters
* LLPSC01 - Notice of individual person with significant control for an LLP
* LLPSC02 - Notice of relevant legal entity (RLE) for an LLP
* LLPSC03 - Notice of other registrable person for an LLP
* LLPSC04 - Change of individual person with significant control (PSC) details for an LLP
* LLPSC05 - Change of relevant legal entity (RLE) details for an LLP
* LLPSC06 - Change of other registrable person with significant control (PSC) details for an LLP
* LLPSC07 - Notice of ceasing to be a person with significant control (PSC) for an LLP
* LLPSC08 - Notification of additional matters for an LLP
* LLPSC09 - Notification of end date of additional matters for an LLP
* EH01 - Election to keep information from register of directors on the public register 
* EH02 - Election to keep information from register of directors&#39; residential addresses on the public register
* EH03 - Election to keep information from register of secretaries on the public register
* EH04 - Election to keep information from register of people with significant control on the public register
* EH05 - Election to keep information from register of members on the public register   
* EH06 - Member's register information update
* EW01 - Withdrawal of election to keep information from register of directors on the public register
* EW02 - Withdrawal of election to keep information from register of directors&#39; residential addresses on the public register
* EW03 - Withdrawal of election to keep information from register of secretaries on the public register
* EW04 - Withdrawal of election to keep information from register of people with significant control on the public register
* EW05 - Withdrawal of election to keep information from register of members on the public register
* LLEH01 - Election by a Limited Liability Partnership (LLP) to keep information from register of LLP members on the public register                    
* LLEH02 - Election by a Limited Liability Partnership (LLP) to keep information from register of LLP members&#39; residential addresses on the public register   
* LLEH03 - Election by a Limited Liability Partnership (LLP) to keep information from register of people with significant control on the public register
* LLEW01 - Withdrawal of election by a Limited Liability Partnership (LLP) to keep information from register of LLP members on the public register        
* LLEW02 - Withdrawal of election by a Limited Liability Partnership (LLP) to keep information from register of LLP members&#39; residential addresses on the public register   
* LLEW03 - Withdrawal of election by a Limited Liability Partnership (LLP) to keep information from register of people with significant control on the public register

## Unsupported forms

* AR01 – Annual Return   
* AA – Annual Account 

## Additional supported services

* eReminders
* Company data
* Members data
* Payment periods