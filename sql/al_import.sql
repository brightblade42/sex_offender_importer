Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(r_Image as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(r_Birth_Date as TEXT) as dateOfBirth
     ,cast(r_Eyes as TEXT)   as eyes
     ,cast(r_Hair as TEXT)   as hair
     ,cast(r_Height as TEXT)   as height
     ,cast(r_Weight as TEXT)   as weight
     ,cast(r_race as TEXT)   as race
     ,cast(r_Sex as TEXT)   as sex
     ,trim(cast(state as TEXT)) as state

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

from al_sex_offenders;
