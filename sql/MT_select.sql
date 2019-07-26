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
