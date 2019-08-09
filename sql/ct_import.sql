Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
------------- CT
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
             FROM CTSexOffenders_aliases als
             WHERE als.id = CTSexOffenders_main.id
               AND CTSexOffenders_main.state = als.state)
) as aliases

     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(address as TEXT)
                       || ' ' || cast(AddressExtended as TEXT)
                       || ' ' || cast(CityStateZip as TEXT)
                       ))
       FROM
           (SELECT address,
                   AddressExtended,
                   CityStateZip
            FROM CTSexOffenders_addresses arad
            where arad.ID = CTSexOffenders_main.ID
              and arad.state = CTSexOffenders_main.state

           )) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', offense)
               )
       FROM

           (SELECT
                    cast(OffenseDescription as TEXT) || ' ' || cast(OffenseDetails as TEXT) as offense
            FROM CTSexOffenders_offenses aro
            where CTSexOffenders_main.ID = aro.ID
              and CTSexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from CTSexOffenders_smts smts
             where smts.id = CTSexOffenders_main.id
               and smts.state = CTSexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from CTSexOffenders_photos azp
             where azp.id = CTSexOffenders_main.id
               and azp.state = CTSexOffenders_main.state)) as photos


from CTSexOffenders_main
