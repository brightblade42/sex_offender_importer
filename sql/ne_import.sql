Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,DOB as DateOfBirth

     ,eyes
     ,hair
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
             FROM NE_SexOffenders_aliases als
             WHERE als.id = NE_SexOffenders_main.id
               AND NE_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(CityStateZip as TEXT)
                       ,'type',cast(AddressType as TEXT)
                       ))

       FROM
           (SELECT Address,
                   CityStateZip,
                   AddressType
            FROM NE_SexOffenders_addresses arad where arad.ID = NE_SexOffenders_main.ID
                                                  and arad.state = NE_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT Crime as offense
            FROM NE_SexOffenders_convictions aro
            where NE_SexOffenders_main.ID = aro.ID
              and NE_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NE_SexOffenders_photos azp
             where azp.id = NE_SexOffenders_main.id
               and azp.state = NE_SexOffenders_main.state)) as photos

from NE_SexOffenders_main
