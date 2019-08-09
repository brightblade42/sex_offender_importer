Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,DateOfBirth
     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MISexOffenders_aliases als
             WHERE als.id = MISexOffenders_main.id
               AND MISexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(Address_Extended as TEXT)
                       , 'type', cast(AddressType as TEXT)
                       ))

       FROM
           (SELECT Address, Address_Extended, AddressType
            FROM MISexOffenders_addresses arad where arad.ID = MISexOffenders_main.ID
                                                  and arad.state = MISexOffenders_main.state)

) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text), 'descr',cast(Description as Text) ))
       FROM

           (SELECT OffenseDescription  as offense,
                   OffenseDetails as Description
            FROM MISexOffenders_covictions aro
            where MISexOffenders_main.ID = aro.ID
              and MISexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from MISexOffenders_smts smts
             where smts.id = MISexOffenders_main.id
               and smts.state = MISexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MISexOffenders_photos azp
             where azp.id = MISexOffenders_main.id
               and azp.state = MISexOffenders_main.state)) as photos

from MISexOffenders_main
