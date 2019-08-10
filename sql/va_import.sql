Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(age as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
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
             FROM VASexOffenders_aliases als
             WHERE als.id = VASexOffenders_main.id
               AND VASexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          ))

       FROM (SELECT Address
             FROM VASexOffenders_addresses arad
             where arad.ID = VASexOffenders_main.ID
               and arad.state = VASexOffenders_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT Statute as offense
            FROM VASexOffenders_offenses aro
            where VASexOffenders_main.ID = aro.ID
              and VASexOffenders_main.state = aro.state
           )


) as offenses

     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from VASexOffenders_photos azp
             where azp.id = VASexOffenders_main.id
               and azp.state = VASexOffenders_main.state)) as photos

from VASexOffenders_main
