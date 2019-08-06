-- we don't know how to generate root <with-no-name> (class Root) :(
create table AK_sex_offender_aliases
(
	ID,
	Aliases,
	state
);

create table AK_sex_offender_main
(
	CurrentReportDate,
	CurrentStatus,
	DOB,
	DatabaseLastUpdated,
	Employer,
	Eyes,
	Hair,
	Height,
	ID,
	Name,
	Race,
	RegisteredUnder,
	Sex,
	Weight,
	state
);

create table AK_sex_offender_offenses
(
	ID,
	CourtDocketNumber,
	Court,
	ConvictionDate,
	OffenseDate,
	Statute,
	Description,
	state
);

create table AK_sex_offender_photos
(
	ID,
	PhotoFile,
	state
);

create table AR_sex_offender_alias
(
	ID,
	Alias,
	state
);

create table AR_sex_offender_photos
(
	ID,
	Photo,
	state
);

create table AZ_SexOffenders_aliases
(
	ID,
	Alias,
	age,
	state
);

create index AZ_SexOffenders_aliases__index
	on AZ_SexOffenders_aliases (ID, state);

create table AZ_SexOffenders_main
(
	ID,
	address TEXT,
	age,
	eyes,
	hair,
	height,
	level,
	name,
	race,
	scars_tattoos,
	sex,
	status,
	weight,
	state
);

create index AZ_SexOffenders_main__index
	on AZ_SexOffenders_main (ID, state);

create table AZ_SexOffenders_offenses
(
	ID,
	description,
	date_convicted,
	conviction_state,
	release_date,
	details,
	state
);

create index AZ_SexOffenders_offenses__index
	on AZ_SexOffenders_offenses (ID, state);

create table AZ_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index AZ_SexOffenders_photos__index
	on AZ_SexOffenders_photos (ID, state);

create table CA_ENC
(
	offense text,
	state text
);

create table CA_SexOffenders_alias
(
	ID,
	Alias,
	state
);

create index CA_SexOffenders_alias__index
	on CA_SexOffenders_alias (ID, state);

create index CA_SexOffenders_alias_index
	on CA_SexOffenders_alias (ID, Alias);

create table CA_SexOffenders_main
(
	Address,
	County,
	DateOfBirth,
	Ethnicity,
	EyeColor,
	HairColor,
	Height,
	ID,
	Name,
	Sex,
	Weight,
	state
);

create index CA_SexOffenders_main_ID_index
	on CA_SexOffenders_main (ID, Name);

create index CA_SexOffenders_main__index
	on CA_SexOffenders_main (ID, state);

create table CA_SexOffenders_offenses
(
	ID,
	OffenseCode,
	OffenseDescription text,
	YearOfLastConviction,
	YearOfLastRelease,
	state text
);

create index CA_SexOffenders_offenses__index
	on CA_SexOffenders_offenses (ID, state);

create table CA_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index CA_SexOffenders_photos__index
	on CA_SexOffenders_photos (ID, state);

create table CA_SexOffenders_risk
(
	ID,
	Tool,
	Score,
	state
);

create table CA_SexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create table CO_SexOffenders_addresses
(
	ID,
	Address,
	AddressExt,
	CityZip,
	DateReported,
	state
);

create index CO_SexOffenders_addresses__index
	on CO_SexOffenders_addresses (ID, state);

create table CO_SexOffenders_aliases
(
	ID,
	Alias,
	FirstName,
	MiddleName,
	LastName,
	Suffix,
	state
);

create index CO_SexOffenders_aliases__index
	on CO_SexOffenders_aliases (ID, state);

create table CO_SexOffenders_convictions
(
	ID,
	Statute,
	Description,
	ConvictionDate,
	state
);

create index CO_SexOffenders_convictions__index
	on CO_SexOffenders_convictions (ID, state);

create table CO_SexOffenders_dobs
(
	ID,
	OtherUsedDOB,
	OurOtherUsedDOB,
	state
);

create table CO_SexOffenders_main
(
	County,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Gender,
	Hair,
	Height,
	ID,
	LastName,
	MiddleName,
	Name,
	OffenderDesignation,
	Race,
	Suffix,
	Weight,
	state
);

create index CO_SexOffenders_main__index
	on CO_SexOffenders_main (ID, state);

create table CO_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index CO_SexOffenders_photos__index
	on CO_SexOffenders_photos (ID, state);

create table CO_SexOffenders_smts
(
	ID,
	ScarMarkTattoo,
	state
);

create index CO_SexOffenders_smts__index
	on CO_SexOffenders_smts (ID, state);

create table CTSexOffenders_addresses
(
	ID,
	Name,
	AddressType,
	Address,
	AddressExtended,
	CityStateZip,
	County,
	state
);

create index CTSexOffenders_addresses__index
	on CTSexOffenders_addresses (ID, state);

create table CTSexOffenders_aliases
(
	ID,
	Alias,
	dob,
	state
);

create index CTSexOffenders_aliases__index
	on CTSexOffenders_aliases (ID, state);

create table CTSexOffenders_comments
(
	ID,
	Comment,
	state
);

create index CTSexOffenders_comments__index
	on CTSexOffenders_comments (ID, state);

create table CTSexOffenders_main
(
	Age,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	LastVerificationDate,
	MiddleName,
	Name,
	PhotoName,
	Race,
	RegistrationNumber,
	Sex,
	Status,
	Suffix,
	Weight,
	state
);

create index CTSexOffenders_main__index
	on CTSexOffenders_main (ID, state);

create table CTSexOffenders_offenses
(
	ID,
	OffenseDescription,
	OffenseDateConvicted,
	OffenseState,
	OffenseDateReleased,
	OffenseDetails,
	state
);

create index CTSexOffenders_offenses__index
	on CTSexOffenders_offenses (ID, state);

create table CTSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index CTSexOffenders_photos__index
	on CTSexOffenders_photos (ID, state);

create table CTSexOffenders_probation_conditions
(
	ID,
	ProbationCondition,
	state
);

create table CTSexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index CTSexOffenders_smts__index
	on CTSexOffenders_smts (ID, state);

create table DE_SexOffenders_SMTs
(
	ID,
	ScarsMarksTattoos,
	Description,
	state
);

create index DE_SexOffenders_SMTs__index
	on DE_SexOffenders_SMTs (ID, state);

create table DE_SexOffenders_addresses
(
	ID,
	Type,
	Street,
	CityStateZip,
	County,
	DevelopmentEmployer,
	state
);

create index DE_SexOffenders_addresses__index
	on DE_SexOffenders_addresses (ID, state);

create table DE_SexOffenders_aliases
(
	ID,
	Alias,
	FirstName,
	MiddleName,
	LastName,
	Suffix,
	state
);

create index DE_SexOffenders_aliases__index
	on DE_SexOffenders_aliases (ID, state);

create table DE_SexOffenders_main
(
	Age,
	BirthDate,
	DOB,
	EyeColor,
	FirstName,
	Gender,
	HairColor,
	Height,
	ID,
	InPrison,
	LastName,
	MiddleName,
	Name,
	OutofState,
	Race,
	RegisteredSince,
	RepeatOffender,
	RiskLevel,
	SkinColor,
	Suffix,
	VerifiedOn,
	Weight,
	state
);

create index DE_SexOffenders_main__index
	on DE_SexOffenders_main (ID, state);

