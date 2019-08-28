
create TABLE TXAddress
(
    AddressId,
    IND_IDN,
    SNU_NBR,
    SNA_TXT,
    SUD_COD,
    SUD_NBR,
    CTY_TXT,
    PLC_COD,
    ZIP_TXT,
    COU_COD,
    LAT_NBR,
    LON_NBR
);

Create index if not exists TXAddress__index on
TXAddress (IND_IDN);

Create TABLE TXPerson
(

    IND_IDN,
    PER_IDN,
    SEX_COD,
    RAC_COD,
    HGT_QTY,
    WGT_QTY,
    HAI_COD,
    EYE_COD,
    ETH_COD
);


Create index if not exists TXPerson__index on
    TXPerson (IND_IDN, PER_IDN);

drop table TXOffense;

Create TABLE TXOffense
(
    IND_IDN,
    OffenseId,
    COO_COD,
    COJ_COD,
    JOO_COD,
    OFF_COD,
    VER_NBR,
    GOC_COD,
    DIS_FLG,
    OST_COD,
    CPR_COD,
    CDD_DTE,
    AOV_NBR,
    SOV_COD,
    CPR_VAL
);

Create index if not exists TXOffense__index on
    TXOffense (IND_IDN, OffenseId);


Create index if not exists TXOffense__index_1 on
    TXOffense (IND_IDN);


Create TABLE TXOff_Code_Sor
(
            COO_COD,
            COJ_COD,
            JOO_COD,
            OFF_COD,
            VER_NBR,
            LEN_TXT,
            STS_COD,
            CIT_TXT,
            BeginDate,
            EndDate
);

Create index if not exists TXOff_Code_Sor__index on
    TXOff_Code_Sor (OFF_COD);


Create TABLE TXBRTHDATE
(
    DOB_IDN,
    PER_IDN,
    TYP_COD,
    DOB_DTE
);


Create index if not exists TXBRTHDATE__index on
    TXBRTHDATE (PER_IDN);

CREATE TABLE TXNAME
(
    NAM_IDN,
    PER_IDN,
    TYP_COD,
    NAM_TXT,
    LNA_TXT,
    FNA_TXT
);

Create index if not exists TXNAME__index on
TXNAME (PER_IDN);

Create TABLE TXPhoto
(
    IND_IDN,
    PhotoId,
    POS_DTE
);

Create index if not exists TXPhoto__index on
    TXPhoto (IND_IDN);

Create index if not exists TXPhoto__index1 on
    TXPhoto (PhotoId);


Drop table TXNAME;
Drop table TXBRTHDATE;
DROP TABLE TXOff_Code_Sor;
Drop TABLE TXOffense;
DROP TABLE TXNAME;
DROP TABLE  TXAddress;
DROP TABLE TXPhoto;

                                                    )