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
select count(*) from Photos where State = "AL"
select * from al_sex_offenders;
select count(*) from AR_sex_offender_main;
select count(*) from SexOffender where state = "AR"
select * from SexOffender where state = "AR"
delete from SexOffender where state = "AR"


--order by state desc;

Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)

----------------AK
select id
     ,name
     ,DOB as DateOfBirth

     ,Eyes
     ,Hair
     ,Height
     ,Weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     ,( SELECT json_group_array (cast(Aliases as TEXT))
        FROM
            (SELECT Aliases

             FROM AK_sex_offender_aliases als
             WHERE als.id = AK_sex_offender_main.id
               AND AK_sex_offender_main .state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          || ' ' ||  cast(City as TEXT)
                          || ' ' || cast(Addr_State AS TEXT),
                                  'type', cast(Type as TEXT)
                          ))

       FROM (SELECT Address,City, Addr_State, Type
             FROM AK_sex_offender_addresses arad
             where arad.ID = AK_sex_offender_main.ID
               and arad.state = AK_sex_offender_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(Description as Text)
                       ))
       FROM

           (SELECT description
            FROM AK_sex_offender_offenses aro
            where AK_sex_offender_main.ID = aro.ID
              and AK_sex_offender_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,json_array("Unknown") as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from AK_sex_offender_photos azp
             where azp.id = AK_sex_offender_main.id
               and azp.state = AK_sex_offender_main.state)) as photos

from AK_sex_offender_main;
-- AR --------------------------------------------------------------------
SELECT id
     ,name
     ,DOB  as DateOfBirth
     ,eyes
     ,hair
     ,'' as height
     ,'' as weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM AR_sex_offender_alias als
            WHERE als.id = AR_sex_offender_main.id
              AND AR_sex_offender_main.state = als.state
           )
) as aliases
     -- addresses
     ,json_array(
        json_object('address', cast(address as Text))) as addresses

     -- offenses
     ,json_array(
         json_object('offense', cast(Offense as Text))) as offenses

     ,json_array(cast(ScarsTattoos as Text)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(Photo as Text))
       from (select Photo from AR_sex_offender_photos azp
             where azp.id = AR_sex_offender_main.id
               and azp.state = AR_sex_offender_main.state)) as photos


from AR_sex_offender_main
UNION
-- AL_sex_offenders -----------------------------------------------------------
select r_Image as id
       ,name
       ,r_Birth_Date as DateOfBirth
        ,r_Eyes as eyes
        ,r_Hair as hair
        ,r_Height as height
       ,r_Weight as weight
         ,r_race as race
        ,r_Sex as sex
       ,trim(state) as state

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
       --offenses
       ,json_array(
               json_object(
                       'offense',ifnull(cast(r_Sex_Crime as Text),'') || '. ' || ifnull(cast(r_Description as Text), '')
                   )) as offenses
       ,json_array(cast(r_Scars_Marks_Tattoos as Text)) as scarsTattoos

       ,json_array(cast(r_Image as Text)) as photos

from al_sex_offenders
Union
--KS -------------------------------------------------------------
select 0 as id
     , name
     , r_Birth_Date as DateOfBirth
     ,r_Eye_Color as eyes
     ,r_Hair_Color as hair
     ,r_Height as height
     ,r_Weight as weight
     ,r_race as race
     ,r_Gender as sex
     ,trim(state) as state
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


        --scarsTatoos
        ,json_array('scarsTattoos',cast(r_Scars as Text) || ' ' || cast(r_Tattoos as Text)) as scarsTattoos
       ,json_array(cast(r_Image as Text)) as photos


from ks_sex_offenders
Union
-- MT ------------------------------------------------------------------
select 0 as id
     , name
     , r_Birth_Date as DateOfBirth

     ,r_Eyes as eyes
     ,r_Hair as hair
     ,r_Height as height
     ,r_Weight as weight
     ,r_race as race
     ,r_Sex as sex
     ,trim(state) as state
     -- aliases
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
       ,json_array(cast(r_Scars_Marks_Tattoos as Text)) as scarsTattoos
        --photos
       ,json_array(cast(r_Image as Text)) as photos

