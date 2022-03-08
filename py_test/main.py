import grpc
import datetime
import google.protobuf.timestamp_pb2
import proto.base_types_pb2
import proto.officer_appointment_pb2
import proto.company_incorporation_pb2
import proto.form_submission_pb2
import proto.psc_pb2
import proto.ch_ewf_pb2_grpc

registered_office = proto.base_types_pb2.UKAddress(
    premise="11",
    street="Test Street",
    post_town="Cardiff",
    country=proto.base_types_pb2.UKAddress.Wales,
    postcode="CF1 1AA",
    po_box="12345"
)

residential_address = proto.base_types_pb2.BaseAddress(
    premise="17",
    street="Test Drive",
    post_town="London",
    postcode="EC1 1AD",
    country="GB"
)

company_address = proto.base_types_pb2.CompanyAddress(
    base_address=residential_address,
    po_box="1234",
)

personal_attributes = [proto.company_incorporation_pb2.PersonalAttribute(
    personal_attribute=proto.company_incorporation_pb2.PersonalAttribute.PassportNumber,
    personal_data="123",
), proto.company_incorporation_pb2.PersonalAttribute(
    personal_attribute=proto.company_incorporation_pb2.PersonalAttribute.NationalInsurance,
    personal_data="PH6"
), proto.company_incorporation_pb2.PersonalAttribute(
    personal_attribute=proto.company_incorporation_pb2.PersonalAttribute.Telephone,
    personal_data="075"
)]

with open("./same-name.pdf", "rb") as f:
    same_name = proto.company_incorporation_pb2.Document(
        data=f.read(),
        filename="same-name.pdf",
        content_type=proto.company_incorporation_pb2.PDF
    )

with open("./name-authorisation.pdf", "rb") as f:
    name_authorization = proto.company_incorporation_pb2.Document(
        data=f.read(),
        filename="name-authorisation.pdf",
        content_type=proto.company_incorporation_pb2.PDF
    )

with open("./modified-articles.pdf", "rb") as f:
    modified_articles = proto.company_incorporation_pb2.Document(
        data=f.read(),
        filename="modified-articles.pdf",
        content_type=proto.company_incorporation_pb2.PDF
    )

with open("./mem-shares.pdf", "rb") as f:
    memorandum_shares = proto.company_incorporation_pb2.Document(
        data=f.read(),
        filename="mem-shares.pdf",
        content_type=proto.company_incorporation_pb2.PDF
    )

with open("./mem-no-shares.pdf", "rb") as f:
    memorandum_no_shares = proto.company_incorporation_pb2.Document(
        data=f.read(),
        filename="mem-no-shares.pdf",
        content_type=proto.company_incorporation_pb2.PDF
    )