create table DE_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index DE_SexOffenders_photos__index
	on DE_SexOffenders_photos (ID, state);

create table FL_SexOffenders_addresses
(
	ID,
	Street,
	CityStateZip,
	County,
	Source,
	DateReceived,
	TypeOfAddress,
	state
);

create index FL_SexOffenders_addresses__index
	on FL_SexOffenders_addresses (ID, state);

create table FL_SexOffenders_aliases
(
	ID,
	Alias,
	DOB,
	state
);

create index FL_SexOffenders_aliases__index
	on FL_SexOffenders_aliases (ID, state);

create table FL_SexOffenders_main
(
	DOB,
	DOCNumber,
	DateOfBirth,
	DateOfPhoto,
	Designation,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	MiddleName,
	Name,
	Race,
	RegistrationStatus,
	Sex,
	Status,
	Suffix,
	Weight,
	state
);

create index FL_SexOffenders_main__index
	on FL_SexOffenders_main (ID, state);

create table FL_SexOffenders_offenses
(
	ID,
	AdjudicationDate,
	CrimeDescription,
	CourtCaseNumber,
	JurisdictionAndState,
	Adjudication,
	state
);

create index FL_SexOffenders_offenses__index
	on FL_SexOffenders_offenses (ID, state);

create table FL_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index FL_SexOffenders_photos__index
	on FL_SexOffenders_photos (ID, state);

create table FL_SexOffenders_smts
(
	ID,
	Type,
	Location,
	Number,
	state
);

create index FL_SexOffenders_smts__index
	on FL_SexOffenders_smts (ID, state);

create table FL_SexOffenders_vehicles
(
	ID,
	Make,
	Type,
	Color,
	Year,
	Body,
	TagNumber,
	state
);

create table FL_SexOffenders_vessels
(
	ID,
	Make,
	Type,
	Color,
	Motor,
	Hull,
	Year,
	RegNumber,
	state
);

create table FL_SexOffenders_victims
(
	ID,
	Gender,
	Minor,
	state
);

create table GA_SexOffenders_main
(
	Absconder,
	EyeColor,
	FirstName,
	HairColor,
	Height,
	ID,
	LastKnownAddress,
	LastName,
	Leveling,
	MiddleName,
	Predator,
	Race,
	RegistrationDate,
	ResidenceVerificationDate,
	Sex,
	Suffix,
	Weight,
	YearOfBirth,
	state
);

create index GA_SexOffenders_main__index
	on GA_SexOffenders_main (ID, state);

create table GA_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index GA_SexOffenders_photos__index
	on GA_SexOffenders_photos (ID, state);

create table HI_sex_offender_aliases
(
	ID,
	alias,
	year_of_birth,
	state
);

create table HI_sex_offender_appeals
(
	ID,
	appeal_status,
	appeal_date,
	state
);

create table HI_sex_offender_license
(
	ID,
	license_type,
	state
);

create table HI_sex_offender_main
(
	ID,
	covered_offender_status,
	eye_color,
	full_name,
	gender,
	hair_color,
	height,
	race,
	record_last_updated,
	website_last_updated,
	weight,
	year_of_birth,
	state
);

create table HI_sex_offender_offenses
(
	ID,
	offense,
	place_of_offense,
	disposition_date,
	name,
	state
);

create table HI_sex_offender_photos
(
	ID,
	PhotoFile,
	state
);

create table HI_sex_offender_smts
(
	ID,
	scars_marks_tattoos,
	state
);

create table HI_sex_offender_vehicles
(
	ID,
	vehicle_type,
	make,
	model,
	license_number,
	color,
	year,
	name,
	state
);

create table IA_sex_offender_aliases
(
	ID,
	last_name,
	first_name,
	middle_name,
	state
);

create table IA_sex_offender_convictions
(
	ID,
	conviction,
	conviction_date,
	registrant_age,
	iowa_code,
	vehicle_used,
	county,
	victim_gender,
	victim_age,
	state
);

create table IA_sex_offender_photos
(
	ID,
	PhotoFile,
	state
);

create table IA_sex_offender_smts
(
	ID,
	skin_marking,
	state
);

create table IDSexOffenders_addresses
(
	ID,
	AddressType,
	Name,
	Address,
	AddressDates,
	County,
	state
);

create index IDSexOffenders_addresses__index
	on IDSexOffenders_addresses (ID, state);

create table IDSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index IDSexOffenders_aliases__index
	on IDSexOffenders_aliases (ID, state);

create index IDSexOffenders_aliases__index_2
	on IDSexOffenders_aliases (ID, state);

create table IDSexOffenders_main
(
	Address,
	BirthPlace,
	CityStateZip,
	Comments,
	County,
	DOB,
	DateOfBirth,
	EyeColor,
	FirstName,
	HairColor,
	Height,
	ID,
	LastName,
	LastPhotoDate,
	LastProcess,
	LastProcessUpdate,
	LastRegistered,
	LastVerificationReceived,
	MiddleName,
	Name,
	PhotoName,
	Race,
	RegID,
	RegStatus,
	Sex,
	Suffix,
	Wanted,
	Weight,
	state
);

create index IDSexOffenders_main__index
	on IDSexOffenders_main (ID, state);

create index IDSexOffenders_main__index_2
	on IDSexOffenders_main (ID, state);

create table IDSexOffenders_offenses
(
	ID,
	Offense,
	Description,
	Date,
	PlaceOfConviction,
	state
);

create index IDSexOffenders_offenses__index
	on IDSexOffenders_offenses (ID, state);

create table IDSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index IDSexOffenders_photos__index
	on IDSexOffenders_photos (ID, state);

create table ILSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create table ILSexOffenders_crime_info
(
	ID,
	CrimeInformation,
	state
);

create table ILSexOffenders_crimes
(
	ID,
	Crime,
	state
);

create table ILSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create table INSexOffenders_addresses
(
	ID,
	Name,
	AddressType,
	Address,
	AddressExtension,
	City State Zip,
	County,
	state
);

create index INSexOffenders_addresses__index
	on INSexOffenders_addresses (ID, state);

create table INSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index INSexOffenders_aliases__index
	on INSexOffenders_aliases (ID, state);

create table INSexOffenders_main
(
	Age,
	Compliance,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	LastVerificationDate,
	Level,
	MiddleName,
	Name,
	PhotoName,
	Race,
	RegistrationNumber,
	Sex,
	Status,
	Suffix,
	Weight,
	YearOfBirth,
	state
);

create index INSexOffenders_main__index
	on INSexOffenders_main (ID, state);

create table INSexOffenders_offenses
(
	ID,
	OffenseDescription,
	DateConvicted,
	ConvictionState,
	DateReleased,
	OffenseDetails,
	StateEquivalent,
	CountyOfConviction,
	CaseNumber,
	Sentence,
	state
);

create index INSexOffenders_offenses__index
	on INSexOffenders_offenses (ID, state);

create table INSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index INSexOffenders_photos__index
	on INSexOffenders_photos (ID, state);

create table INSexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index INSexOffenders_smts__index
	on INSexOffenders_smts (ID, state);

create table INSexOffenders_warrants
(
	ID,
	Date,
	Alias,
	Warrant,
	state
);

create table LA_SexOffenders_addresses
(
	ID,
	Name,
	AddressType,
	Address,
	County,
	state
);

