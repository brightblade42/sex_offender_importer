Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
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

from al_sex_offenders;