from mt_sex_offenders
Union
-- TN --------------------------------------------------------------------------------
select 0 as id
     , name
     , r_Birth_Date as DateOfBirth

     ,r_Eye_Color as eyes
     ,r_Hair_Color as hair
     ,r_Height as height
     ,r_Weight as weight
     ,r_race as race
     ,r_Sex as sex
     ,trim(state) as state
     -- aliases

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
       ,age  as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
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

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,Ethnicity as race
     ,sex
     ,trim(state) as state
     -- aliases

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

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
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
       --scarsTattoos
       ,json_array("Update Pending") as scarsTattoos

       --photos
       ,(select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from GA_SexOffenders_photos azp
              where azp.id = GA_SexOffenders_main.id
                and azp.state = GA_SexOffenders_main.state)) as photos

From GA_SexOffenders_main
UNION
--- ID ----------------------------------------------------
SELECT id
       ,ifnull(LastName,'') || ', ' || ifnull(FirstName,'') || ' ' || ifnull(MiddleName,'') as name
       ,DateOfBirth
     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
    ,(SELECT json_group_array(cast(alias as Text))
     FROM
         (SELECT alias
         FROM IDSexOffenders_aliases als
          WHERE als.id = IDSexOffenders_main.id
            AND IDSexOffenders_main.state = als.state
         )
    ) as aliases

    ,json_array(
               json_object('address', cast(Address as TEXT) || ' ' || cast(CityStateZip as TEXT))) as addresses

    --offenses
    ,(SELECT
        json_group_array(json_object (
            'offense', cast(Offense as Text) || ' ' || cast(Description as TEXT) ))
     FROM (SELECT Offense, Description
           FROM IDSexOffenders_offenses azo
           WHERE azo.id = IDSexOffenders_main.id
             and IDSexOffenders_main.state = azo.state
          )
    ) as offenses

       --scarsTattoos
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

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,RaceEthnicity as race
     ,'' as sex
     ,trim(state) as state
     -- aliases
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

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
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
     --scarsTattoos
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
     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
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

     ,eyes
     ,hair
     ,height
     ,weight
     ,'' as race --no race listed
     ,sex
     ,trim(state) as state
     -- aliases
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

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,DateOfBirth
     ,trim(state) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM SDSexOffenders_alias als
            WHERE als.id = SDSexOffenders_main.id
              AND SDSexOffenders_main.state = als.state
           )
) as aliases
    --addresses
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
     --scarsTattoos
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

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
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
     --scarsTattoos
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

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
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
    --scarsTattoos
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

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
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
     --scarsTattoos
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

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
       -- aliases
       ,( SELECT json_group_array (cast(alias as TEXT))
         FROM
             (SELECT alias
              FROM CO_SexOffenders_aliases als
              WHERE als.id = CO_SexOffenders_main.id
                AND CO_SexOffenders_main.state = als.state)
       ) as "aliases"

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
    -- scarsTattoos
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
     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM CTSexOffenders_aliases als
             WHERE als.id = CTSexOffenders_main.id
               AND CTSexOffenders_main.state = als.state)
) as aliases

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
    --scarsTattoos
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

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM DE_SexOffenders_aliases als
             WHERE als.id = DE_SexOffenders_main.id
               AND DE_SexOffenders_main.state = als.state)
) as aliases

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
    --scarsTattoos
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

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM FL_SexOffenders_aliases als
             WHERE als.id = FL_SexOffenders_main.id
               AND FL_SexOffenders_main.state = als.state)
) as "aliases"

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
    --scarsTattoos
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

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM IDSexOffenders_aliases als
             WHERE als.id = IDSexOffenders_main.id
               AND IDSexOffenders_main.state = als.state)
) as "aliases"

     --addresses
     ,json_array(
        json_object(
                'address',cast(Address as Text) || ' ' || cast(CityStateZip as Text))
    ) as addresses

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
    --scarsTattoos
     ,json_array("None Reported") as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from IDSexOffenders_photos azp
             where azp.id = IDSexOffenders_main.id
               and azp.state = IDSexOffenders_main.state)) as photos