def main():
    channel = grpc.insecure_channel('localhost:50051')
    stub = proto.ch_ewf_pb2_grpc.CHFillingStub(channel)

    date_signed = google.protobuf.timestamp_pb2.Timestamp()
    date_signed.FromDatetime(datetime.datetime.now())
    dob = google.protobuf.timestamp_pb2.Timestamp()
    dob.FromDatetime(datetime.datetime(1986, 11, 10))
    company_incorporation = proto.company_incorporation_pb2.CompanyIncorporation(
        company_name="Test Company LLP Designated Only Name Authorisation",
        language=proto.form_submission_pb2.English,
        date_signed=date_signed,
        company_type=proto.company_incorporation_pb2.LLPOnlyDesignated,
        # registers_held_on_public_record=[
        #     proto.base_types_pb2.RegisterDirectors,
        #     proto.base_types_pb2.RegisterSecretaries,
        #     proto.base_types_pb2.RegisterPersonsOfSignificantControl,
        # ],
        country_of_incorporation=proto.company_incorporation_pb2.EnglandAndWales,
        registered_office=registered_office,
        articles=getattr(proto.company_incorporation_pb2, "None"),
        appointments=[proto.company_incorporation_pb2.Appointment(
            consent_to_act=True,
            member=proto.officer_appointment_pb2.Member(
                designated=True,
                person=proto.base_types_pb2.MemberPerson(
                    person=proto.base_types_pb2.PersonName(
                        forenames=["Test"],
                        surname="Person"
                    ),
                    service_address=proto.base_types_pb2.ServiceAddress(
                        address=proto.base_types_pb2.CompanyAddress(
                            base_address=residential_address
                        )
                    ),
                    date_of_birth=dob,
                    country_of_residence="England",
                    residential_address=proto.base_types_pb2.ResidentialAddress(
                        address=proto.base_types_pb2.ResidentialAddress.ResidentialAddress(
                            address=residential_address
                        )
                    )
                )
            )
        ), proto.company_incorporation_pb2.Appointment(
            consent_to_act=True,
            member=proto.officer_appointment_pb2.Member(
                designated=True,
                person=proto.base_types_pb2.MemberPerson(
                    person=proto.base_types_pb2.PersonName(
                        forenames=["Test"],
                        surname="Person"
                    ),
                    service_address=proto.base_types_pb2.ServiceAddress(
                        address=proto.base_types_pb2.CompanyAddress(
                            base_address=residential_address
                        )
                    ),
                    date_of_birth=dob,
                    country_of_residence="England",
                    residential_address=proto.base_types_pb2.ResidentialAddress(
                        address=proto.base_types_pb2.ResidentialAddress.ResidentialAddress(
                            address=residential_address
                        )
                    )
                )
            )
        )],
        pscs=proto.company_incorporation_pb2.PSCs(
            pscs=[proto.company_incorporation_pb2.PSC(
                notification=proto.psc_pb2.Notification(
                    individual=proto.psc_pb2.Individual(
                        person=proto.base_types_pb2.PersonName(
                            forenames=["Test"],
                            surname="Person"
                        ),
                        service_address=proto.base_types_pb2.ServiceAddress(
                            address=proto.base_types_pb2.CompanyAddress(
                                base_address=residential_address
                            )
                        ),
                        date_of_birth=dob,
                        nationality="British",
                        country_of_residence="England",
                        residential_address=proto.base_types_pb2.ResidentialAddress(
                            address=proto.base_types_pb2.ResidentialAddress.ResidentialAddress(
                                address=residential_address
                            )
                        ),
                        consent_statement=True
                    )
                ),
                nature_of_control=proto.psc_pb2.NatureOfControls(
                    llp_nature_of_controls=proto.psc_pb2.LLPNatureOfControls(
                        nature_of_controls=[
                            proto.psc_pb2.LLPNatureOfControls.RightToSurplusAssets75To100,
                            proto.psc_pb2.LLPNatureOfControls.VotingRights75To100,
                            proto.psc_pb2.LLPNatureOfControls.RightToAppointAndRemoveMembers,
                        ]
                    )
                )
            )]
        ),
        # statement_of_capital=[proto.base_types_pb2.Capital(
        #     total_amount_unpaid=0,
        #     total_number_of_shares_issued=1,
        #     currency="GBP",
        #     total_aggregate_nominal_value=1,
        #     shares=[proto.base_types_pb2.Share(
        #         share_class="Ordinary",
        #         prescribed_particulars="Test",
        #         num_shares=1,
        #         aggregate_nominal_value=1
        #     )]
        # )],
        # subscribers=[proto.company_incorporation_pb2.Subscriber(
        #     person=proto.company_incorporation_pb2.Person(
        #         person=proto.base_types_pb2.Person(
        #             forename="Test",
        #             surname="Person"
        #         ),
        #         personal_attributes=personal_attributes,
        #         address=residential_address
        #     ),
        #     allotments=[proto.company_incorporation_pb2.Allotment(
        #         share_class="Ordinary",
        #         num_shares=1,
        #         amount_paid_due_per_share=1,
        #         amount_unpaid_per_share=0,
        #         share_currency="GBP",
        #         share_value=1
        #     )],
        #     memorandum_statement=proto.company_incorporation_pb2.MemberWithShares,
        # )],
        member=proto.company_incorporation_pb2.Authorizer(
            person=proto.base_types_pb2.Person(
                forename="Test",
                surname="Person"
            ),
            personal_attributes=personal_attributes,
        ),
        same_day=False,
        # sic_codes=["12345"]
        name_authorization=name_authorization
        # same_name=same_name,
    )
    print(stub.CompanyIncorporation(company_incorporation))


if __name__ == "__main__":
    main()
