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

from mt_sex_offenders order by state;
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

from tn_sex_offenders order by state;
select * from tn_sex_offenders
