Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select ifnull(case_num, '0') as id
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
