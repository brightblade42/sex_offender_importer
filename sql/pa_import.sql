Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(BirthYear as TEXT) as DateOfBirth
     ,cast(EyeColor as TEXT) as eyes
     ,cast(HairColor as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(Gender as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT aliases as alias
             FROM PA_SexOffenders_aliases als
             WHERE als.id = PA_SexOffenders_main.id
               AND PA_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          ))

       FROM (SELECT Address
             FROM PA_SexOffenders_addresses arad
             where arad.ID = PA_SexOffenders_main.ID
               and arad.state = PA_SexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(Offense as Text)))
       FROM

           (SELECT Offense
            FROM PA_SexOffenders_offenses aro
            where PA_SexOffenders_main.ID = aro.ID
              and PA_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(Type as text) || ' ' || cast(Location as TEXT) || ' ' || cast(Description as TEXT))

       from (select Type, Location, Description from PA_SexOffenders_smts smts
             where smts.id = PA_SexOffenders_main.id
               and smts.state = PA_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from PA_SexOffenders_photos azp
             where azp.id = PA_SexOffenders_main.id
               and azp.state = PA_SexOffenders_main.state)) as photos

from PA_SexOffenders_main
