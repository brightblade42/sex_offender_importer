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



-- AL_sex_offenders -----------------------------------------------------------
select 0 as id, name, r_Birth_Date as DateOfBirth,state,

       -- aliases
       json_array(
               json_object(
                       'alias',ifnull(cast(r_Aliases as Text), '')
                   )) as aliases,
       --addresses
       json_array(
               json_object(
                       'address1',ifnull(cast(r_Home_Address_1 as Text), ''),
                       'address2', ifnull(cast(r_Home_City_State_Zip as Text), '')
                   )) as addresses,
       json_array(
               json_object(
                       'offense',ifnull(cast(r_Sex_Crime as Text),'') || '. ' || ifnull(cast(r_Description as Text), '')
                   )) as offenses,

       json_array(
               json_object(
                       'eyes',cast(r_eyes as Text), 'hair', cast(r_Hair as Text),
                       'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                       'race', cast(r_Race as Text), 'scarsTattoos',cast(r_Scars_Marks_Tattoos as Text),
                       'sex',cast(r_Sex as Text)

                   )) as personalDetails,

       json_array(cast(r_Image as Text)) as photos

from al_sex_offenders
Union
--KS -------------------------------------------------------------
select 0 as id, name, r_Birth_Date as DateOfBirth,state,
       -- aliases
       json_array(
               json_object(
                       'alias',ifnull(cast(r_Aliases as Text), '')
                   )) as aliases,
       --addresses
       json_array(
               json_object(
                       'address1',ifnull(cast(r_Address_1 as Text), ''),
                       'address2', ifnull(cast(r_Address_2 as Text), '')
                   )) as addresses,
       --offenses
       json_array(
               json_object(
                       'offense',ifnull(cast(r_Offense_1 as Text),'')
                                     || '' || ifnull(cast(r_Offense_2 as Text), '')
                                     || '' || ifnull(cast(r_Offense_3 as Text), '')
                   )) as offenses,


       json_array(
               json_object(
                       'eyes',cast(r_Eye_Color as Text), 'hair', cast(r_Hair_Color as Text),
                       'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                       'race', cast(r_Race as Text), 'scarsTattoos',cast(r_Scars as Text) || ' ' || cast(r_Tattoos as Text),
                       'sex',cast(r_Gender as Text)

                   )) as personalDetails,

       json_array(cast(r_Image as Text)) as photos


from ks_sex_offenders
Union
-- MT ------------------------------------------------------------------
select 0 as id, name, r_Birth_Date as DateOfBirth,state,

       -- aliases
       json_array(
               json_object(
                       'alias',ifnull(cast(r_Nicknames as Text), '')
                   )) as aliases,
       --addresses
       json_array(
               json_object(
                       'address1',ifnull(cast(r_Full_Address as Text), ''),
                       'address2', ''
                   )) as addresses,
       --offenses
       json_array(
               json_object(
                       'offense',ifnull(cast(r_Sentence_Statute_1 as Text),'')
                                     || '' || ifnull(cast(r_Sentence_Statute_2 as Text), '')
                                     || '' || ifnull(cast(r_Sentence_Statute_3 as Text), '')
                   )) as offenses,
        --personal details
       json_array(
               json_object(
                       'eyes',cast(r_eyes as Text), 'hair', cast(r_Hair as Text),
                       'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                       'race', cast(r_Race as Text), 'scarsTattoos',cast(r_Scars_Marks_Tattoos as Text),
                       'sex',cast(r_Sex as Text)

                   )) as personalDetails,
        --photos
       json_array(cast(r_Image as Text)) as photos

from mt_sex_offenders
Union
-- TN --------------------------------------------------------------------------------
select 0 as id, name, r_Birth_Date as DateOfBirth,state,

       -- aliases
       json_array(
               json_object(
                       'alias',ifnull(cast(r_Aliases as Text), '')
                   )) as aliases,
       --addresses
       json_array(
               json_object(
                       'address1',ifnull(cast(r_Address_1 as Text), ''),
                       'address2', ifnull(cast(r_Address_2 as Text),'')
                   )) as addresses,
       --offenses
       json_array(
               json_object(
                       'offense',ifnull(cast(r_Offenses as Text),'')
                   )) as offenses,
       --personal details
       json_array(
               json_object(
                   'eyes',cast(r_Eye_Color as Text), 'hair', cast(r_Hair_Color as Text),
                   'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                   'race', cast(r_Race as Text),
                   'scarsTattoos',cast(r_Scars_and_Marks as Text) || ' ' || cast(r_Tattoos as text),
                   'sex',cast(r_Sex as Text)

                   )) as personalDetails,
       --photos
       json_array(cast(r_Image as Text)) as photos

from tn_sex_offenders
Union

--------------------------------------------------------------------

-- THE 13 -------------------------------------------------------------


