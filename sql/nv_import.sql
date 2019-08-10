Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(YearOfBirth as TEXT) as DateOfBirth
     ,cast(EyeColor as TEXT) as eyes
     ,cast(HairColor as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NV_SexOffenders_aliases als
             WHERE als.id = NV_SexOffenders_main.id
               AND NV_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT) || ' ' || cast(citystatezip as TEXT))
                  )

       FROM (SELECT Address, citystatezip
             FROM NV_SexOffenders_addresses arad
             where arad.ID = NV_SexOffenders_main.ID
               and arad.state = NV_SexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT ConvictionDescription as offense
            FROM NV_SexOffenders_offenses aro
            where NV_SexOffenders_main.ID = aro.ID
              and NV_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(ScarTattoo as text) || ' ' || cast(Location as TEXT) || ' ' || cast(Description as TEXT))

       from (select ScarTattoo, Location, Description from NV_SexOffenders_smts smts
             where smts.id = NV_SexOffenders_main.id
               and smts.state = NV_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NV_SexOffenders_photos azp
             where azp.id = NV_SexOffenders_main.id
               and azp.state = NV_SexOffenders_main.state)) as photos

from NV_SexOffenders_main