from IDSexOffenders_main
Union
--------- IN
select id
     ,name
     ,YearOfBirth as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM INSexOffenders_aliases als
             WHERE als.id = INSexOffenders_main.id
               AND INSexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(AddressExtension as TEXT), 'type', cast(AddressType as Text)
                       ))

       FROM
           (SELECT Address,
                   AddressExtension,
                   AddressType
            FROM INSexOffenders_addresses arad where arad.ID = INSexOffenders_main.ID
                                                 and arad.state = INSexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text)
                       || ' ' || cast(OffenseDetails as Text)
               ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM INSexOffenders_offenses aro
            where INSexOffenders_main.ID = aro.ID
              and INSexOffenders_main.state = aro.state
           )
) as offenses

    --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from INSexOffenders_smts smts
             where smts.id = INSexOffenders_main.id
               and smts.state = INSexOffenders_main.state)) as scarsTattoos
    --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from INSexOffenders_photos azp
             where azp.id = INSexOffenders_main.id
               and azp.state = INSexOffenders_main.state)) as photos


from INSexOffenders_main
Union
------------------ LA
select id
     ,name
     ,DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM LA_SexOffenders_aliases als
             WHERE als.id = LA_SexOffenders_main.id
               AND LA_SexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT), 'type', cast(AddressType as Text)
                       ))

       FROM
           (SELECT Address,
                   AddressType
            FROM LA_SexOffenders_addresses arad where arad.ID = LA_SexOffenders_main.ID
                                                  and arad.state = LA_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text)
                       || ' ' || cast(OffenseDetails as Text)
                       ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM LA_SexOffenders_offenses aro
            where LA_SexOffenders_main.ID = aro.ID
              and LA_SexOffenders_main.state = aro.state
           )
) as offenses

    --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from LA_SexOffenders_smts smts
             where smts.id = LA_SexOffenders_main.id
               and smts.state = LA_SexOffenders_main.state)) as scarsTattoos
    --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from LA_SexOffenders_photos azp
             where azp.id = LA_SexOffenders_main.id
               and azp.state = LA_SexOffenders_main.state)) as photos

from LA_SexOffenders_main
UNION

---------------- MA
select id
     ,name
     ,Year_Of_Birth as DateOfBirth

     ,Eye_Color as eyes
     ,Hair_Color as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MA_SexOffenders_aliases als
             WHERE als.id = MA_SexOffenders_main.id
               AND MA_SexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT), 'type', cast(Type as Text)
                       ))

       FROM
           (SELECT Address,
                   Type
            FROM MA_SexOffenders_addresses arad where arad.ID = MA_SexOffenders_main.ID
                                                  and arad.state = MA_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT Jurisdiction as offense
            FROM MA_SexOffenders_offenses aro
            where MA_SexOffenders_main.ID = aro.ID
              and MA_SexOffenders_main.state = aro.state
           )
) as offenses

    --scarsTattoos
    ,json_array("None Reported") as scarsTattoos
     /*,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from MA_SexOffenders_smts smts
             where smts.id = MA_SexOffenders_main.id
               and smts.state = MA_SexOffenders_main.state)) as scarsTattoos
*/
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MA_SexOffenders_photos azp
             where azp.id = MA_SexOffenders_main.id
               and azp.state = MA_SexOffenders_main.state)) as photos

from MA_SexOffenders_main

UNION

---------------- MD
select id
     ,name
     ,DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MDSexOffenders_aliases als
             WHERE als.id = MDSexOffenders_main.id
               AND MDSexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(city_state_zip as TEXT), 'type', cast(AddressType as Text)
                       ))

       FROM
           (SELECT Address,
                   city_state_zip,
                   AddressType
            FROM MDSexOffenders_addresses arad where arad.ID = MDSexOffenders_main.ID
                                                 and arad.state = MDSexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text), 'descr',cast(Description as Text) ))
       FROM

           (SELECT Charges as offense,
                   Description
            FROM MDSexOffenders_offenses aro
            where MDSexOffenders_main.ID = aro.ID
              and MDSexOffenders_main.state = aro.state
           )
) as offenses
    --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MDSexOffenders_photos azp
             where azp.id = MDSexOffenders_main.id
               and azp.state = MDSexOffenders_main.state)) as photos

