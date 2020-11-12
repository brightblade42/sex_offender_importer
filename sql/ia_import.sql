Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     --,ifnull(cast(First_Name as TEXT),'') || ', ' || ifnull(cast(middle_name as TEXT),'') || ' '
     --     || ifnull(cast(last_name as TEXT),'') as name
     ,cast(Name as TEXT) as Name
     ,cast(Birthdate as TEXT)   as DateOfBirth
     ,cast(Eyes as TEXT)   as Eyes
     ,cast(Hair as TEXT)   as Hair
     ,cast(Height as TEXT)   as Height
     ,cast(Weight as TEXT)   as Weight
     ,cast(Race as TEXT) as race
     ,cast(Gender as TEXT)   as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     ,( SELECT json_group_array (cast(last_name as TEXT) || ',' || cast(first_name as TEXT ) || ' ' || cast(middle_name as TEXT))
        FROM
            (SELECT als.last_name, als.first_name, als.middle_name

             FROM IA_sex_offender_aliases als
             WHERE als.id = IA_sex_offender_main.id
               AND IA_sex_offender_main .state = als.state)
) as aliases

     --addresses
     ,'[]' as addresses
     --,json_array(json_object('address', cast(address as TEXT))) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text)
                       ))
       FROM

           (SELECT conviction as offense
            FROM IA_sex_offender_convictions conv
            where IA_sex_offender_main.ID = conv.ID
              and IA_sex_offender_main.state = conv.state
           )
) as offenses
     --scars tattoos
     ,(select json_group_array( cast(smt as text))
       from (select skin_marking as smt from IA_Sex_Offender_smts smts
             where smts.id = IA_Sex_Offender_main.id
               and smts.state = IA_Sex_Offender_main.state)) as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from IA_sex_offender_photos photos
             where photos.id = IA_sex_offender_main.id
               and photos.state = IA_sex_offender_main.state)) as photos

from IA_sex_offender_main
