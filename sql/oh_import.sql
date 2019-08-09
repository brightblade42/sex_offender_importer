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
             FROM OHSexOffenders_aliases als
             WHERE als.id = OHSexOffenders_main.id
               AND OHSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT) || ' ' || cast(citystatezip as TEXT),
                                  'type', cast(AddressType as TEXT))
                  )

       FROM (SELECT Address, citystatezip, AddressType
             FROM OHSexOffenders_addresses arad
             where arad.ID = OHSexOffenders_main.ID
               and arad.state = OHSexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text) || ' ' || cast(OffenseDetails as TEXT) ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM OHSexOffenders_convictions aro
            where OHSexOffenders_main.ID = aro.ID
              and OHSexOffenders_main.state = aro.state
           )
) as offenses


     --scarsTattoos
     ,(select json_group_array( cast(ScarsMarksTattoos as text))

       from (select ScarsMarksTattoos from OHSexOffenders_smts smts
             where smts.id = OHSexOffenders_main.id
               and smts.state = OHSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from OHSexOffenders_photos azp
             where azp.id = OHSexOffenders_main.id
               and azp.state = OHSexOffenders_main.state)) as photos

from OHSexOffenders_main
