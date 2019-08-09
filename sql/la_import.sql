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
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM LA_SexOffenders_aliases als
             WHERE als.id = LA_SexOffenders_main.id
               AND LA_SexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT), 'type', cast(AddressType as Text)
                       ))

       FROM
           (SELECT Address,
                   AddressType
            FROM LA_SexOffenders_addresses arad where arad.ID = LA_SexOffenders_main.ID
                                                  and arad.state = LA_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text)
                       || ' ' || cast(OffenseDetails as Text)
                       ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM LA_SexOffenders_offenses aro
            where LA_SexOffenders_main.ID = aro.ID
              and LA_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from LA_SexOffenders_smts smts
             where smts.id = LA_SexOffenders_main.id
               and smts.state = LA_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from LA_SexOffenders_photos azp
             where azp.id = LA_SexOffenders_main.id
               and azp.state = LA_SexOffenders_main.state)) as photos

from LA_SexOffenders_main