from MDSexOffenders_main
UNION

---------------- ME
select id
     ,name
     ,DateOfBirth

     ,'' as eyes
     ,'' as hair
     ,'' as height
     ,'' as weight
     ,'' as race
     ,'' as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM ME_SexOffenders_aliases als
             WHERE als.id = ME_SexOffenders_main.id
               AND ME_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT)
                       ))

       FROM
           (SELECT TownOfDomicile as address
            FROM ME_SexOffenders_addresses arad where arad.ID = ME_SexOffenders_main.ID
                                                  and arad.state = ME_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text), 'descr',cast(Description as Text) ))
       FROM

           (SELECT Statute as offense,
                   Offense as Description
            FROM ME_SexOffenders_offenses aro
            where ME_SexOffenders_main.ID = aro.ID
              and ME_SexOffenders_main.state = aro.state
           )
) as offenses

     /*,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from ME_SexOffenders_smts smts
             where smts.id = ME_SexOffenders_main.id
               and smts.state = ME_SexOffenders_main.state)) as scarsTattoos
    */
     --personalDetails
     ,json_array("None Reported") as scarsTattoos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from ME_SexOffenders_photos azp
             where azp.id = ME_SexOffenders_main.id
               and azp.state = ME_SexOffenders_main.state)) as photos

from ME_SexOffenders_main
Union

---------------- MI
select id
     ,name
     ,DateOfBirth
     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MISexOffenders_aliases als
             WHERE als.id = MISexOffenders_main.id
               AND MISexOffenders_main.state = als.state)
) as aliases

     ,json_array("Update Pending") as addresses
     --addresses
/*
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT)
                       ))

       FROM
           (SELECT TownOfDomicile as address
            FROM MISexOffenders_addresses arad where arad.ID = MISexOffenders_main.ID
                                                  and arad.state = MISexOffenders_main.state)

) as addresses


 */
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text), 'descr',cast(Description as Text) ))
       FROM

           (SELECT OffenseDescription  as offense,
                   OffenseDetails as Description
            FROM MISexOffenders_covictions aro
            where MISexOffenders_main.ID = aro.ID
              and MISexOffenders_main.state = aro.state
           )
) as offenses
    --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from MISexOffenders_smts smts
             where smts.id = MISexOffenders_main.id
               and smts.state = MISexOffenders_main.state)) as scarsTattoos
    --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MISexOffenders_photos azp
             where azp.id = MISexOffenders_main.id
               and azp.state = MISexOffenders_main.state)) as photos

from MISexOffenders_main
UNION

---------------- MO
select id
     ,name
     ,DateOfBirth
     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MOSexOffenders_aliases als
             WHERE als.id = MOSexOffenders_main.id
               AND MOSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Street as TEXT) || ' ' || cast(CityState as TEXT)
                       || ' ' || cast(Zip as TEXT)
                       ))

       FROM
           (SELECT Street,
                   CityState,
                   Zip
            FROM MOSexOffenders_addresses arad where arad.ID = MOSexOffenders_main.ID
                                                 and arad.state = MOSexOffenders_main.state)

) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT OffenseDescription as offense
            FROM MOSexOffenders_offenses aro
            where MOSexOffenders_main.ID = aro.ID
              and MOSexOffenders_main.state = aro.state
           )
) as offenses
    --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from MOSexOffenders_smts smts
             where smts.id = MOSexOffenders_main.id
               and smts.state = MOSexOffenders_main.state)) as scarsTattoos
    --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MOSexOffenders_photos azp
             where azp.id = MOSexOffenders_main.id
               and azp.state = MOSexOffenders_main.state)) as photos

