select Address, Name, state from CA_SexOffenders_main
union ALL
select Address, Name, state from CO_SexOffenders_main limit 10;


select Name,State  from CTSexOffenders_main
UNION
select Name, State from CO_SexOffenders_main
limit 500;

-- 39 missing some states.
select * from sqlite_master where name like '%main%' order by name;
--address tables 26
select * from sqlite_master where name like '%_address%' order by name;

--main tables that contain the address data 13
select * from sqlite_master where name  like '%main%'and sql like '%Address%' order by name;

select * from sqlite_master order by type;

--the odd balls. 4
select * from sqlite_master where name like "%_sex_offenders";

select * from al_sex_offenders;


--Insert into Sex_Offender_T1
-- AL_sex_offenders -----------------------------------------------------------
select 0 as id
       ,name
       ,r_Birth_Date as DateOfBirth
       ,state

       -- aliases
       ,json_array(
               json_object(
                       'alias',ifnull(cast(r_Aliases as Text), '')
                   )) as aliases
       --addresses
       ,json_array(
               json_object(
                       'address1',ifnull(cast(r_Home_Address_1 as Text), ''),
                       'address2', ifnull(cast(r_Home_City_State_Zip as Text), '')
                   )) as addresses
       ,json_array(
               json_object(
                       'offense',ifnull(cast(r_Sex_Crime as Text),'') || '. ' || ifnull(cast(r_Description as Text), '')
                   )) as offenses

       ,json_array(
               json_object(
                       'eyes',cast(r_eyes as Text), 'hair', cast(r_Hair as Text),
                       'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                       'race', cast(r_Race as Text),
                       'sex',cast(r_Sex as Text)

                   )) as personalDetails

       ,json_array(cast(r_Scars_Marks_Tattoos as Text)) as scarsTattoos

       ,json_array(cast(r_Image as Text)) as photos

from al_sex_offenders
Union
--KS -------------------------------------------------------------
select 0 as id
     , name
     , r_Birth_Date as DateOfBirth
     ,state
       -- aliases
       ,json_array(
               json_object(
                       'alias',ifnull(cast(r_Aliases as Text), '')
                   )) as aliases
       --addresses
       ,json_array(
               json_object(
                       'address1',ifnull(cast(r_Address_1 as Text), ''),
                       'address2', ifnull(cast(r_Address_2 as Text), '')
                   )) as addresses
       --offenses
       ,json_array(
               json_object(
                       'offense',ifnull(cast(r_Offense_1 as Text),'')
                                     || '' || ifnull(cast(r_Offense_2 as Text), '')
                                     || '' || ifnull(cast(r_Offense_3 as Text), '')
                   )) as offenses

        --personal details
       ,json_array(
               json_object(
                       'eyes',cast(r_Eye_Color as Text), 'hair', cast(r_Hair_Color as Text),
                       'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                       'race', cast(r_Race as Text),
                       'sex',cast(r_Gender as Text)

                   )) as personalDetails

        --scarsTatoos
        ,json_array('scarsTattoos',cast(r_Scars as Text) || ' ' || cast(r_Tattoos as Text)) as scarsTattoos
       ,json_array(cast(r_Image as Text)) as photos


from ks_sex_offenders
Union
-- MT ------------------------------------------------------------------
select 0 as id
     , name
     , r_Birth_Date as DateOfBirth
     ,state
       -- aliases
       ,json_array(
               json_object(
                       'alias',ifnull(cast(r_Nicknames as Text), '')
                   )) as aliases
       --addresses
       ,json_array(
               json_object(
                       'address1',ifnull(cast(r_Full_Address as Text), ''),
                       'address2', ''
                   )) as addresses
       --offenses
       ,json_array(
               json_object(
                       'offense',ifnull(cast(r_Sentence_Statute_1 as Text),'')
                                     || '' || ifnull(cast(r_Sentence_Statute_2 as Text), '')
                                     || '' || ifnull(cast(r_Sentence_Statute_3 as Text), '')
                   )) as offenses
        --personal details
       ,json_array(
               json_object(
                   'eyes',cast(r_eyes as Text)
                   ,'hair', cast(r_Hair as Text)
                   ,'height', cast(r_Height as Text)
                   ,'weight', cast(r_Weight as Text)
                   ,'race', cast(r_Race as Text)
                   ,'sex',cast(r_Sex as Text)

               )) as personalDetails
       ,json_array(cast(r_Scars_Marks_Tattoos as Text)) as scarsTattoos
        --photos
       ,json_array(cast(r_Image as Text)) as photos

