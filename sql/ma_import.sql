Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(Year_Of_Birth as TEXT)   as DateOfBirth
     ,cast(Eye_Color as TEXT)   as eyes
     ,cast(Hair_Color as TEXT)   as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MA_SexOffenders_aliases als
             WHERE als.id = MA_SexOffenders_main.id
               AND MA_SexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT), 'type', cast(Type as Text)
                       ))

       FROM
           (SELECT Address,
                   Type
            FROM MA_SexOffenders_addresses arad where arad.ID = MA_SexOffenders_main.ID
                                                  and arad.state = MA_SexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT Jurisdiction as offense
            FROM MA_SexOffenders_offenses aro
            where MA_SexOffenders_main.ID = aro.ID
              and MA_SexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
    /*,(select json_group_array( cast(smt as text))
      from (select ScarsMarksTattoos as smt from MA_SexOffenders_smts smts
            where smts.id = MA_SexOffenders_main.id
              and smts.state = MA_SexOffenders_main.state)) as scarsTattoos
*/
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MA_SexOffenders_photos azp
             where azp.id = MA_SexOffenders_main.id
               and azp.state = MA_SexOffenders_main.state)) as photos

from MA_SexOffenders_main