from MOSexOffenders_main
UNION
---------- MS TODO: Missing query
----------------NE
select id
     ,name
     ,DOB as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NE_SexOffenders_aliases als
             WHERE als.id = NE_SexOffenders_main.id
               AND NE_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(CityStateZip as TEXT)
                       ,'type',cast(AddressType as TEXT)
                       ))

       FROM
           (SELECT Address,
                   CityStateZip,
                   AddressType
            FROM NE_SexOffenders_addresses arad where arad.ID = NE_SexOffenders_main.ID
                                                  and arad.state = NE_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT Crime as offense
            FROM NE_SexOffenders_convictions aro
            where NE_SexOffenders_main.ID = aro.ID
              and NE_SexOffenders_main.state = aro.state
           )
) as offenses

    --scarsTattoos
    ,json_array("None Reported") as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NE_SexOffenders_photos azp
             where azp.id = NE_SexOffenders_main.id
               and azp.state = NE_SexOffenders_main.state)) as photos

from NE_SexOffenders_main
UNION

----------------NH
select id
     ,name
     ,DOB as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NHSexOffenders_alias als
             WHERE als.id = NHSexOffenders_main.id
               AND NHSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(CityStateZip as TEXT)
                       ,'type',cast(AddressType as TEXT)
                       ))

       FROM
           (SELECT Address,
                   CityStateZip,
                   AddressType
            FROM NHSexOffenders_addresses arad where arad.ID = NHSexOffenders_main.ID
                                                 and arad.state = NHSexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT Offense as offense
            FROM NHSexOffenders_offenses aro
            where NHSexOffenders_main.ID = aro.ID
              and NHSexOffenders_main.state = aro.state
           )
) as offenses

    --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select Description as smt from NHSexOffenders_smts smts
             where smts.id = NHSexOffenders_main.id
               and smts.state = NHSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NHSexOffenders_photos azp
             where azp.id = NHSexOffenders_main.id
               and azp.state = NHSexOffenders_main.state)) as photos

from NHSexOffenders_main
UNION

----------------NJ
select id
     ,name
     ,cast(age as TEXT) as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NJSexOffenders_aliases als
             WHERE als.id = NJSexOffenders_main.id
               AND NJSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT))
                  )

       FROM (SELECT Address
             FROM NJSexOffenders_addresses arad
             where arad.ID = NJSexOffenders_main.ID
               and arad.state = NJSexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || '. ' || cast(description as TEXT) ))
       FROM

           (SELECT offense_description as offense,
                   offense_details as description
            FROM NJSexOffenders_convictions aro
            where NJSexOffenders_main.ID = aro.ID
              and NJSexOffenders_main.state = aro.state
           )
) as offenses


     ,(select json_group_array( cast(smt as text))

       from (select scars_marks_tattoos as smt from NJSexOffenders_smts smts
             where smts.id = NJSexOffenders_main.id
               and smts.state = NJSexOffenders_main.state)) as scarsTattoos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NJSexOffenders_photos azp
             where azp.id = NJSexOffenders_main.id
               and azp.state = NJSexOffenders_main.state)) as photos

from NJSexOffenders_main
Union

---------------- NM
select id
     ,name
     ,YearOfBirth as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NMSexOffenders_aliases als
             WHERE als.id = NMSexOffenders_main.id
               AND NMSexOffenders_main.state = als.state)
    ) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(street as TEXT) || ' ' || cast(citystatezip as TEXT))
                  )

       FROM (SELECT street, citystatezip
             FROM NMSexOffenders_addresses arad
             where arad.ID = NMSexOffenders_main.ID
               and arad.state = NMSexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || '. ' || cast(description as TEXT) ))
       FROM

           (SELECT OffenseDescription as offense,
                   OffenseDetails as description
            FROM NMSexOffenders_offenses aro
            where NMSexOffenders_main.ID = aro.ID
              and NMSexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))

       from (select ScarsMarksTattoos as smt from NMSexOffenders_smts smts
             where smts.id = NMSexOffenders_main.id
               and smts.state = NMSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NMSexOffenders_photos azp
             where azp.id = NMSexOffenders_main.id
               and azp.state = NMSexOffenders_main.state)) as photos