from mt_sex_offenders
Union
-- TN --------------------------------------------------------------------------------
select 0 as id
     , name
     , r_Birth_Date as DateOfBirth
     ,state

       -- aliases
       ,json_array(
               json_object(
                       'alias',ifnull(cast(r_Aliases as Text), '')
                   )) as aliases
       --addresses
       ,json_array(
               json_object(
                       'address1',ifnull(cast(r_Address_1 as Text), ''),
                       'address2', ifnull(cast(r_Address_2 as Text),'')
                   )) as addresses
       --offenses
       ,json_array(
               json_object(
                       'offense',ifnull(cast(r_Offenses as Text),'')
                   )) as offenses
       --personal details
       ,json_array(
               json_object(
                   'eyes',cast(r_Eye_Color as Text), 'hair', cast(r_Hair_Color as Text),
                   'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                   'race', cast(r_Race as Text),
                   'sex',cast(r_Sex as Text)

                   )) as personalDetails
       ,json_array(cast(r_Scars_and_Marks as Text) || ' ' || cast(r_Tattoos as text))
       --photos
       ,json_array(cast(r_Image as Text)) as photos

from tn_sex_offenders
Union

--------------------------------------------------------------------

-- THE 13 -------------------------------------------------------------


-- AZ --------------------------------------------------------------------
SELECT id
      ,name
       ,'' as DateOfBirth
       ,state
         -- aliases
       ,(SELECT json_group_array(cast(alias as Text))
        FROM
            (SELECT alias
             FROM AZ_SexOffenders_aliases als
             WHERE als.id = AZ_SexOffenders_main.id
               AND AZ_SexOffenders_main.state = als.state
            )
       ) as aliases
        -- addresses
       ,json_array(
               json_object('address', cast(address as Text))) as addresses

       -- offenses
       ,(SELECT json_group_array(json_object ( 'offense', cast(offense as Text) ))
        FROM (SELECT description as offense
              FROM AZ_SexOffenders_offenses azo
              WHERE azo.id = AZ_SexOffenders_main.id
                and AZ_SexOffenders_main.state = azo.state
             )
       ) as offenses

       -- personal details
       ,(select json_group_array(
                       json_object( 'age', cast(age as Text),
                                    'eyes', cast(eyes as Text),
                                    'hair', cast(hair as Text),
                                    'height', cast(height as Text),
                                    'weight', cast(weight as Text),
                                    'race', cast(race as Text),
                                    'sex',cast(sex as Text),
                                    'status', cast(status as Text)
                           ))
        from (select age, eyes, hair, height, level, race,
                     sex, status, weight
              from AZ_SexOffenders_main azm
              where azm.id = AZ_SexOffenders_main.id
                and azm.state = AZ_SexOffenders_main.state
             )) as personalDetails

       ,json_array(cast(scars_tattoos as Text)) as scarsTattoos
        --photos
       ,(select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from AZ_SexOffenders_photos azp
              where azp.id = AZ_SexOffenders_main.id
                and azp.state = AZ_SexOffenders_main.state)) as photos


from AZ_SexOffenders_main

UNION


/*create index if not exists CA_SexOffenders_alias_index
    on CA_SexOffenders_alias (Id, Alias);
select count(*) from CA_SexOffenders_photos;

 */
-- CA ---------------------------------------------------------
SELECT id
       ,name
       ,'' as DateOfBirth
       ,state
       -- aliases
       ,(SELECT json_group_array(cast(alias as Text))
        FROM
            (SELECT alias
             FROM CA_SexOffenders_alias als
             WHERE als.id = CA_SexOffenders_main.id
               AND CA_SexOffenders_main.state = als.state
            )
       ) as aliases
       --addresses
       ,json_array(
               json_object('address', cast(address as Text))) as addresses

       -- offenses
       ,(SELECT
            json_group_array(json_object ( 'offense', cast(offense as Text)
                ))
        FROM (SELECT OffenseDescription as offense
              FROM CA_SexOffenders_offenses azo
              WHERE azo.id = CA_SexOffenders_main.id
                and CA_SexOffenders_main.state = azo.state
             )
       ) as offenses
       -- personal details
       ,(select json_group_array(
                       json_object(
                               'eyes', cast(eyes as Text),
                               'hair', cast(hair as Text),
                               'height', cast(height as Text),
                               'weight', cast(weight as Text),
                               'race', cast(race as Text),
                               'sex',cast(sex as Text)
                           ))
        from (select EyeColor as  eyes,
                     HairColor hair,
                     Height,
                     Ethnicity as race,
                     sex,
                     weight
              from CA_SexOffenders_main azm
              where azm.id = CA_SexOffenders_main.id
                and azm.state = CA_SexOffenders_main.state
             )) as personalDetails
        --scars tattoos
        ,(select json_group_array( cast(smt as text))
          from (select ScarMarkTattoo as smt from NCSexOffenders_smts smts
                where smts.id = CA_SexOffenders_main.id
                  and smts.state = CA_SexOffenders_main.state)) as scarsTattoos

       --photos
       ,(select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from CA_SexOffenders_photos azp
              where azp.id = CA_SexOffenders_main.id
                and azp.state = CA_SexOffenders_main.state)) as photos