create index LA_SexOffenders_addresses__index
	on LA_SexOffenders_addresses (ID, state);

create table LA_SexOffenders_aliases
(
	ID,
	Alias,
	dob,
	state
);

create index LA_SexOffenders_aliases__index
	on LA_SexOffenders_aliases (ID, state);

create table LA_SexOffenders_comments
(
	ID,
	Comment,
	state
);

create index LA_SexOffenders_comments__index
	on LA_SexOffenders_comments (ID, state);

create table LA_SexOffenders_main
(
	Age,
	Compliance,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	Level,
	MiddleName,
	Name,
	Race,
	RegistrationNumber,
	Sex,
	Status,
	Suffix,
	Weight,
	state
);

create index LA_SexOffenders_main__index
	on LA_SexOffenders_main (ID, state);

create table LA_SexOffenders_offenses
(
	ID,
	OffenseDescription,
	DateConvicted,
	ConvictionState,
	DateReleased,
	OffenseDetails,
	state
);

create index LA_SexOffenders_offenses__index
	on LA_SexOffenders_offenses (ID, state);

create table LA_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index LA_SexOffenders_photos__index
	on LA_SexOffenders_photos (ID, state);

create table LA_SexOffenders_probation_conditions
(
	ID,
	ProbationCondition,
	state
);

create table LA_SexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index LA_SexOffenders_smts__index
	on LA_SexOffenders_smts (ID, state);

create table LA_SexOffenders_vehicles
(
	ID,
	Plate,
	Make,
	Model,
	Year,
	Color,
	state
);

create table LA_SexOffenders_warrants
(
	ID,
	WarrantInformation,
	state
);

create table MA_SexOffenders_addresses
(
	ID,
	Row,
	Address,
	Type,
	state
);

create index MA_SexOffenders_addresses__index
	on MA_SexOffenders_addresses (ID, state);

create table MA_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index MA_SexOffenders_aliases__index
	on MA_SexOffenders_aliases (ID, state);

create table MA_SexOffenders_main
(
	Age,
	Eye_Color,
	Hair_Color,
	Height,
	ID,
	Level,
	Name,
	Race,
	Sex,
	Weight,
	Year_Of_Birth,
	state
);

create index MA_SexOffenders_main__index
	on MA_SexOffenders_main (ID, state);

create table MA_SexOffenders_offenses
(
	ID,
	Row,
	Jurisdiction,
	Chapter,
	Conviction_Date,
	Number_of_Convictions,
	state
);

create index MA_SexOffenders_offenses__index
	on MA_SexOffenders_offenses (ID, state);

create table MA_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index MA_SexOffenders_photos__index
	on MA_SexOffenders_photos (ID, state);

create table MDSexOffenders_addresses
(
	ID,
	AddressType,
	Address,
	Street,
	city_state_zip,
	state
);

create index MDSexOffenders_addresses__index
	on MDSexOffenders_addresses (ID, state);

create table MDSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index MDSexOffenders_aliases__index
	on MDSexOffenders_aliases (ID, state);

create table MDSexOffenders_main
(
	CurrentAge,
	CurrentRegistrationDate,
	CustodySupervisionInformation,
	DateOfBirth,
	DateOfLastChangeofAddress,
	EyeColor,
	FirstName,
	HairColor,
	Height,
	ID,
	InformationContact,
	LastName,
	MiddleName,
	Name,
	PhotoName,
	Race,
	RegistrationStatus,
	Sex,
	SkinTone,
	Suffix,
	SupervisingAgency,
	Tier,
	Weight,
	state
);

create index MDSexOffenders_main__index
	on MDSexOffenders_main (ID, state);

create table MDSexOffenders_offenses
(
	ID,
	ConvictionDate,
	Location,
	RegAuthority,
	Charges,
	Description,
	state
);

create index MDSexOffenders_offenses__index
	on MDSexOffenders_offenses (ID, state);

create table MDSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index MDSexOffenders_photos__index
	on MDSexOffenders_photos (ID, state);

create table MDSexOffenders_vehicles
(
	ID,
	VehicleColor,
	VehicleMake,
	VehiclePlateNumber,
	state
);

create table ME_SexOffenders_addresses
(
	ID,
	TownOfDomicile,
	state
);

create index ME_SexOffenders_addresses__index
	on ME_SexOffenders_addresses (ID, state);

create table ME_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index ME_SexOffenders_aliases__index
	on ME_SexOffenders_aliases (ID, state);

create table ME_SexOffenders_employer_info
(
	ID,
	Employer,
	StreetAddress,
	ExtraAddressLine,
	CityStateZip,
	county,
	state
);

create table ME_SexOffenders_main
(
	DatabaseLastUpdatedOn,
	DateOfBirth,
	FirstName,
	ID,
	LastName,
	MiddleName,
	Name,
	RegistrantType,
	Suffix,
	VerificationPeriod,
	state
);

create index ME_SexOffenders_main__index
	on ME_SexOffenders_main (ID, state);

create table ME_SexOffenders_offenses
(
	ID,
	Statute,
	Offense,
	state
);

create index ME_SexOffenders_offenses__index
	on ME_SexOffenders_offenses (ID, state);

create table ME_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index ME_SexOffenders_photos__index
	on ME_SexOffenders_photos (ID, state);

create table MISexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index MISexOffenders_aliases__index
	on MISexOffenders_aliases (ID, state);

create table MISexOffenders_covictions
(
	ID,
	OffenseDescription,
	DateConvicted,
	ConvictionState,
	DateReleased,
	OffenseDetails,
	CountyOfConviction,
	CaseNumber,
	Sentence,
	state
);

create index MISexOffenders_covictions__index
	on MISexOffenders_covictions (ID, state);

create table MISexOffenders_main
(
	Age,
	Comments,
	Compliance,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	MiddleName,
	Name,
	PhotoName,
	Race,
	RegistrationNumber,
	Sex,
	Suffix,
	Weight,
	state
);

create index MISexOffenders_main__index
	on MISexOffenders_main (ID, state);

create table MISexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index MISexOffenders_photos__index
	on MISexOffenders_photos (ID, state);

create table MISexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index MISexOffenders_smts__index
	on MISexOffenders_smts (ID, state);

create table MISexOffenders_vehicles
(
	ID,
	Plate,
	Make,
	Model,
	Year,
	Color,
	state
);

create table MN_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index MN_SexOffenders_aliases__index
	on MN_SexOffenders_aliases (ID, state);

create table MN_SexOffenders_main
(
	AddressCounty,
	Build,
	DOB,
	DateOfBirth,
	EyeColor,
	HairColor,
	Height,
	ID,
	LawEnforcementAgency,
	LawEnforcementAgencyPhone,
	Name,
	OID,
	OffenseInformation,
	OffenseStatutes,
	RaceEthnicity,
	RegisteredAddress,
	RegisteredCityStateZip,
	ReleaseDate,
	SkinTone,
	SupervisingAgent,
	SupervisionComments,
	Weight,
	state
);

create index MN_SexOffenders_main__index
	on MN_SexOffenders_main (ID, state);

create table MN_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index MN_SexOffenders_photos__index
	on MN_SexOffenders_photos (ID, state);

create table MOSexOffenders_SMTs
(
	ID,
	ScarsMarksTattoos,
	state
);

