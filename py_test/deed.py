import grpc
import datetime
import google.protobuf.timestamp_pb2
import proto.charge_registration_pb2
import proto.form_submission_pb2
import proto.ch_ewf_pb2_grpc
import proto.base_types_pb2

with open("./deed.pdf", "rb") as f:
    deed = proto.base_types_pb2.Document(
        data=f.read(),
        filename="deed.pdf",
        content_type=proto.base_types_pb2.PDF
    )

personal_attributes = [proto.base_types_pb2.PersonalAttribute(
    personal_attribute=proto.base_types_pb2.PersonalAttribute.PassportNumber,
    personal_data="123",
), proto.base_types_pb2.PersonalAttribute(
    personal_attribute=proto.base_types_pb2.PersonalAttribute.NationalInsurance,
    personal_data="PH6"
), proto.base_types_pb2.PersonalAttribute(
    personal_attribute=proto.base_types_pb2.PersonalAttribute.Telephone,
    personal_data="075"
)]


def main():
    channel = grpc.insecure_channel('localhost:50051')
    stub = proto.ch_ewf_pb2_grpc.CHFillingStub(channel)

    date_signed = google.protobuf.timestamp_pb2.Timestamp()
    date_signed.FromDatetime(datetime.datetime.now())
    charge_registration = proto.charge_registration_pb2.ChargeRegistration(
        form_submission=proto.form_submission_pb2.FormSubmission(
            company_number=12345678,
            company_name="Test Company Limited",
            company_type=proto.base_types_pb2.CompanyEnglandAndWales,
            date_signed=date_signed,
            language=proto.form_submission_pb2.English,
            authentication_code="ABCDEFG"
        ),
        creation_date=date_signed,
        persons_entitled=["Barry Bee", "Person 2", "Person 3", "Person 4"],
        additional_persons_entitled=True,
        floating_charge=proto.charge_registration_pb2.CoversAll,
        bare_trustee=True,
        deed_certification_statement="Test Statement",
        deed_certified_by="Test Person",
        personal_attributes=personal_attributes,
        deed=deed,
        deed_supplemental=deed
    )
    print(stub.ChargeRegistration(charge_registration))


if __name__ == "__main__":
    main()