from CA_SexOffenders_main
/*
CREATE TABLE GA_SexOffenders_main ( Absconder,  EyeColor,  FirstName,  HairColor,  Height,  ID,  LastKnownAddress,  LastName,
  Leveling,  MiddleName,  Predator,  Race,  RegistrationDate,  ResidenceVerificationDate,  Sex,  Suffix,  Weight,  YearOfBirth, state )
*/

UNION

--- GA ---------------------------------------------------
SELECT id
       ,ifnull(LastName,'') || ', ' || ifnull(FirstName,'') || ' ' || ifnull(MiddleName,'') as name
       --'' as age,
       ,YearOfBirth as DateOfBirth
       ,state
       -- aliases TODO: MISSING TABLE
       /*
       (SELECT json_group_array(cast(alias as Text))
        FROM
            (SELECT alias
             FROM GA_SexOffenders_alias als
             WHERE als.id = GA_SexOffenders_main.id
               AND GA_SexOffenders_main.state = als.state
            )
       ) as aliases,

        */
        ,json_array("Update pending") as aliases
       --addresses
       ,json_array(
               json_object('address', cast(LastKnownAddress as Text))) as addresses

       -- offenses TODO: MISSING TABLE
       /*(SELECT
            json_group_array(json_object ( 'offense', cast(offense as Text)
                ))
        FROM (SELECT OffenseDescription as offense
              FROM GA_SexOffenders_offenses azo
              WHERE azo.id = GA_SexOffenders_main.id
                and GA_SexOffenders_main.state = azo.state
             )
       ) as offenses,

        */
        ,json_array("Update pending") as offenses
       -- personal details
       ,(select json_group_array(
                       json_object(
                               'eyes', cast(eyes as Text),
                               'hair', cast(hair as Text),
                               'height', cast(height as Text),
                               'weight', cast(weight as Text),
                               'race', cast(race as Text),
                               'sex',cast(sex as Text)
                           ))
        from (select EyeColor as  eyes,
                     HairColor hair,
                     Height,
                     race,
                     sex,
                     weight
              from GA_SexOffenders_main azm
              where azm.id = GA_SexOffenders_main.id
                and azm.state = GA_SexOffenders_main.state
             )) as personalDetails
       --scars
       ,json_array("Update Pending") as scarsTattoos

       --photos
       ,(select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from GA_SexOffenders_photos azp
              where azp.id = GA_SexOffenders_main.id
                and azp.state = GA_SexOffenders_main.state)) as photos

From GA_SexOffenders_main
UNION
--- ID ----------------------------------------------------
SELECT id,
       ifnull(LastName,'') || ', ' || ifnull(FirstName,'') || ' ' || ifnull(MiddleName,'') as name ,
       DateOfBirth,
       state,
    (SELECT json_group_array(cast(alias as Text))
     FROM
         (SELECT alias
         FROM IDSexOffenders_aliases als
          WHERE als.id = IDSexOffenders_main.id
            AND IDSexOffenders_main.state = als.state
         )
    ) as aliases,

       json_array(
               json_object('address', cast(Address as TEXT) || ' ' || cast(CityStateZip as TEXT))) as addresses,

    --offenses
    (SELECT
        json_group_array(json_object (
            'offense', cast(Offense as Text) || ' ' || cast(Description as TEXT) ))
     FROM (SELECT Offense, Description
           FROM IDSexOffenders_offenses azo
           WHERE azo.id = IDSexOffenders_main.id
             and IDSexOffenders_main.state = azo.state
          )
    ) as offenses,

       -- personal details
       (select json_group_array(
                       json_object(
                               'eyes', cast(eyes as Text),
                               'hair', cast(hair as Text),
                               'height', cast(height as Text),
                               'weight', cast(weight as Text),
                               'race', cast(race as Text),
                               'sex',cast(sex as Text)
                           ))
        from (select EyeColor as  eyes,
                     HairColor hair,
                     Height,
                     race,
                     sex,
                     weight
              from IDSexOffenders_main azm
              where azm.id = IDSexOffenders_main.id
                and azm.state = IDSexOffenders_main.state
             )) as personalDetails

       ,json_array("Update Pending") as scarsTattoos

       --photos
       ,(select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from IDSexOffenders_photos azp
              where azp.id = IDSexOffenders_main.id
                and azp.state = IDSexOffenders_main.state)) as photos