create index MOSexOffenders_smts__index
	on MOSexOffenders_SMTs (ID, state);

create table MOSexOffenders_addresses
(
	ID,
	AddressType,
	Street,
	CityState,
	Zip,
	County,
	Name,
	state
);

create index MOSexOffenders_addresses__index
	on MOSexOffenders_addresses (ID, state);

create table MOSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index MOSexOffenders_aliases__index
	on MOSexOffenders_aliases (ID, state);

create table MOSexOffenders_convictions
(
	ID,
	ConvictionPleaDescription,
	ConvictionPleaCityState,
	ConvictionPleaCounty,
	ConvictionPleaDate,
	state
);

create index MOSexOffenders_convictions__index
	on MOSexOffenders_convictions (ID, state);

create table MOSexOffenders_dob_aliases
(
	ID,
	DOBAlias,
	state
);

create table MOSexOffenders_main
(
	Compliance,
	DateOfBirth,
	EyeColor,
	Gender,
	HairColor,
	Height,
	ID,
	Name,
	Race,
	Weight,
	state
);

create index MOSexOffenders_main__index
	on MOSexOffenders_main (ID, state);

create table MOSexOffenders_offenses
(
	ID,
	OffenseDescription,
	OffenseCityState,
	OffenseCounty,
	OffenseDate,
	state
);

create index MOSexOffenders_offenses__index
	on MOSexOffenders_offenses (ID, state);

create table MOSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index MOSexOffenders_photos__index
	on MOSexOffenders_photos (ID, state);

create table MOSexOffenders_release
(
	ID,
	Type,
	ReleaseDate,
	state
);

create table MOSexOffenders_vehicles
(
	ID,
	Year,
	Make,
	Model,
	Color,
	LicensePlate,
	LicenseState,
	state
);

create table MOSexOffenders_victim_info
(
	ID,
	OffenseDescription,
	VictimGender,
	VictimAge,
	state
);

create table MS_SexOffenders_addresses
(
	ID,
	AddressType,
	Address,
	CityStateZip,
	County,
	state
);

create table MS_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create table MS_SexOffenders_offenses
(
	ID,
	ConvictionDate,
	Location,
	OffenseCode,
	Offense,
	state
);

create table MS_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create table MS_SexOffenders_smts
(
	ID,
	ScarTattoo,
	Location,
	Description,
	state
);

create table MS_SexOffenders_vehicles
(
	ID,
	LicenseNumber,
	Make,
	Model,
	Color,
	state
);

create table NCSexOffenders_alias
(
	ID,
	Alias,
	state
);

create index NCSexOffenders_alias__index
	on NCSexOffenders_alias (ID, state);

create table NCSexOffenders_main
(
	AddressLine1,
	AddressLine2,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastAddressVerified,
	LastName,
	LeftPhotoName,
	MiddleName,
	MinimumRegistrationPeriod,
	Name,
	PhotoName,
	Race,
	RegistrationDate,
	RegistrationStatus,
	RegistrationType,
	RightPhotoName,
	SRN,
	Sex,
	Suffix,
	Violations,
	Weight,
	state
);

create index NCSexOffenders_main__index
	on NCSexOffenders_main (ID, state);

create table NCSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index NCSexOffenders_photos__index
	on NCSexOffenders_photos (ID, state);

create table NCSexOffenders_smts
(
	ID,
	ScarMarkTattoo,
	state
);

create index NCSexOffenders_smts__index
	on NCSexOffenders_smts (ID, state);

create table NDSexOffenders_alias
(
	ID,
	Alias,
	state
);

create index NDSexOffenders_alias__index
	on NDSexOffenders_alias (ID, state);

create table NDSexOffenders_convictions
(
	ID,
	Offense,
	ConvictionDate,
	Court,
	Dispostion,
	state
);

create index NDSexOffenders_convictions__index
	on NDSexOffenders_convictions (ID, state);

create table NDSexOffenders_main
(
	AddressName,
	CityStateZip,
	County,
	DOB,
	DateOfBirth,
	ExpirationDate,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	LastUpdatedDate,
	MiddleName,
	Name,
	POBoxAddress,
	PhotoDate,
	PhotoName,
	Race,
	RiskLevel,
	Sex,
	Skin,
	SpecialDirections,
	Status,
	StreetAddress,
	Suffix,
	SupervisionInformation,
	Weight,
	state
);

create index NDSexOffenders_main__index
	on NDSexOffenders_main (ID, state);

create table NDSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index NDSexOffenders_photos__index
	on NDSexOffenders_photos (ID, state);

create table NE_SexOffenders_addresses
(
	ID,
	DateReported,
	AddressType,
	Address,
	AdditionalInfo,
	CityStateZip,
	County,
	state
);

create index NE_SexOffenders_addresses__index
	on NE_SexOffenders_addresses (ID, state);

create table NE_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index NE_SexOffenders_aliases__index
	on NE_SexOffenders_aliases (ID, state);

create table NE_SexOffenders_convictions
(
	ConvictionDate,
	Court,
	Crime,
	ID,
	Jurisdiction,
	PlaceofCrime,
	StatuteNumber,
	VictimofCrime,
	state
);

create index NE_SexOffenders_convictions__index
	on NE_SexOffenders_convictions (ID, state);

create table NE_SexOffenders_main
(
	DOB,
	Eyes,
	Hair,
	Height,
	ID,
	Name,
	Race,
	RegistrationDuration,
	Sex,
	Weight,
	state
);

create index NE_SexOffenders_main__index
	on NE_SexOffenders_main (ID, state);

create table NE_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index NE_SexOffenders_photos__index
	on NE_SexOffenders_photos (ID, state);

create table NE_SexOffenders_schools
(
	ID,
	SchoolName,
	SchoolAddress,
	SchoolCityStateZip,
	SchoolCounty,
	state
);

create table NE_SexOffenders_vehicles
(
	ID,
	Vehicle,
	AdditionalInfo,
	state
);

create table NHSexOffenders_addresses
(
	ID,
	AddressType,
	Address,
	County,
	street,
	CityStateZip,
	state
);

create index NHSexOffenders_addresses__index
	on NHSexOffenders_addresses (ID, state);

create table NHSexOffenders_alias
(
	ID,
	Alias,
	dob,
	state
);

create index NHSexOffenders_alias__index
	on NHSexOffenders_alias (ID, state);

create table NHSexOffenders_crimes
(
	ID,
	OtherCrime,
	ConvictionDate,
	state
);

create table NHSexOffenders_main
(
	Age,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	MiddleName,
	Name,
	ParoleProbationOrSupervisedDate,
	PhotoDate,
	PhotoName,
	Race,
	RequirementDescription,
	Sex,
	StatusOfParoleProbationOrSupervisedRelease,
	Suffix,
	Weight,
	state
);

create index NHSexOffenders_main__index
	on NHSexOffenders_main (ID, state);

create table NHSexOffenders_offenses
(
	ID,
	Court,
	AdjudicationDate,
	Offense,
	state
);

create index NHSexOffenders_offenses__index
	on NHSexOffenders_offenses (ID, state);

create table NHSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create table NHSexOffenders_smts
(
	ID,
	Category,
	Description,
	state
);

create index NHSexOffenders_smts__index
	on NHSexOffenders_smts (ID, state);

create table NJSexOffenders_addresses
(
	id,
	name,
	address,
	state
);

