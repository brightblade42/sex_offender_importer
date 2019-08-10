Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select ifnull(cast(r_Image as TEXT),'0') as id
     , cast(name as TEXT) as name
     ,cast(r_Birth_Date as TEXT)   as DateOfBirth
     ,cast(r_Eye_Color as TEXT)   as eyes
     ,cast(r_Hair_Color as TEXT)   as hair
     ,cast(r_Height as TEXT)   as height
     ,cast(r_Weight as TEXT)   as weight
     ,cast(r_race as TEXT)   as race
     ,cast(r_Gender as TEXT)   as sex
     ,trim(cast(state as TEXT)) as state
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