From IDSexOffenders_main
UNION
--- MN ----------------------------------------------------
SELECT id
       ,ifnull(Name,'') as Name
       ,DateOfBirth
       ,state
       ,(SELECT json_group_array(cast(alias as Text))
        FROM
            (SELECT alias
             FROM MN_SexOffenders_aliases als
             WHERE als.id = MN_SexOffenders_main.id
               AND MN_SexOffenders_main.state = als.state
            )
       ) as aliases

       ,json_array(
               json_object('address', cast(RegisteredAddress as TEXT) || ' ' || cast(RegisteredCityStateZip as TEXT))) as addresses

       --offenses
       ,json_array(
            json_object('offense', cast(OffenseInformation as TEXT))
           ) as offenses
       -- personal details
       ,(select json_group_array(
                       json_object(
                               'eyes', cast(eyes as Text)
                               ,'hair', cast(hair as Text)
                               ,'height', cast(height as Text)
                               ,'weight', cast(weight as Text)
                               ,'race', cast(race as Text)
                              -- ,'sex',cast(sex as Text)
                           ))
        from (select EyeColor as  eyes,
                     HairColor as hair,
                     Height,
                     weight,
                     RaceEthnicity as race

              from MN_SexOffenders_main azm
              where azm.id = MN_SexOffenders_main.id
                and azm.state = MN_SexOffenders_main.state
             )) as personalDetails

       ,json_array("Update Pending") as scarsTattoos

       --photos
       ,(select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from MN_SexOffenders_photos azp
              where azp.id = MN_SexOffenders_main.id
                and azp.state = MN_SexOffenders_main.state)) as photos

From MN_SexOffenders_main
union
--- NC  -------------------------------------
SELECT id
     ,ifnull(Name,'') as Name
     ,DateOfBirth
     ,state
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM NCSexOffenders_alias als
            WHERE als.id = NCSexOffenders_main.id
              AND NCSexOffenders_main.state = als.state
           )
) as aliases

     ,json_array(
        json_object('address', cast(AddressLine1 as TEXT) || ' ' || cast(AddressLine2 as TEXT))) as addresses

     --offenses TODO: MISSING TABLE
     ,json_array( json_object('offense', 'UPDATE PENDING')) as offenses
     -- personal details
     ,(select json_group_array(
                      json_object(
                              'eyes', cast(eyes as Text)
                          ,'hair', cast(hair as Text)
                          ,'height', cast(height as Text)
                          ,'weight', cast(weight as Text)
                          ,'race', cast(race as Text)
                          ,'sex',cast(sex as Text)
                          ))
       from (select Eyes
                    ,Hair
                    ,Height
                    ,weight
                    ,Race
                    ,Sex
             from NCSexOffenders_main azm
             where azm.id = NCSexOffenders_main.id
               and azm.state = NCSexOffenders_main.state
            )) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select ScarMarkTattoo as smt from NCSexOffenders_smts smts
             where smts.id = NCSexOffenders_main.id
               and smts.state = NCSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from NCSexOffenders_photos azp
             where azp.id = NCSexOffenders_main.id
               and azp.state = NCSexOffenders_main.state)) as photos


From NCSexOffenders_main
Union

--- ND  -------------------------------------
SELECT id
     ,ifnull(Name,'') as Name
     ,DateOfBirth
     ,state
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM NDSexOffenders_alias als
            WHERE als.id = NDSexOffenders_main.id
              AND NDSexOffenders_main.state = als.state
           )
) as aliases

     ,json_array(
        json_object('address', cast(StreetAddress as Text) || ' ' || cast(AddressName as TEXT) || ' ' || cast(CityStateZip as TEXT))) as addresses

     --offenses

     ,(SELECT
		json_group_array (
			json_object ('offense', cast(offense as TEXT))
		)
		FROM

		(SELECT
			offense
		    FROM NDSexOffenders_convictions aro where NDSexOffenders_main.ID = aro.ID
		    and NDSexOffenders_main.state = aro.state
		)) as offenses
        -- personal details
     ,(select json_group_array(
                      json_object(
                              'eyes', cast(eyes as Text)
                          ,'hair', cast(hair as Text)
                          ,'height', cast(height as Text)
                          ,'weight', cast(weight as Text)
                          ,'race', cast(race as Text)
                           ,'sex',cast(sex as Text)
                          ))
       from (select Eyes
                  ,Hair
                  ,Height
                  ,weight
                  ,Race
                 ,Sex
             from NDSexOffenders_main azm
             where azm.id = NDSexOffenders_main.id
               and azm.state = NDSexOffenders_main.state
            )) as personalDetails

    ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from NDSexOffenders_photos azp
             where azp.id = NDSexOffenders_main.id
               and azp.state = NDSexOffenders_main.state)) as photos