create index NJSexOffenders_addresses__index
	on NJSexOffenders_addresses (id, state);

create table NJSexOffenders_aliases
(
	id,
	alias,
	age,
	state
);

create index NJSexOffenders_aliases__index
	on NJSexOffenders_aliases (id, state);

create table NJSexOffenders_convictions
(
	id,
	offense_description,
	convicted_date,
	conviction_state,
	released_date,
	offense_details,
	county_of_conviction,
	state
);

create index NJSexOffenders_convictions__index
	on NJSexOffenders_convictions (id, state);

create table NJSexOffenders_main
(
	age,
	comments,
	eyes,
	hair,
	height,
	id,
	level,
	name,
	race,
	registration_number,
	sex,
	weight,
	state
);

create index NJSexOffenders_main__index
	on NJSexOffenders_main (id, state);

create table NJSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index NJSexOffenders_photos__index
	on NJSexOffenders_photos (ID, state);

create table NJSexOffenders_smts
(
	id,
	scars_marks_tattoos,
	state
);

create index NJSexOffenders_smts__index
	on NJSexOffenders_smts (id, state);

create table NJSexOffenders_vehicles
(
	id,
	plate,
	make,
	model,
	year,
	color,
	state
);

create table NMSexOffenders_addresses
(
	ID,
	AddressType,
	AddressLine1,
	AddressLine2,
	AddressLine3,
	AddressLine4,
	street,
	citystatezip,
	name,
	state
);

create index NMSexOffenders_addresses__index
	on NMSexOffenders_addresses (ID, state);

create table NMSexOffenders_aliases
(
	ID,
	Alias,
	age,
	state
);

create index NMSexOffenders_aliases__index
	on NMSexOffenders_aliases (ID, state);

create table NMSexOffenders_main
(
	Age,
	Compliance,
	EyeColor,
	FirstName,
	HairColor,
	Height,
	ID,
	LastName,
	MiddleName,
	Name,
	Race,
	RegistrationNumber,
	Sex,
	Suffix,
	Weight,
	YearOfBirth,
	state
);

create index NMSexOffenders_main__index
	on NMSexOffenders_main (ID, state);

create table NMSexOffenders_offenses
(
	ID,
	OffenseDescription,
	DateConvicted,
	ConvictionState,
	DateReleased,
	OffenseDetails,
	Counts,
	CountyOfConviction,
	state
);

create index NMSexOffenders_offenses__index
	on NMSexOffenders_offenses (ID, state);

create table NMSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index NMSexOffenders_photos__index
	on NMSexOffenders_photos (ID, state);

create table NMSexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index NMSexOffenders_smts__index
	on NMSexOffenders_smts (ID, state);

create table NV_SexOffenders_addresses
(
	ID,
	AddressType,
	Address,
	CityStateZip,
	County,
	state
);

create index NV_SexOffenders_addresses__index
	on NV_SexOffenders_addresses (ID, state);

create table NV_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index NV_SexOffenders_aliases__index
	on NV_SexOffenders_aliases (ID, state);

create table NV_SexOffenders_main
(
	Compliance,
	Ethnicity,
	EyeColor,
	FirstName,
	HairColor,
	Height,
	ID,
	LastName,
	MiddleName,
	Name,
	Race,
	Sex,
	TierLevel,
	Weight,
	YearofBirth,
	state
);

create index NV_SexOffenders_main__index
	on NV_SexOffenders_main (ID, state);

create table NV_SexOffenders_offenses
(
	ID,
	ConvictionDate,
	ConvictionDescription,
	CourtName,
	ConvictionName,
	OffenseLocation,
	Institution,
	state
);

create index NV_SexOffenders_offenses__index
	on NV_SexOffenders_offenses (ID, state);

create table NV_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index NV_SexOffenders_photos__index
	on NV_SexOffenders_photos (ID, state);

create table NV_SexOffenders_smts
(
	ID,
	ScarTattoo,
	Location,
	Description,
	state
);

create index NV_SexOffenders_smts__index
	on NV_SexOffenders_smts (ID, state);

create table NV_SexOffenders_vehicles
(
	ID,
	LicenseNumber,
	Type,
	Year,
	Make,
	Model,
	Style,
	Color,
	state
);

create table NYSexOffenders_addresses
(
	ID,
	Type,
	AddressLine1,
	AddressLine2,
	AddressLine3,
	AddressLine4,
	AddressLine5,
	Street,
	CityStateZip,
	CountyCountry,
	Name,
	InmateID,
	state
);

create index NYSexOffenders_addresses__index
	on NYSexOffenders_addresses (ID, state);

create table NYSexOffenders_aliases
(
	ID,
	alias,
	dob,
	state
);

create index NYSexOffenders_aliases__index
	on NYSexOffenders_aliases (ID, state);

create table NYSexOffenders_current_conviction
(
	ID,
	our_conviction_id,
	dateofcrime,
	dateconvicted,
	victimsexage,
	arrestingagency,
	offensedescriptions,
	relationshiptovictim,
	weaponused,
	forceused,
	computerused,
	pornographyinvolved,
	sentence,
	state
);

create index NYSexOffenders_current_conviction__index
	on NYSexOffenders_current_conviction (ID, state);

create table NYSexOffenders_current_conviction_charges
(
	ID,
	our_conviction_id,
	Title,
	Section,
	Subsection,
	Class,
	Category,
	Counts,
	Description,
	state
);

create table NYSexOffenders_main
(
	CorrLens,
	DOB,
	Designation,
	Ethnicity,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	InmateID,
	LastName,
	LawEnforcementAgencyHavingJurisdiction,
	MaximumExpirationDatePostReleaseSupervisionDateOfSentence,
	MiddleName,
	Name,
	OffenderID,
	PhotoDate,
	Race,
	RiskLevel,
	Sex,
	SpecialConditions,
	SupervisingAgency,
	Weight,
	state
);

create index NYSexOffenders_main__index
	on NYSexOffenders_main (ID, state);

create table NYSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index NYSexOffenders_photos__index
	on NYSexOffenders_photos (ID, state);

create table NYSexOffenders_previous_conviction
(
	ID,
	our_conviction_id,
	dateofcrime,
	dateconvicted,
	victimsexage,
	arrestingagency,
	offensedescriptions,
	relationshiptovictim,
	weaponused,
	forceused,
	computerused,
	pornographyinvolved,
	sentence,
	state
);

create table NYSexOffenders_previous_conviction_charges
(
	ID,
	our_conviction_id,
	Title,
	Section,
	Subsection,
	Class,
	Category,
	Counts,
	Description,
	state
);

create table NYSexOffenders_smts
(
	ID,
	ScarMarkTattoo,
	state
);

create index NYSexOffenders_smts__index
	on NYSexOffenders_smts (ID, state);

create table OHSexOffenders_addresses
(
	ID,
	Name,
	AddressType,
	Address,
	AddressExtended,
	CityStateZip,
	County,
	state
);

create index OHSexOffenders_addresses__index
	on OHSexOffenders_addresses (ID, state);

create table OHSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index OHSexOffenders_aliases__index
	on OHSexOffenders_aliases (ID, state);

create table OHSexOffenders_convictions
(
	ID,
	OffenseDescription,
	DateConvicted,
	ConvictionState,
	DateReleased,
	OffenseDetails,
	StateEquivalent,
	state
);