from NMSexOffenders_main
Union

----------------KY
select id
     ,name
     ,birth_date as DateOfBirth

     ,eye_color as eyes
     ,hair_color as hair
     ,height
     ,weight
     ,race
     ,gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     , (SELECT json_group_array(cast(alias as TEXT))

        FROM (SELECT alias_name as alias
              FROM ky_Sex_Offenders_aliases als
              WHERE als.id = ky_Sex_Offenders_main.id
                AND ky_Sex_Offenders_main.state = als.state
             )

    ) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT))
                  )

       FROM (SELECT Address
             FROM ky_Sex_Offenders_addresses arad
             where arad.ID = ky_Sex_Offenders_main.ID
               and arad.state = ky_Sex_Offenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT victim_info as offense
            FROM ky_sex_offenders_victim_info aro
            where ky_Sex_Offenders_main.ID = aro.ID
              and ky_Sex_Offenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,json_array("NoneReported") as scarsTattoos
     /*,(select json_group_array( cast(smt as text))

       from (select ScarsMarksTattoos as smt from CTSexOffenders_smts smts
             where smts.id = CTSexOffenders_main.id
               and smts.state = CTSexOffenders_main.state)) as scarsTattoos

      */
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from ky_Sex_Offenders_photos azp
             where azp.id = ky_Sex_Offenders_main.id
               and azp.state = ky_Sex_Offenders_main.state)) as photos

from ky_Sex_Offenders_main
UNION
---------------- NV
select id
     ,name
     ,YearOfBirth as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NV_SexOffenders_aliases als
             WHERE als.id = NV_SexOffenders_main.id
               AND NV_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT) || ' ' || cast(citystatezip as TEXT))
                  )

       FROM (SELECT Address, citystatezip
             FROM NV_SexOffenders_addresses arad
             where arad.ID = NV_SexOffenders_main.ID
               and arad.state = NV_SexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT ConvictionDescription as offense
            FROM NV_SexOffenders_offenses aro
            where NV_SexOffenders_main.ID = aro.ID
              and NV_SexOffenders_main.state = aro.state
           )
) as offenses

    --scarsTattoos
     ,(select json_group_array( cast(ScarTattoo as text) || ' ' || cast(Location as TEXT) || ' ' || cast(Description as TEXT))

       from (select ScarTattoo, Location, Description from NV_SexOffenders_smts smts
             where smts.id = NV_SexOffenders_main.id
               and smts.state = NV_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NV_SexOffenders_photos azp
             where azp.id = NV_SexOffenders_main.id
               and azp.state = NV_SexOffenders_main.state)) as photos

from NV_SexOffenders_main
UNION
----------------NY
select id
     ,name
     ,DOB as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NYSexOffenders_aliases als
             WHERE als.id = NYSexOffenders_main.id
               AND NYSexOffenders_main.state = als.state)
) as aliases
     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Street as TEXT) || ' ' || cast(CityStateZip as TEXT))
                  )

       FROM (SELECT Street, CityStateZip
             FROM NYSexOffenders_addresses arad
             where arad.ID = NYSexOffenders_main.ID
               and arad.state = NYSexOffenders_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || '. ' || cast(description as TEXT) ))
       FROM

           (SELECT OffenseDescriptions as offense,
                   victimsexage as description
            FROM NYSexOffenders_current_conviction aro
            where NYSexOffenders_main.ID = aro.ID
              and NYSexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))

       from (select ScarMarkTattoo as smt from NYSexOffenders_smts smts
             where smts.id = NYSexOffenders_main.id
               and smts.state = NYSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NYSexOffenders_photos azp
             where azp.id = NYSexOffenders_main.id
               and azp.state = NYSexOffenders_main.state)) as photos

from NYSexOffenders_main
Union