From NDSexOffenders_main

union

--- OR  -------------------------------------
SELECT id
     ,ifnull(Name,'') as Name
     ,DOB as DateOfBirth
     ,state
     --alias
     ,json_array("None Reported") as aliases
    --address
     ,json_array(
        json_object('address', cast(Residence as TEXT))) as addresses
     --offenses
   ,(SELECT
		json_group_array (
			json_object ('offense', offense)
		)
		FROM

		(SELECT
			offenseName as offense
		    FROM ORSexOffenders_offenses aro where ORSexOffenders_main.ID = aro.ID
		    and ORSexOffenders_main.state = aro.state
		)
    ) as offenses
     -- personal details
     ,(select json_group_array(
                      json_object(
                              'eyes', cast(eyes as Text)
                          ,'hair', cast(hair as Text)
                          ,'height', cast(height as Text)
                          ,'weight', cast(weight as Text)
                          ,'sex',cast(sex as Text)
                          ))
       from (select Eyes
                  ,Hair
                  ,Height
                  ,weight
                  ,Sex
             from ORSexOffenders_main azm
             where azm.id = ORSexOffenders_main.id
               and azm.state = ORSexOffenders_main.state
            )) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select smt from ORSexOffenders_smts smts
             where smts.id = ORSexOffenders_main.id
               and smts.state = ORSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from ORSexOffenders_photos azp
             where azp.id = ORSexOffenders_main.id
               and azp.state = ORSexOffenders_main.state)) as photos


From ORSexOffenders_main
Union

--- SD  -------------------------------------
SELECT id
     ,ifnull(Name,'') as Name
     ,DateOfBirth
     ,state
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM SDSexOffenders_alias als
            WHERE als.id = SDSexOffenders_main.id
              AND SDSexOffenders_main.state = als.state
           )
) as aliases

     ,json_array(
        json_object('address', cast(Address as Text) || ' ' || cast(CityStateZip as TEXT))) as addresses

     --offenses

     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as TEXT) || ' ' || cast(description as Text))
               )
       FROM

           (SELECT
                CrimesConvicted as offense
                ,CrimeDescription as description
            FROM SDSexOffenders_convictions aro where SDSexOffenders_main.ID = aro.ID
                                                  and SDSexOffenders_main.state = aro.state
           )) as offenses
     -- personal details
     ,(select json_group_array(
                      json_object(
                              'eyes', cast(eyes as Text)
                          ,'hair', cast(hair as Text)
                          ,'height', cast(height as Text)
                          ,'weight', cast(weight as Text)
                          ,'race', cast(race as Text)
                          ,'sex',cast(sex as Text)
                          ))
       from (select EyeColor as eyes
                  ,HairColor as hair
                  ,Height
                  ,weight
                  ,Race
                  ,Gender as sex
             from SDSexOffenders_main azm
             where azm.id = SDSexOffenders_main.id
               and azm.state = SDSexOffenders_main.state
            )) as personalDetails

     ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from SDSexOffenders_photos azp
             where azp.id = SDSexOffenders_main.id
               and azp.state = SDSexOffenders_main.state)) as photos


From SDSexOffenders_main

UNION
--- UT  -------------------------------------
SELECT id
     ,Name
     ,DOB as DateOfBirth
     ,state
     --aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM UT_SexOffenders_aliases als
            WHERE als.id = UT_SexOffenders_main.id
              AND UT_SexOffenders_main.state = als.state
           )
        ) as aliases

     --addresses
     ,json_array(
        json_object('address', cast(Address as Text))) as addresses

     --offenses

     ,(SELECT
           json_group_array ( json_object ('offense', cast(offense as TEXT) ))
       FROM

           (SELECT
               OffenseDescription as offense
            FROM UT_SexOffenders_offenses aro where UT_SexOffenders_main.ID = aro.ID
                                                  and UT_SexOffenders_main.state = aro.state
           )) as offenses
     -- personal details
     ,(select json_group_array(
                      json_object(
                              'eyes', cast(eyes as Text)
                          ,'hair', cast(hair as Text)
                          ,'height', cast(height as Text)
                          ,'weight', cast(weight as Text)
                          ,'race', cast(race as Text)
                          ,'sex',cast(sex as Text)
                          ))
       from (select Eyes
                  ,Hair
                  ,Height
                  ,weight
                  ,Race
                  ,Sex
             from UT_SexOffenders_main azm
             where azm.id = UT_SexOffenders_main.id
               and azm.state = UT_SexOffenders_main.state
            )) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from UT_SexOffenders_smts smts
             where smts.id = UT_SexOffenders_main.id
               and smts.state = UT_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from UT_SexOffenders_photos azp
             where azp.id = UT_SexOffenders_main.id
               and azp.state = UT_SexOffenders_main.state)) as photos


