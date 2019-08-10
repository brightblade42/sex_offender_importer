Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DOB as TEXT) as DateOfBirth
     ,cast(Eyes as TEXT) as Eyes
     ,cast(Hair as TEXT) as Hair
     ,cast(Height as TEXT) as Height
     ,cast(Weight as TEXT) as Weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     ,( SELECT json_group_array (cast(Aliases as TEXT))
        FROM
            (SELECT Aliases

             FROM AK_sex_offender_aliases als
             WHERE als.id = AK_sex_offender_main.id
               AND AK_sex_offender_main .state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          || ' ' ||  cast(City as TEXT)
                          || ' ' || cast(Addr_State AS TEXT),
                                  'type', cast(Type as TEXT)
                          ))

       FROM (SELECT Address,City, Addr_State, Type
             FROM AK_sex_offender_addresses arad
             where arad.ID = AK_sex_offender_main.ID
               and arad.state = AK_sex_offender_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(Description as Text)
                       ))
       FROM

           (SELECT description
            FROM AK_sex_offender_offenses aro
            where AK_sex_offender_main.ID = aro.ID
              and AK_sex_offender_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,json_array("Unknown") as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from AK_sex_offender_photos azp
             where azp.id = AK_sex_offender_main.id
               and azp.state = AK_sex_offender_main.state)) as photos

from AK_sex_offender_main
