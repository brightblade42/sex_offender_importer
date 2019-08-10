Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DOB as TEXT) as DateOfBirth

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
             FROM NHSexOffenders_alias als
             WHERE als.id = NHSexOffenders_main.id
               AND NHSexOffenders_main.state = als.state)
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
            FROM NHSexOffenders_addresses arad where arad.ID = NHSexOffenders_main.ID
                                                 and arad.state = NHSexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT Offense as offense
            FROM NHSexOffenders_offenses aro
            where NHSexOffenders_main.ID = aro.ID
              and NHSexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select Description as smt from NHSexOffenders_smts smts
             where smts.id = NHSexOffenders_main.id
               and smts.state = NHSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NHSexOffenders_photos azp
             where azp.id = NHSexOffenders_main.id
               and azp.state = NHSexOffenders_main.state)) as photos

from NHSexOffenders_main
