Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,DateOfBirth

     ,'' as eyes
     ,'' as hair
     ,'' as height
     ,'' as weight
     ,'' as race
     ,'' as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM ME_SexOffenders_aliases als
             WHERE als.id = ME_SexOffenders_main.id
               AND ME_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT)
                       ))

       FROM
           (SELECT TownOfDomicile as address
            FROM ME_SexOffenders_addresses arad where arad.ID = ME_SexOffenders_main.ID
                                                  and arad.state = ME_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text), 'descr',cast(Description as Text) ))
       FROM

           (SELECT Statute as offense,
                   Offense as Description
            FROM ME_SexOffenders_offenses aro
            where ME_SexOffenders_main.ID = aro.ID
              and ME_SexOffenders_main.state = aro.state
           )
) as offenses

     --personalDetails
     ,json_array("None Reported") as scarsTattoos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from ME_SexOffenders_photos azp
             where azp.id = ME_SexOffenders_main.id
               and azp.state = ME_SexOffenders_main.state)) as photos

from ME_SexOffenders_main
