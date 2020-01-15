Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select ifnull(cast(case_num as TEXT),'0') as id
     ,cast(name as TEXT) as name
     ,cast(r_Birth_Date as TEXT) as DateOfBirth
     ,cast(r_Eyes as TEXT) as eyes
     ,cast(r_Hair as TEXT) as hair
     ,cast(r_Height as TEXT) as height
     ,cast(r_Weight as TEXT) as weight
     ,cast(r_race as TEXT) as race
     ,cast(r_Sex as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     -- aliases
     ,json_array( cast(r_Nicknames as Text)) as aliases
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
