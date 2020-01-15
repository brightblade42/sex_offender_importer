Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(full_name as TEXT)   as name
     ,cast(year_of_birth as TEXT)   as DateOfBirth
     ,cast(eye_color as TEXT)   as Eyes
     ,cast(hair_color as TEXT)   as Hair
     ,cast(Height as TEXT) as Height
     ,cast(Weight as TEXT) as Weight
     ,cast(race as TEXT) as race
     ,cast(gender as TEXT)   as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias

             FROM HI_sex_offender_aliases als
             WHERE als.id = HI_sex_offender_main.id
               AND HI_sex_offender_main .state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          , 'type', cast(Type as TEXT)
                          ))

       FROM (SELECT Address, address_type as Type
             FROM HI_sex_offender_addresses arad
             where arad.ID = HI_sex_offender_main.ID
               and arad.state = HI_sex_offender_main.state)
)as addresses

--     ,json_array("Unknown") as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text)
                       ))
       FROM

           (SELECT offense
            FROM HI_sex_offender_offenses aro
            where HI_sex_offender_main.ID = aro.ID
              and HI_sex_offender_main.state = aro.state
           )
) as offenses
     --scars tattoos
     ,(select json_group_array( cast(smt as text))
       from (select Scars_Marks_Tattoos as smt from HI_Sex_Offender_smts smts
             where smts.id = HI_Sex_Offender_main.id
               and smts.state = HI_Sex_Offender_main.state)) as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from HI_sex_offender_photos azp
             where azp.id = HI_sex_offender_main.id
               and azp.state = HI_sex_offender_main.state)) as photos

from HI_sex_offender_main
