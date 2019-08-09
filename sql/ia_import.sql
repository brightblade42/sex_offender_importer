Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,ifnull(Last_Name,'') || ', ' || ifnull(First_Name,'') || ' ' || ifnull(Middle_Name,'') as name
     ,birthdate as DateOfBirth
     ,eye_color as Eyes
     ,hair_color as Hair
     ,height_inches as Height
     ,weight_pounds as Weight
     ,race
     ,gender as sex
     ,trim(state) as state
     -- aliases
     ,( SELECT json_group_array (cast(last_name as TEXT) || ',' || cast(first_name as TEXT ) || ' ' || cast(middle_name as TEXT))
        FROM
            (SELECT als.last_name, als.first_name, als.middle_name

             FROM IA_sex_offender_aliases als
             WHERE als.id = IA_sex_offender_main.id
               AND IA_sex_offender_main .state = als.state)
) as aliases

     --addresses
     ,json_array(json_object('address', cast(address as TEXT))) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text)
                       ))
       FROM

           (SELECT conviction as offense
            FROM IA_sex_offender_convictions aro
            where IA_sex_offender_main.ID = aro.ID
              and IA_sex_offender_main.state = aro.state
           )
) as offenses
     --scars tattoos
     ,(select json_group_array( cast(smt as text))
       from (select skin_marking as smt from IA_Sex_Offender_smts smts
             where smts.id = IA_Sex_Offender_main.id
               and smts.state = IA_Sex_Offender_main.state)) as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from IA_sex_offender_photos azp
             where azp.id = IA_sex_offender_main.id
               and azp.state = IA_sex_offender_main.state)) as photos

from IA_sex_offender_main
