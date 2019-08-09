Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,YearOfBirth as DateOfBirth

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
             FROM INSexOffenders_aliases als
             WHERE als.id = INSexOffenders_main.id
               AND INSexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(AddressExtension as TEXT), 'type', cast(AddressType as Text)
                       ))

       FROM
           (SELECT Address,
                   AddressExtension,
                   AddressType
            FROM INSexOffenders_addresses arad where arad.ID = INSexOffenders_main.ID
                                                 and arad.state = INSexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text)
                       || ' ' || cast(OffenseDetails as Text)
                       ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM INSexOffenders_offenses aro
            where INSexOffenders_main.ID = aro.ID
              and INSexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from INSexOffenders_smts smts
             where smts.id = INSexOffenders_main.id
               and smts.state = INSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from INSexOffenders_photos azp
             where azp.id = INSexOffenders_main.id
               and azp.state = INSexOffenders_main.state)) as photos


from INSexOffenders_main