----------------OH
select id
     ,name
     ,DateOfBirth
     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM OHSexOffenders_aliases als
             WHERE als.id = OHSexOffenders_main.id
               AND OHSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT) || ' ' || cast(citystatezip as TEXT),
                                  'type', cast(AddressType as TEXT))
                  )

       FROM (SELECT Address, citystatezip, AddressType
             FROM OHSexOffenders_addresses arad
             where arad.ID = OHSexOffenders_main.ID
               and arad.state = OHSexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text) || ' ' || cast(OffenseDetails as TEXT) ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM OHSexOffenders_convictions aro
            where OHSexOffenders_main.ID = aro.ID
              and OHSexOffenders_main.state = aro.state
           )
) as offenses


     --scarsTattoos
     ,(select json_group_array( cast(ScarsMarksTattoos as text))

       from (select ScarsMarksTattoos from OHSexOffenders_smts smts
             where smts.id = OHSexOffenders_main.id
               and smts.state = OHSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from OHSexOffenders_photos azp
             where azp.id = OHSexOffenders_main.id
               and azp.state = OHSexOffenders_main.state)) as photos

from OHSexOffenders_main
UNION


---------------- PA
select id
     ,name
     ,BirthYear as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT aliases as alias
             FROM PA_SexOffenders_aliases als
             WHERE als.id = PA_SexOffenders_main.id
               AND PA_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          ))

       FROM (SELECT Address
             FROM PA_SexOffenders_addresses arad
             where arad.ID = PA_SexOffenders_main.ID
               and arad.state = PA_SexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(Offense as Text)))
       FROM

           (SELECT Offense
            FROM PA_SexOffenders_offenses aro
            where PA_SexOffenders_main.ID = aro.ID
              and PA_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(Type as text) || ' ' || cast(Location as TEXT) || ' ' || cast(Description as TEXT))

       from (select Type, Location, Description from PA_SexOffenders_smts smts
             where smts.id = PA_SexOffenders_main.id
               and smts.state = PA_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from PA_SexOffenders_photos azp
             where azp.id = PA_SexOffenders_main.id
               and azp.state = PA_SexOffenders_main.state)) as photos

from PA_SexOffenders_main
UNION

---------------- SC
select id
     ,FirstName || ' ' || MiddleName || ' ' || LastName as name
     ,DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM SCSexOffenders_aliases als
             WHERE als.id = SCSexOffenders_main.id
               AND SCSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(AddressLine1 as TEXT) || ' ' || cast(AddressLine2 as TEXT)
                          ))

       FROM (SELECT AddressLine1, AddressLine2
             FROM SCSexOffenders_addresses arad
             where arad.ID = SCSexOffenders_main.ID
               and arad.state = SCSexOffenders_main.state)
)as addresses
     --offenses
     ,json_array("Update Pending") as offenses
    -- TODO: Missing Offenses table
     /*
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(Offense as Text)))
       FROM

           (SELECT Offense
            FROM SCSexOffenders_offenses aro
            where SCSexOffenders_main.ID = aro.ID
              and SCSexOffenders_main.state = aro.state
           )
) as offenses

      */

     --scarsTattoos
     ,(select json_group_array( cast(Type as text) || ' ' || cast(Location as TEXT) || ' ' || cast(Description as TEXT))

       from (select Type, Location, Description from SCSexOffenders_smts smts
             where smts.id = SCSexOffenders_main.id
               and smts.state = SCSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from SCSexOffenders_photos azp
             where azp.id = SCSexOffenders_main.id
               and azp.state = SCSexOffenders_main.state)) as photos

from SCSexOffenders_main
UNION