From UT_SexOffenders_main

Union
--- VT  -------------------------------------
SELECT id
     ,Name
     ,DateOfBirth
     ,state
     --aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM VTSexOffenders_aliases als
            WHERE als.id = VTSexOffenders_main.id
              AND VTSexOffenders_main.state = als.state
           )
) as aliases

     --addresses
     ,json_array(
        json_object('address', cast(Address as Text))) as addresses

     --offenses
     ,json_array("Pending Update") as offenses
     --TODO: MISSING TABLE
/*
     ,(SELECT
           json_group_array ( json_object ('offense', cast(offense as TEXT) ))
       FROM

           (SELECT
                OffenseDescription as offense
            FROM VTSexOffenders_offenses aro where VTSexOffenders_main.ID = aro.ID
                                                and VTSexOffenders_main.state = aro.state
           )) as offenses*/
     -- personal details
     ,(select json_group_array(
                      json_object(
                              'eyes', cast(eyes as Text)
                          ,'hair', cast(hair as Text)
                          ,'height', cast(height as Text)
                          ,'weight', cast(weight as Text)
                          ,'race', cast(race as Text)
                          ,'sex',cast(sex as Text)
                          ))
       from (select Eyes
                  ,Hair
                  ,Height
                  ,weight
                  ,Race
                  ,Sex
             from VTSexOffenders_main azm
             where azm.id = VTSexOffenders_main.id
               and azm.state = VTSexOffenders_main.state
            )) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from VTSexOffenders_smts smts
             where smts.id = VTSexOffenders_main.id
               and smts.state = VTSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from VTSexOffenders_photos azp
             where azp.id = VTSexOffenders_main.id
               and azp.state = VTSexOffenders_main.state)) as photos


From VTSexOffenders_main
UNION
--- WA  -------------------------------------
SELECT id
     ,Name
     ,age as DateOfBirth
     ,state
     --aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM WA_SexOffenders_aliases als
            WHERE als.id = WA_SexOffenders_main.id
              AND WA_SexOffenders_main.state = als.state
           )
) as aliases

     --addresses
     ,json_array(
        json_object('address', cast(Address as Text))) as addresses

     --offenses

     ,(SELECT
           json_group_array ( json_object ('offense', cast(offense as TEXT) ))
       FROM

           (SELECT
                OffenseDescription as offense
            FROM WA_SexOffenders_offenses aro where WA_SexOffenders_main.ID = aro.ID
                                                and WA_SexOffenders_main.state = aro.state
           )) as offenses
     -- personal details
     ,(select json_group_array(
                      json_object(
                              'eyes', cast(eyes as Text)
                          ,'hair', cast(hair as Text)
                          ,'height', cast(height as Text)
                          ,'weight', cast(weight as Text)
                          ,'race', cast(race as Text)
                          ,'sex',cast(sex as Text)
                          ))
       from (select Eyes
                  ,Hair
                  ,Height
                  ,weight
                  ,Race
                  ,Sex
             from WA_SexOffenders_main azm
             where azm.id = WA_SexOffenders_main.id
               and azm.state = WA_SexOffenders_main.state
            )) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from WA_SexOffenders_smts smts
             where smts.id = WA_SexOffenders_main.id
               and smts.state = WA_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from WA_SexOffenders_photos azp
             where azp.id = WA_SexOffenders_main.id
               and azp.state = WA_SexOffenders_main.state)) as photos


From WA_SexOffenders_main
UNION
-- The remaining

------------ CO

