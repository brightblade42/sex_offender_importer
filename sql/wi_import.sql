Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,age as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM WI_SexOffenders_aliases als
             WHERE als.id = WI_SexOffenders_main.id
               AND WI_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(street as TEXT) || ' ' || cast(CityStateZip as TEXT)
                          ))

       FROM (SELECT street, CityStateZip
             FROM WI_SexOffenders_addresses arad
             where arad.ID = WI_SexOffenders_main.ID
               and arad.state = WI_SexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseCode as Text) || ' ' || cast(Offense as TEXT)
                       ))
       FROM

           (SELECT OffenseCode, Offense
            FROM WI_SexOffenders_offenses aro
            where WI_SexOffenders_main.ID = aro.ID
              and WI_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from WI_SexOffenders_photos azp
             where azp.id = WI_SexOffenders_main.id
               and azp.state = WI_SexOffenders_main.state)) as photos

from WI_SexOffenders_main