create index OHSexOffenders_convictions__index
	on OHSexOffenders_convictions (ID, state);

create table OHSexOffenders_main
(
	Age,
	Comments,
	Compliance,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	Level,
	MiddleName,
	Name,
	Race,
	RegistrationNumber,
	Sex,
	Suffix,
	Weight,
	state
);

create index OHSexOffenders_main__index
	on OHSexOffenders_main (ID, state);

create table OHSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index OHSexOffenders_photos__index
	on OHSexOffenders_photos (ID, state);

create table OHSexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index OHSexOffenders_smts__index
	on OHSexOffenders_smts (ID, state);

create table OHSexOffenders_vehicles
(
	ID,
	Plate,
	Make,
	Model,
	Year,
	Color,
	state
);

create table OHSexOffenders_victim_info
(
	ID,
	VictimInfoCategory,
	Description,
	state
);

create table ORSexOffenders_employments
(
	ID,
	Title,
	Place,
	state
);

create table ORSexOffenders_main
(
	Age,
	DOB,
	Eyes,
	Hair,
	Height,
	ID,
	LastRegistrationDate,
	Name,
	Residence Address,
	Sex,
	Status,
	Weight,
	state
);

create index ORSexOffenders_main__index
	on ORSexOffenders_main (ID, state);

create table ORSexOffenders_offenses
(
	ID,
	ConvictionDate,
	OffenseName,
	state
);

create index ORSexOffenders_offenses__index
	on ORSexOffenders_offenses (ID, state);

create table ORSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index ORSexOffenders_photos__index
	on ORSexOffenders_photos (ID, state);

create table ORSexOffenders_smts
(
	ID,
	SMT,
	state
);

create index ORSexOffenders_smts__index
	on ORSexOffenders_smts (ID, state);

create table PA_SexOffenders_SMTs
(
	ID,
	Type,
	Location,
	Description,
	state
);

create index PA_SexOffenders_smts__index
	on PA_SexOffenders_SMTs (ID, state);

create table PA_SexOffenders_addresses
(
	ID,
	AddressType,
	Address,
	Municipality,
	County,
	GeneralWorkArea,
	state
);

create index PA_SexOffenders_addresses__index
	on PA_SexOffenders_addresses (ID, state);

create table PA_SexOffenders_aliases
(
	ID,
	Aliases,
	state
);

create index PA_SexOffenders_aliases__index
	on PA_SexOffenders_aliases (ID, state);

create table PA_SexOffenders_main
(
	BirthYear,
	EyeColor,
	Gender,
	HairColor,
	Height,
	ID,
	LastUpdated,
	Name,
	OffenderType,
	PrimaryOffense,
	Race,
	RegistrationStart,
	Weight,
	state
);

create index PA_SexOffenders_main__index
	on PA_SexOffenders_main (ID, state);

create table PA_SexOffenders_offenses
(
	ID,
	Offense,
	OffenseDate,
	ConvictionDate,
	IsVictimMinor,
	state
);

create index PA_SexOffenders_offenses__index
	on PA_SexOffenders_offenses (ID, state);

create table PA_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index PA_SexOffenders_photos__index
	on PA_SexOffenders_photos (ID, state);

create table Photos
(
	id INTEGER,
	name TEXT,
	size Integer,
	data Blob,
	state TEXT
);

create index Photos_idx
	on Photos (id, state);

create table RISexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create table SCSexOffenders_addresses
(
	ID,
	Type,
	AddressLine1,
	AddressLine2,
	County,
	state
);

create index SCSexOffenders_addresses__index
	on SCSexOffenders_addresses (ID, state);

create table SCSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index SCSexOffenders_aliases__index
	on SCSexOffenders_aliases (ID, state);

create table SCSexOffenders_boats
(
	ID,
	Registration Number,
	Make,
	Model,
	Type,
	Port,
	Name,
	Color,
	state
);

create table SCSexOffenders_main
(
	DateOfBirth,
	Ethnicity,
	Eyes,
	FirstName,
	Gender,
	Hair,
	Height,
	ID,
	LastName,
	MiddleName,
	OffenderType,
	Race,
	Weight,
	state
);

create index SCSexOffenders_main__index
	on SCSexOffenders_main (ID, state);

create table SCSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index SCSexOffenders_photos__index
	on SCSexOffenders_photos (ID, state);

create table SCSexOffenders_smts
(
	ID,
	Type,
	Location,
	Description,
	state
);

create index SCSexOffenders_smts__index
	on SCSexOffenders_smts (ID, state);

create table SCSexOffenders_vehicles
(
	ID,
	Plate,
	Make,
	Model,
	Color,
	state
);

create table SDSexOffenders_alias
(
	ID,
	Alias,
	state
);

create index SDSexOffenders_alias__index
	on SDSexOffenders_alias (ID, state);

create table SDSexOffenders_convictions
(
	ID,
	CrimesConvicted,
	CrimeDescription,
	CountyOfConviction,
	StateOfConviction,
	DateOfConviction,
	DateOfCommission,
	state
);

create index SDSexOffenders_convictions__index
	on SDSexOffenders_convictions (ID, state);

create table SDSexOffenders_main
(
	Address,
	AgencyAddress,
	AgencyCityStateZip,
	AgencyFax,
	AgencyPO,
	AgencyPhone,
	CityStateZip,
	CommunitySafetyZoneRestrictions,
	County,
	CustodyStatus,
	DOB,
	DateOfBirth,
	EyeColor,
	Gender,
	HairColor,
	Height,
	ID,
	Incarceration,
	Name,
	OffenderStatus,
	PhotoDate,
	PhotoName,
	Race,
	RegisteringAgency,
	SecondaryAddress,
	SecondaryCityStateZip,
	SecondaryCounty,
	Weight,
	state
);

create index SDSexOffenders_main__index
	on SDSexOffenders_main (ID, state);

create table SDSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index SDSexOffenders_photos__index
	on SDSexOffenders_photos (ID, state);

create table SexOffender
(
	id Integer,
	name,
	age,
	dateOfBirth,
	state,
	aliases,
	offenses,
	addresses,
	photos,
	personalDetails
);

create unique index SexOffender_idx
	on SexOffender (id, state);

create table Sex_Offender_T1
(
	ID,
	name,
	dateOfBirth,
	state,
	aliases,
	addresses,
	offenses,
	scarsTattoos,
	photos,
	eyes text,
	hair text,
	height text,
	weight text,
	race text,
	sex text
);

create index Sex_Offender_T1__index
	on Sex_Offender_T1 (ID, state);

create table States
(
	name text,
	abbrev text
);

create table UT_SexOffenders_addresses
(
	ID,
	Type,
	Address,
	state
);

create index UT_SexOffenders_addresses__index
	on UT_SexOffenders_addresses (ID, state);

create table UT_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index UT_SexOffenders_aliases__index
	on UT_SexOffenders_aliases (ID, state);

create table UT_SexOffenders_main
(
	Address,
	Age,
	DOB,
	Eyes,
	Hair,
	Height,
	ID,
	Name,
	Race,
	RegistrationNum,
	Sex,
	Status,
	Weight,
	state
);

create index UT_SexOffenders_main__index
	on UT_SexOffenders_main (ID, state);

