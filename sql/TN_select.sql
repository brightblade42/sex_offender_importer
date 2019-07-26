select 0 as id
     , name
     , r_Birth_Date as DateOfBirth
     ,state

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
     --personal details
     ,json_array(
        json_object(
                'eyes',cast(r_Eye_Color as Text), 'hair', cast(r_Hair_Color as Text),
                'height', cast(r_Height as Text), 'weight', cast(r_Weight as Text),
                'race', cast(r_Race as Text),
                'sex',cast(r_Sex as Text)

            )) as personalDetails
     ,json_array(cast(r_Scars_and_Marks as Text) || ' ' || cast(r_Tattoos as text))
     --photos
     ,json_array(cast(r_Image as Text)) as photos

from tn_sex_offenders