----------------UT
select id
     ,name
     ,DOB as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM UT_SexOffenders_aliases als
             WHERE als.id = UT_SexOffenders_main.id
               AND UT_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          ))

       FROM (SELECT Address
             FROM UT_SexOffenders_addresses arad
             where arad.ID = UT_SexOffenders_main.ID
               and arad.state = UT_SexOffenders_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text) || ' ' || cast(OffenseDetails as TEXT)
                       ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM UT_SexOffenders_offenses aro
            where UT_SexOffenders_main.ID = aro.ID
              and UT_SexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(ScarsMarksTattoos as text))

       from (select ScarsMarksTattoos from UT_SexOffenders_smts smts
             where smts.id = UT_SexOffenders_main.id
               and smts.state = UT_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from UT_SexOffenders_photos azp
             where azp.id = UT_SexOffenders_main.id
               and azp.state = UT_SexOffenders_main.state)) as photos

from UT_SexOffenders_main
Union

---------------- VA
select id
     ,name
     ,age as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM VASexOffenders_aliases als
             WHERE als.id = VASexOffenders_main.id
               AND VASexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          ))

       FROM (SELECT Address
             FROM VASexOffenders_addresses arad
             where arad.ID = VASexOffenders_main.ID
               and arad.state = VASexOffenders_main.state)
)as addresses

     --offenses
     ,json_array("Update Pending") as offenses
     /*,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text) || ' ' || cast(OffenseDetails as TEXT)
                       ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM VASexOffenders_offenses aro
            where VASexOffenders_main.ID = aro.ID
              and VASexOffenders_main.state = aro.state
           )


) as offenses

      */
     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
/*     ,(select json_group_array( cast(ScarsMarksTattoos as text))

       from (select ScarsMarksTattoos from VASexOffenders_smts smts
             where smts.id = VASexOffenders_main.id
               and smts.state = VASexOffenders_main.state)) as scarsTattoos

 */
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from VASexOffenders_photos azp
             where azp.id = VASexOffenders_main.id
               and azp.state = VASexOffenders_main.state)) as photos

from VASexOffenders_main

-- order by state desc;
UNION

---------------- WI
select id
     ,name
     ,age as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM WI_SexOffenders_aliases als
             WHERE als.id = WI_SexOffenders_main.id
               AND WI_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(street as TEXT) || ' ' || cast(CityStateZip as TEXT)
                          ))

       FROM (SELECT street, CityStateZip
             FROM WI_SexOffenders_addresses arad
             where arad.ID = WI_SexOffenders_main.ID
               and arad.state = WI_SexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseCode as Text) || ' ' || cast(Offense as TEXT)
                       ))
       FROM

           (SELECT OffenseCode, Offense
            FROM WI_SexOffenders_offenses aro
            where WI_SexOffenders_main.ID = aro.ID
              and WI_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
     /*,(select json_group_array( cast(ScarsMarksTattoos as text))

       from (select ScarsMarksTattoos from WI_SexOffenders_smts smts
             where smts.id = WI_SexOffenders_main.id
               and smts.state = WI_SexOffenders_main.state)) as scarsTattoos

      */
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from WI_SexOffenders_photos azp
             where azp.id = WI_SexOffenders_main.id
               and azp.state = WI_SexOffenders_main.state)) as photos

from WI_SexOffenders_main
Union
---------------- WY
select id
     ,name
     ,DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM WYSexOffenders_aliases als
             WHERE als.id = WYSexOffenders_main.id
               AND WYSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT),'type', cast(AddressType as TEXT)
                          ))

       FROM (SELECT Address, AddressType
             FROM WYSexOffenders_addresses arad
             where arad.ID = WYSexOffenders_main.ID
               and arad.state = WYSexOffenders_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || ' ' || cast(description as TEXT)
                       ))
       FROM

           (SELECT details as offense, description
            FROM WYSexOffenders_offenses aro
            where WYSexOffenders_main.ID = aro.ID
              and WYSexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
    ,(select json_group_array( cast(smt as text))

      from (select Scars_Marks_Tattoos as smt from WYSexOffenders_smts smts
            where smts.id = WYSexOffenders_main.id
              and smts.state = WYSexOffenders_main.state)) as scarsTattoos


     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from WYSexOffenders_photos azp
             where azp.id = WYSexOffenders_main.id
               and azp.state = WYSexOffenders_main.state)) as photos

from WYSexOffenders_main
order by state desc;