select id
       ,name
       ,DateOfBirth
       ,state
       -- aliases
       ,( SELECT json_group_array (cast(alias as TEXT))
         FROM
             (SELECT alias
              FROM CO_SexOffenders_aliases als
              WHERE als.id = CO_SexOffenders_main.id
                AND CO_SexOffenders_main.state = als.state)
       ) as "aliases"

        --offenses
       ,(SELECT
            json_group_array (
                    json_object ('offense', cast(offense as TEXT))
                )
        FROM

            (SELECT
                 Description as offense
            FROM CO_SexOffenders_convictions aro
            where CO_SexOffenders_main.ID = aro.ID
            and CO_SexOffenders_main.state = aro.state
            )
       ) as offenses
      -- addresses
      ,(SELECT
            json_group_array(
                json_object ('address', cast(address as TEXT)
                                    || ' ' || cast(AddressExt as TEXT)
                                    || ' ' || cast(CityZip as TEXT)
        ))
        FROM
            (SELECT address,
                    AddressExt,
                    CityZip
            FROM CO_SexOffenders_addresses arad
            where arad.ID = CO_SexOffenders_main.ID
            and arad.state = CO_SexOffenders_main.state

            )) as addresses

        --personalDetails
       ,(SELECT
            json_group_array (
                json_object(
                'eyes',cast(Eyes as TEXT),
                'hair', cast(Hair as TEXT),
                'race',cast(Race as TEXT),
                'sex',cast (Sex as TEXT)
            ))

        FROM
            (SELECT
                 Eyes,
                 Hair,
                 Race,
                 Gender as Sex
             FROM

                 CO_SexOffenders_main arm where arm.ID = CO_SexOffenders_main.id
                                            and arm.state = CO_SexOffenders_main.state
            )
       ) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select ScarMarkTattoo as smt from CO_SexOffenders_smts smts
             where smts.id = CO_SexOffenders_main.id
               and smts.state = CO_SexOffenders_main.state)) as scarsTattoos

       ,(select json_group_array(cast(PhotoFile as TEXT))
        from (select PhotoFile from CO_SexOffenders_photos azp
              where azp.id = CO_SexOffenders_main.id
                and azp.state = CO_SexOffenders_main.state)) as photos


from CO_SexOffenders_main
Union

------------- CT
select id
     ,name
     ,DateOfBirth
     ,state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM CTSexOffenders_aliases als
             WHERE als.id = CTSexOffenders_main.id
               AND CTSexOffenders_main.state = als.state)
) as "aliases"

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', offense)
               )
       FROM

           (SELECT
                cast(OffenseDescription as TEXT) || ' ' || cast(OffenseDetails as TEXT) as offense
            FROM CTSexOffenders_offenses aro
            where CTSexOffenders_main.ID = aro.ID
              and CTSexOffenders_main.state = aro.state
           )
) as offenses
     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(address as TEXT)
                       || ' ' || cast(AddressExtended as TEXT)
                       || ' ' || cast(CityStateZip as TEXT)
                       ))
       FROM
           (SELECT address,
                   AddressExtended,
                   CityStateZip
            FROM CTSexOffenders_addresses arad
            where arad.ID = CTSexOffenders_main.ID
              and arad.state = CTSexOffenders_main.state

           )) as addresses

     --personalDetails
     ,(SELECT
           json_group_array (
                   json_object(
                           'eyes',cast(Eyes as TEXT),
                           'hair', cast(Hair as TEXT),
                           'race',cast(Race as TEXT),
                           'sex',cast (Sex as TEXT)
                       ))

       FROM
           (SELECT
                Eyes,
                Hair,
                Race,
                Sex
            FROM

                CTSexOffenders_main arm where arm.ID = CTSexOffenders_main.id
                                           and arm.state = CTSexOffenders_main.state
           )
) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from CTSexOffenders_smts smts
             where smts.id = CTSexOffenders_main.id
               and smts.state = CTSexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from CTSexOffenders_photos azp
             where azp.id = CTSexOffenders_main.id
               and azp.state = CTSexOffenders_main.state)) as photos