-- AZ --------------------------------------------------------------------
SELECT id,
       name,
       '' as DateOfBirth,
       -- aliases
       state,
       (SELECT json_group_array(cast(alias as Text))
        FROM
            (SELECT alias
             FROM AZ_SexOffenders_aliases als
             WHERE als.id = AZ_SexOffenders_main.id
               AND AZ_SexOffenders_main.state = als.state
            )
       ) as aliases,
       json_array(
               json_object('address', cast(address as Text))) as addresses,

       -- offenses
       (SELECT
            json_group_array(json_object ( 'offense', cast(offense as Text)
                --, 'state', cast(state as Text),
                -- 'conviction_state', cast(conviction_state as Text),
                --'date_convicted', cast(date_convicted as Text),
                --'release_date', cast(release_date as Text),
                --'details', cast(details as Text)
                ))
        FROM (SELECT description as offense
                     --,state,
                     -- conviction_state,
                     -- date_convicted,
                     --  release_date,
                     --  details
              FROM AZ_SexOffenders_offenses azo
              WHERE azo.id = AZ_SexOffenders_main.id
                and AZ_SexOffenders_main.state = azo.state
             )
       ) as offenses

        ,
       -- personal details
       (select json_group_array(
                       json_object( 'age', cast(age as Text),
                                    'eyes', cast(eyes as Text),
                                    'hair', cast(hair as Text),
                                    'height', cast(height as Text),
                                    'weight', cast(weight as Text),
                           --   'level', cast(level as Text),
                                    'race', cast(race as Text),
                                    'scarsTattoos', cast(scars_tattoos as Text),
                                    'sex',cast(sex as Text),
                                    'status', cast(status as Text)
                           ))
        from (select age, eyes, hair, height, level, race, scars_tattoos,
                     sex, status, weight
              from AZ_SexOffenders_main azm
              where azm.id = AZ_SexOffenders_main.id
                and azm.state = AZ_SexOffenders_main.state
             )) as personalDetails,
        --photos
       (select json_group_array(cast(PhotoFile as Text))
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
SELECT id,
       name,
       --'' as age,
       '' as DateOfBirth,
       state,
       -- aliases
       (SELECT json_group_array(cast(alias as Text))
        FROM
            (SELECT alias
             FROM CA_SexOffenders_alias als
             WHERE als.id = CA_SexOffenders_main.id
               AND CA_SexOffenders_main.state = als.state
            )
       ) as aliases,
       --addresses
       json_array(
               json_object('address', cast(address as Text))) as addresses,

       -- offenses
       (SELECT
            json_group_array(json_object ( 'offense', cast(offense as Text)
                ))
        FROM (SELECT OffenseDescription as offense
              FROM CA_SexOffenders_offenses azo
              WHERE azo.id = CA_SexOffenders_main.id
                and CA_SexOffenders_main.state = azo.state
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
                           --'scarsTattoos', cast(scars_tattoos as Text),
                               'sex',cast(sex as Text)
                           ))
        from (select EyeColor as  eyes,
                     HairColor hair,
                     Height,
                     Ethnicity as race,
                     -- MISSING SCARS
                     sex,
                     weight
              from CA_SexOffenders_main azm
              where azm.id = CA_SexOffenders_main.id
                and azm.state = CA_SexOffenders_main.state
             )) as personalDetails,

       --photos
       (select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from CA_SexOffenders_photos azp
              where azp.id = CA_SexOffenders_main.id
                and azp.state = CA_SexOffenders_main.state)) as photos


from CA_SexOffenders_main;
/*
CREATE TABLE GA_SexOffenders_main ( Absconder,  EyeColor,  FirstName,  HairColor,  Height,  ID,  LastKnownAddress,  LastName,
  Leveling,  MiddleName,  Predator,  Race,  RegistrationDate,  ResidenceVerificationDate,  Sex,  Suffix,  Weight,  YearOfBirth, state )
*/

UNION


SELECT id,
       ifnull(LastName,'') || ', ' || ifnull(FirstName,'') || ' ' || ifnull(MiddleName,'') as name ,
       --'' as age,
       YearOfBirth as DateOfBirth,
       state,
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
        json_array("Update pending") as aliases,
       --addresses
       json_array(
               json_object('address', cast(LastKnownAddress as Text))) as addresses,

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
        json_array("Update pending") as offenses,
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
              from GA_SexOffenders_main azm
              where azm.id = GA_SexOffenders_main.id
                and azm.state = GA_SexOffenders_main.state
             )) as personalDetails,

       --photos
       (select json_group_array(cast(PhotoFile as Text))
        from (select PhotoFile from GA_SexOffenders_photos azp
              where azp.id = GA_SexOffenders_main.id
                and azp.state = GA_SexOffenders_main.state)) as photos

From GA_SexOffenders_main

order by state;







