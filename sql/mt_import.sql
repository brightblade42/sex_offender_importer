Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select ifnull(case_num,'0') as id
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
