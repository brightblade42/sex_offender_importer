Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,YearOfBirth as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
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