from CTSexOffenders_main
union
-------------DE
select id
     ,name
     ,BirthDate as DateOfBirth
     ,state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM DE_SexOffenders_aliases als
             WHERE als.id = DE_SexOffenders_main.id
               AND DE_SexOffenders_main.state = als.state)
) as "aliases"

     --offenses
     ,json_array("Pending Update") as offenses
     /*
     ,(SELECT
           json_group_array (
                   json_object ('offense', offense)
               )
       FROM

           (SELECT
                cast(OffenseDescription as TEXT) || ' ' || cast(OffenseDetails as TEXT) as offense
            FROM DE_SexOffenders_offenses aro
            where DE_SexOffenders_main.ID = aro.ID
              and DE_SexOffenders_main.state = aro.state
           )
) as offenses

      */
     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Street as TEXT)
                       || ' ' || cast(CityStateZip as TEXT)
                       ,'type',cast(Type as TEXT)))
       FROM
           (SELECT street,
                   CityStateZip,
                   Type
            FROM DE_SexOffenders_addresses arad
            where arad.ID = DE_SexOffenders_main.ID
              and arad.state = DE_SexOffenders_main.state

           )) as addresses

     --personalDetails
     ,(SELECT
           json_group_array (
                   json_object(
                           'eyes',cast(Eyes as TEXT),
                           'hair', cast(Hair as TEXT),
                           'race',cast(Race as TEXT),
                           'sex',cast (Sex as TEXT)
                       ))

       FROM
           (SELECT
               EyeColor as  Eyes,
               HairColor as  Hair,
               Race,
               Gender as sex
            FROM

                DE_SexOffenders_main arm where arm.ID = DE_SexOffenders_main.id
                                           and arm.state = DE_SexOffenders_main.state
           )
) as personalDetails

     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from DE_SexOffenders_smts smts
             where smts.id = DE_SexOffenders_main.id
               and smts.state = DE_SexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from DE_SexOffenders_photos azp
             where azp.id = DE_SexOffenders_main.id
               and azp.state = DE_SexOffenders_main.state)) as photos


from DE_SexOffenders_main
union

------------- FL
select id
     ,name
     ,DateOfBirth
     ,state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM FL_SexOffenders_aliases als
             WHERE als.id = FL_SexOffenders_main.id
               AND FL_SexOffenders_main.state = als.state)
) as "aliases"

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', offense)
               )
       FROM

           (SELECT
                    cast(CrimeDescription as TEXT) as offense
            FROM FL_SexOffenders_offenses aro
            where FL_SexOffenders_main.ID = aro.ID
              and FL_SexOffenders_main.state = aro.state
           )
) as offenses
     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(street as TEXT)
                       || ' ' || cast(CityStateZip as TEXT)
                       ))
       FROM
           (SELECT street,
                   CityStateZip
            FROM FL_SexOffenders_addresses arad
            where arad.ID = FL_SexOffenders_main.ID
              and arad.state = FL_SexOffenders_main.state

           )) as addresses

     --personalDetails
     ,(SELECT
           json_group_array (
                   json_object(
                           'eyes',cast(Eyes as TEXT),
                           'hair', cast(Hair as TEXT),
                           'race',cast(Race as TEXT),
                           'sex',cast (Sex as TEXT)
                       ))

       FROM
           (SELECT
                Eyes,
                Hair,
                Race,
                Sex
            FROM

                FL_SexOffenders_main arm where arm.ID = FL_SexOffenders_main.id
                                          and arm.state = FL_SexOffenders_main.state
           )
) as personalDetails

     ,(select json_group_array( cast(type as text) || ' ' || cast(number as Text) || ' ' || cast(location as Text))
       from (select type, location, number  from FL_SexOffenders_smts smts
             where smts.id = FL_SexOffenders_main.id
               and smts.state = FL_SexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from FL_SexOffenders_photos azp
             where azp.id = FL_SexOffenders_main.id
               and azp.state = FL_SexOffenders_main.state)) as photos


from FL_SexOffenders_main
Union
-------------ID
select id
     ,name
     ,DateOfBirth
     ,state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM IDSexOffenders_aliases als
             WHERE als.id = IDSexOffenders_main.id
               AND IDSexOffenders_main.state = als.state)
) as "aliases"

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', offense)
               )
       FROM

           (SELECT
                cast(Description as TEXT) as offense
            FROM IDSexOffenders_offenses aro
            where IDSexOffenders_main.ID = aro.ID
              and IDSexOffenders_main.state = aro.state
           )
) as offenses
     --addresses
     ,json_array(
        json_object(
                'address',cast(Address as Text) || ' ' || cast(CityStateZip as Text))
            ) as addresses

     --personalDetails
     ,(SELECT
           json_group_array (
                   json_object(
                           'eyes',cast(Eyes as TEXT),
                           'hair', cast(Hair as TEXT),
                           'race',cast(Race as TEXT),
                           'sex',cast (Sex as TEXT)
                       ))

       FROM
           (SELECT
               EyeColor as Eyes,
               HairColor as Hair,
                Race,
                Sex
            FROM

                IDSexOffenders_main arm where arm.ID = IDSexOffenders_main.id
                                           and arm.state = IDSexOffenders_main.state
           )
) as personalDetails

     ,json_array("None Reported") as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from IDSexOffenders_photos azp
             where azp.id = IDSexOffenders_main.id
               and azp.state = IDSexOffenders_main.state)) as photos


from IDSexOffenders_main


order by state desc;