create table UT_SexOffenders_offenses
(
	ID,
	OffenseDescription,
	DateConvicted,
	ConvictionState,
	DateReleased,
	OffenseDetails,
	state
);

create index UT_SexOffenders_offenses__index
	on UT_SexOffenders_offenses (ID, state);

create table UT_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index UT_SexOffenders_photos__index
	on UT_SexOffenders_photos (ID, state);

create table UT_SexOffenders_pro_licenses
(
	ID,
	ProfessionalLicense,
	state
);

create table UT_SexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index UT_SexOffenders_smts__index
	on UT_SexOffenders_smts (ID, state);

create table UT_SexOffenders_vehicles
(
	ID,
	Plate,
	Make,
	Model,
	Year,
	Color,
	state
);

create table VASexOffenders_addresses
(
	ID,
	type,
	address,
	lastname,
	middlename,
	firstname,
	suffix,
	placeofwork,
	state
);

create index VASexOffenders_addresses__index
	on VASexOffenders_addresses (ID, state);

create table VASexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index VASexOffenders_aliases__index
	on VASexOffenders_aliases (ID, state);

create table VASexOffenders_main
(
	Age,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	InitialRegistration,
	LastName,
	MiddleName,
	Name,
	PhotoDate,
	PhotoName,
	Race,
	RegistrationNumber,
	RegistrationRenewed,
	Sex,
	Status,
	Suffix,
	Violent,
	Weight,
	state
);

create index VASexOffenders_main__index
	on VASexOffenders_main (ID, state);

create table VASexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index VASexOffenders_photos__index
	on VASexOffenders_photos (ID, state);

create table VTSexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index VTSexOffenders_aliases__index
	on VTSexOffenders_aliases (ID, state);

create table VTSexOffenders_comments
(
	ID,
	Comment,
	state
);

create index VTSexOffenders_comments__index
	on VTSexOffenders_comments (ID, state);

create table VTSexOffenders_main
(
	Address,
	Age,
	Compliance,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	Level,
	MiddleName,
	Name,
	PhotoName,
	Race,
	RegistrationNumber,
	Sex,
	Suffix,
	Weight,
	state
);

create index VTSexOffenders_main__index
	on VTSexOffenders_main (ID, state);

create table VTSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index VTSexOffenders_photos__index
	on VTSexOffenders_photos (ID, state);

create table VTSexOffenders_probation_conditions
(
	ID,
	ProbationInformation,
	state
);

create table VTSexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index VTSexOffenders_smts__index
	on VTSexOffenders_smts (ID, state);

create table VTSexOffenders_warrant
(
	ID,
	Warrant,
	state
);

create table WA_SexOffenders_aliases
(
	ID,
	Alias,
	age,
	state
);

create index WA_SexOffenders_aliases__index
	on WA_SexOffenders_aliases (ID, state);

create table WA_SexOffenders_comments
(
	ID,
	Comment,
	state
);

create index WA_SexOffenders_comments__index
	on WA_SexOffenders_comments (ID, state);

create table WA_SexOffenders_main
(
	Address,
	Age,
	Compliance,
	County,
	Eyes,
	FirstName,
	Hair,
	Height,
	ID,
	LastName,
	Level,
	MiddleName,
	Name,
	PhotoName,
	Race,
	RegistrationNumber,
	Sex,
	Suffix,
	Weight,
	state
);

create index WA_SexOffenders_main__index
	on WA_SexOffenders_main (ID, state);

create table WA_SexOffenders_offenses
(
	ID,
	OffenseDescription,
	DateConvicted,
	ConvictionState,
	DateReleased,
	OffenseDetails,
	StateEquivalent,
	CountyOfConviction,
	CaseNumber,
	Sentence,
	state
);

create index WA_SexOffenders_offenses__index
	on WA_SexOffenders_offenses (ID, state);

create table WA_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index WA_SexOffenders_photos__index
	on WA_SexOffenders_photos (ID, state);

create table WA_SexOffenders_probation_conditions
(
	ID,
	ProbationCondition,
	state
);

create table WA_SexOffenders_smts
(
	ID,
	ScarsMarksTattoos,
	state
);

create index WA_SexOffenders_smts__index
	on WA_SexOffenders_smts (ID, state);

create table WI_SexOffenders_addresses
(
	ID,
	Addressline1,
	Addressline2,
	Addressline3,
	Addressline4,
	street,
	CityStateZip,
	County,
	ReportDate,
	VerificationDate,
	Name,
	state
);

create index WI_SexOffenders_addresses__index
	on WI_SexOffenders_addresses (ID, state);

create table WI_SexOffenders_aliases
(
	ID,
	Alias,
	state
);

create index WI_SexOffenders_aliases__index
	on WI_SexOffenders_aliases (ID, state);

create table WI_SexOffenders_main
(
	Age,
	ComplianceStatus,
	DOC,
	Ethnicity,
	EyeColor,
	Gender,
	HairColor,
	Height,
	ID,
	IncarcerationSupervisionStatus,
	Name,
	PhotoDate,
	Race,
	RegistrantRespondedToLatestUSPSmail,
	RegistrationBegin,
	RegistrationEnd,
	Weight,
	state
);

create index WI_SexOffenders_main__index
	on WI_SexOffenders_main (ID, state);

create table WI_SexOffenders_offenses
(
	ID,
	ConvictionDate,
	ConvicitonCounty,
	CovictionState,
	CaseNumber,
	OffenseCode,
	Offense,
	state
);

create index WI_SexOffenders_offenses__index
	on WI_SexOffenders_offenses (ID, state);

create table WI_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index WI_SexOffenders_photos__index
	on WI_SexOffenders_photos (ID, state);

create table WV_SexOffenders_main
(
	Age,
	DOB,
	DateOfBirth,
	Eyes,
	FirstName,
	Gender,
	Hair,
	Height,
	ID,
	LastModified,
	LastName,
	MiddleName,
	Name,
	PhotoName,
	Race,
	Suffix,
	Weight,
	state
);

create table WV_SexOffenders_offenses
(
	ID,
	ConvcitionDate,
	VictimRelation,
	VictimSex,
	VictimAge,
	Offense,
	state
);

create table WV_SexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create table WYSexOffenders_addresses
(
	ID,
	Name,
	AddressType,
	Address,
	state
);

create index WYSexOffenders_addresses__index
	on WYSexOffenders_addresses (ID, state);

create table WYSexOffenders_aliases
(
	ID,
	Alias,
	DOB,
	state
);

create index WYSexOffenders_aliases__index
	on WYSexOffenders_aliases (ID, state);

create table WYSexOffenders_main
(
	DOB,
	DateOfBirth,
	EyeColor,
	HairColor,
	Height,
	ID,
	LastVerificationDate,
	LifetimeRegistration,
	Name,
	Race,
	RegID,
	RegistrationStartDate,
	Sex,
	Status,
	Weight,
	state
);

create index WYSexOffenders_main__index
	on WYSexOffenders_main (ID, state);

create table WYSexOffenders_offenses
(
	ID,
	description,
	date_convicted,
	conviction_state,
	release_date,
	details,
	state_equivalent,
	state
);

create index WYSexOffenders_offenses__index
	on WYSexOffenders_offenses (ID, state);

create table WYSexOffenders_photos
(
	ID,
	PhotoFile,
	state
);

create index WYSexOffenders_photos__index
	on WYSexOffenders_photos (ID, state);

