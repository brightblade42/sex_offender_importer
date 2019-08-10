Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
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