create table WYSexOffenders_smts
(
	ID,
	scars_marks_tattoos,
	state
);

create index WYSexOffenders_smts__index
	on WYSexOffenders_smts (ID, state);

create table WYSexOffenders_vehicles
(
	ID,
	Plate,
	Make,
	Model,
	Year,
	Color,
	state
);

create table al_sex_offenders
(
	r_Name,
	r_Image,
	r_Status,
	r_Race,
	r_Sex,
	r_Birth_Date,
	r_Age,
	r_Height,
	r_Weight,
	r_Hair,
	r_Eyes,
	r_Release_Date,
	r_Address_is_temporary,
	r_Address_Verified_Date,
	r_Registration_date,
	r_Arrest_Date,
	r_Conviction_Date,
	r_Sex_Crime,
	r_Crime_Location,
	r_Crime_Country,
	r_Description,
	r_Ucr_Code,
	r_Vehicle,
	r_Vehicle_Tag,
	r_Vehicle_Color,
	r_Occupation,
	r_Victim_Gender,
	r_Victim_Age,
	r_Home_Address_1,
	r_Home_City_State_Zip,
	r_Home_County,
	r_Work_Address_1,
	r_Work_City_State_Zip,
	r_Work_County,
	r_Scars_Marks_Tattoos,
	r_Aliases,
	r_Schools,
	end_unformatted,
	r_Birth_Date_YMD,
	r_Release_Date_YMD,
	r_Conviction_Date_YMD,
	r_Registration_Date_YMD,
	r_Arrest_Date_YMD,
	r_Address_Verified_Date_YMD,
	r_Home_City,
	r_Home_State,
	r_Home_Zip,
	r_Work_City,
	r_Work_State,
	r_Work_Zip,
	r_Last_Name,
	r_Other_Names,
	endraw,
	name,
	case_num,
	zip,
	state
);

create table ks_sex_offenders
(
	r_First_Name,
	r_Last_Name,
	r_Middle_Name,
	r_Name_Suffix,
	r_Gender,
	r_Race,
	r_Ethnicity,
	r_Birth_Date,
	r_Height,
	r_Weight,
	r_Hair_Color,
	r_Eye_Color,
	r_Address_1,
	r_Address_2,
	r_City_State_Zip,
	r_Address_Date,
	r_Aliases,
	r_Image,
	r_Scars,
	r_Tattoos,
	r_Vehicle_Year,
	r_Vehicle_Make,
	r_Vehicle_Model,
	r_Vehicle_Style,
	r_Vehicle_Color,
	r_Vehicle_Plate_Number,
	r_Vehicle_State,
	r_Residential_Address_1,
	r_Residential_Address_2,
	r_Residential_City_State_Zip,
	r_School_Address_1,
	r_School_Address_2,
	r_School_City_State_Zip,
	r_Employer_Address_1,
	r_Employer_Address_2,
	r_Employer_City_State_Zip,
	r_Offense_Date_1,
	r_Conviction_Date_1,
	r_Statute_1,
	r_Offense_1,
	r_Offense_City_1,
	r_Offense_County_1,
	r_Offense_State_1,
	r_Victim_Age_1,
	r_Victim_Sex_1,
	r_Offense_Date_2,
	r_Conviction_Date_2,
	r_Statute_2,
	r_Offense_2,
	r_Offense_City_2,
	r_Offense_County_2,
	r_Offense_State_2,
	r_Victim_Age_2,
	r_Victim_Sex_2,
	r_Offense_Date_3,
	r_Conviction_Date_3,
	r_Statute_3,
	r_Offense_3,
	r_Offense_City_3,
	r_Offense_County_3,
	r_Offense_State_3,
	r_Victim_Age_3,
	r_Victim_Sex_3,
	end_unformatted,
	r_Birth_Date_YMD,
	r_City,
	r_State,
	r_Zip,
	endraw,
	name,
	case_num,
	zip,
	state
);

create table ky_sex_offenders_addresses
(
	id,
	type,
	address,
	county,
	state
);

create index ky_Sex_Offenders_addresses__index
	on ky_sex_offenders_addresses (id, state);

create table ky_sex_offenders_aliases
(
	id,
	alias_name,
	birth_date,
	state
);

create index ky_Sex_Offenders_aliases__index
	on ky_sex_offenders_aliases (id, state);

create table ky_sex_offenders_main
(
	id,
	name,
	photo_date,
	remarks,
	sex_offender_number,
	birth_date,
	gender,
	registry_type,
	registry_status,
	supervised_release,
	race,
	height,
	weight,
	hair_color,
	eye_color,
	biometric,
	state
);

create index ky_Sex_Offenders_main__index
	on ky_sex_offenders_main (id, state);

create table ky_sex_offenders_photos
(
	ID,
	PhotoFile,
	state
);

create index ky_Sex_Offenders_photos__index
	on ky_sex_offenders_photos (ID, state);

create table ky_sex_offenders_victim_info
(
	id,
	victim_info,
	state
);

create index ky_sex_offenders_victim_info__index
	on ky_sex_offenders_victim_info (id, state);

create table mt_sex_offenders
(
	r_Name,
	r_Age,
	r_Sex,
	r_Offender_Type,
	r_Birth_Date,
	r_Nicknames,
	r_Full_Address,
	r_Agency,
	r_Source,
	r_Race,
	r_Skin_Tone,
	r_Hair,
	r_Eyes,
	r_Scars_Marks_Tattoos,
	r_Height,
	r_Weight,
	r_Vehicles,
	r_Update_Date,
	r_Sentence_Date_1,
	r_Sentence_Statute_1,
	r_Sentence_Counts_1,
	r_Sentence_Date_2,
	r_Sentence_Statute_2,
	r_Sentence_Counts_2,
	r_Sentence_Date_3,
	r_Sentence_Statute_3,
	r_Sentence_Counts_3,
	r_Image,
	r_Other_Addresses,
	r_County,
	end_unformatted,
	r_Last_Name,
	r_Other_Names,
	r_Address_1,
	r_Address_2,
	r_City,
	r_State,
	r_Zip,
	r_Birth_Date_YMD,
	r_Update_Date_YMD,
	endraw,
	name,
	case_num,
	zip,
	state
);

create table tn_sex_offenders
(
	r_Name,
	r_Birth_Date,
	r_Race,
	r_Sex,
	r_Height,
	r_Weight,
	r_Eye_Color,
	r_Hair_Color,
	r_County_of_Residence,
	r_Update_Date,
	r_Last_Registration_Date,
	r_Status,
	r_Classification,
	r_Tid,
	r_Driver_License,
	r_Driver_License_State,
	r_Scars_and_Marks,
	r_Tattoos,
	r_Aliases,
	r_Offenses,
	r_Address_1,
	r_Address_2,
	r_City_State_Zip,
	r_Supervision_Site,
	r_Parole_Officer,
	r_Secondary_Address_1,
	r_Secondary_Address_2,
	r_Secondary_City_State_Zip,
	r_University,
	r_Image,
	end_unformatted,
	r_Birth_Date_YMD,
	r_Last_Name,
	r_Other_Names,
	r_City,
	r_State,
	r_Zip,
	r_Secondary_City,
	r_Secondary_State,
	r_Secondary_Zip,
	endraw,
	name,
	case_num,
	zip,
	state
);

